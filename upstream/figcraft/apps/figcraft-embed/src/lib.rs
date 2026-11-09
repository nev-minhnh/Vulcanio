//! Vulcanio Interface as a Vulcanio module: the desktop app's frame logic and UI unchanged, created inside
//! the Vulcanio window instead of its own.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
#![allow(dead_code)]

#[cfg(target_os = "macos")]
mod apple_events;
mod args;
mod control_server;
mod gamepad;
mod graphics;
mod recovery;
mod services;
mod startup;

#[cfg(all(test, target_os = "macos"))]
mod apple_events_tests;
#[cfg(test)]
mod args_tests;
#[cfg(test)]
mod graphics_tests;
#[cfg(test)]
mod recovery_online_tests;
#[cfg(test)]
mod recovery_tests;
#[cfg(test)]
mod services_tests;
#[cfg(test)]
mod settings_dir_tests;
#[cfg(test)]
mod startup_tests;

use std::sync::{Arc, Mutex};

use figcraft_engine::Session;
use figcraft_ui_egui::windows::with_app;
use figcraft_ui_egui::{ControlRequest, FigApp};
use serde_json::json;

/// The app state lives in [`figcraft_ui_egui::windows`] (other windows draw it too).
struct App {
    queue: Arc<Mutex<Vec<ControlRequest>>>,
    ctx_slot: Arc<Mutex<Option<egui::Context>>>,
    /// The window title last sent: a viewport command asks for another frame, so it's only sent
    /// when the title changes (sending it every frame kept an idle window redrawing).
    title: String,
    /// Shared with the services, which write the copies after a panic.
    recovery: recovery::Shared,
    /// Documents and quit requests from Finder, the Dock and Open With.
    #[cfg(target_os = "macos")]
    apple_events: fmv_macos_events::Inbox,
}

impl eframe::App for App {
    /// Native eframe logic callback: store the first context, drain control requests, run
    /// guarded background work, poll macOS open events, handle close requests and title
    /// changes, and schedule recovery repaints on the app clock.
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        {
            let mut slot = self.ctx_slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if slot.is_none() {
                *slot = Some(ctx.clone());
            }
        }
        let title = with_app(|fig| {
            control_server::drain_queue(&self.queue, &mut fig.control_queue);
            // Phase-safe background work: control queues, autosave/recovery deadlines, live polling.
            // It runs on logic-only callbacks too, so it never reads UI input.
            figcraft_ui_egui::windows::guard(fig, ctx, |fig| fig.background_tick(ctx));
            #[cfg(target_os = "macos")]
            apple_events::poll(&self.apple_events, fig, ctx);
            // Closing the window asks about unsaved changes (and keeps the app running while other
            // windows are open).
            figcraft_ui_egui::windows::on_close_request(fig, ctx);
            fig.title()
        });
        // Window title follows the document.
        if let Some(title) = title
            && title != self.title
        {
            ctx.send_viewport_cmd(egui::ViewportCommand::Title(title.clone()));
            self.title = title;
        }
        // Recovery deadlines use the same monotonic clock as the app's background work: a
        // logic-only tick leaves `ctx.input(i.time)` stale.
        if let Some(wait) = with_app(|fig| {
            let now = fig.now_secs();
            recovery::lock(&self.recovery).as_mut().and_then(|r| r.tick(&fig.session, now))
        })
        .flatten()
        {
            ctx.request_repaint_after(std::time::Duration::from_secs_f64(wait));
        }
    }

    fn raw_input_hook(&mut self, _ctx: &egui::Context, raw_input: &mut egui::RawInput) {
        // Pointer and key events sent over the control channel (`ui.input`), for the window
        // about to draw.
        if let Some(events) = with_app(|fig| figcraft_ui_egui::windows::take_input(fig, raw_input.viewport_id)) {
            raw_input.events.extend(events);
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        services::parented(frame, || with_app(|fig| figcraft_ui_egui::windows::guard(fig, &ctx, |fig| fig.ui(ui))));
    }

    fn on_exit(&mut self) {
        // With the app out of reach (busy), its recovery copies stay for the next start.
        let copies = recovery::lock(&self.recovery).take();
        with_app(|fig| services::shutdown(fig, services::config_dir().as_deref(), copies));
        // Everything is kept: end before the windows' teardown, where AppKit's Touch Bar support
        // throws on the content view AccessKit restores (craft-libs#3, #781).
        #[cfg(target_os = "macos")]
        std::process::exit(0);
    }
}


fn lang_code(lang: vulcanio_host::Lang) -> String {
    lang.code().to_string()
}

/// Create Vulcanio Interface inside the Vulcanio window.
pub fn create(host: &vulcanio_host::HostContext<'_>) -> Box<dyn vulcanio_host::Module> {
    let cc = host.cc;
    let ctx_slot: Arc<Mutex<Option<egui::Context>>> = Arc::new(Mutex::new(Some(cc.egui_ctx.clone())));
    let wake = |slot: &Arc<Mutex<Option<egui::Context>>>| {
        let slot = slot.clone();
        move || {
            if let Some(c) = slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref() {
                c.request_repaint();
            }
        }
    };
    // System fonts are indexed when a font list or a missing font needs them (Vulcanio keeps an
    // app's start light).
    figcraft_text::scan_system_fonts_on_demand();
    let mut session = Session::empty();
    session.live.install(Box::new(figcraft_collab_net::NetLink::new(Arc::new(wake(&ctx_slot)))));
    let mut failed_open = None;
    for f in &host.files {
        let f = f.to_string_lossy().into_owned();
        if let Err(e) = session.execute("file.open", &json!({"path": f})) {
            log::error!("{f}: {e}");
            let name = std::path::Path::new(&f).file_name().map_or_else(|| f.clone(), |n| n.to_string_lossy().into_owned());
            failed_open = Some(format!("{name}: {e}"));
        }
    }
    let opened: Vec<u64> = session.docs.iter().map(|d| d.uid).collect();
    let recovery_root = services::config_dir().map(|d| d.join("Recovery"));
    let recovery = recovery::Shared::default();
    let svc = services::Desktop::load(recovery.clone());
    svc.follow_system_dark(wake(&ctx_slot));
    if let Some(prefs) = svc.prefs.clone() {
        session.prefs = prefs;
    }
    let recovered = recovery_root.as_deref().map(|r| recovery::recover_all(r, &mut session)).unwrap_or_default();
    let mut copies = recovery_root.as_deref().and_then(recovery::Recovery::start);
    if let Some(r) = copies.as_mut() {
        r.adopt(recovered.waiting);
    }
    *recovery::lock(&recovery) = copies;
    session.plugins.install_dir = services::config_dir().map(|d| d.join("Plugins"));
    session.live.links = figcraft_collab_net::links::system();
    session.invite_links_startup();
    let port = std::env::var("FIGCRAFT_CONTROL_PORT").ok().and_then(|p| p.parse().ok());
    let first_run = svc.prefs.is_none();
    // Inside Vulcanio every start shows Home: no new blank file (a control port still gets one).
    if session.docs.is_empty() && port.is_some() && opens_new_file(first_run, session.prefs.start_on_home, true) {
        let _ = session.execute("file.new", &json!({}));
    }
    session.prefs.language = lang_code(host.lang);
    figcraft_ui_egui::i18n::set(&session.prefs.language, figcraft_ui_egui::Services::system_language(&svc).as_deref());
    let mut fig = FigApp::new(session, Box::new(svc));
    // No update check: Vulcanio updates as one app, never Vulcanio Interface's executable on its own.
    // Home first, unless files were asked for (recovered work stays open in tabs behind it).
    fig.ui.home = host.files.is_empty();
    fig.ui.font_checks = opened;
    fig.note_recovered(recovered.copies);
    if let Some(message) = failed_open {
        fig.show_error(message);
    }
    let queue: Arc<Mutex<Vec<ControlRequest>>> = Arc::new(Mutex::new(Vec::new()));
    if let Some(p) = port {
        control_server::start(p, graphics::port_wait(), queue.clone(), ctx_slot.clone(), fig.session.events.clone());
    }
    // Vulcanio draws the window frame (the system's).
    fig.ui.title_bar = figcraft_ui_egui::TitleBar::Native;
    figcraft_ui_egui::windows::install(fig);
    with_app(|fig| fig.services.set_egui_context(&cc.egui_ctx));
    Box::new(App {
        queue,
        ctx_slot,
        title: String::new(),
        recovery,
        #[cfg(target_os = "macos")]
        apple_events: fmv_macos_events::Inbox::default(),
    })
}

impl vulcanio_host::Module for App {
    fn app(&mut self) -> &mut dyn eframe::App {
        self
    }

    fn open_paths(&mut self, ctx: &egui::Context, paths: &[std::path::PathBuf]) {
        with_app(|fig| {
            for p in paths {
                if fig.run("file.open", json!({"path": p.to_string_lossy()})).is_some() {
                    fig.ui.home = false;
                }
            }
        });
        ctx.request_repaint();
    }

    fn new_document(&mut self, ctx: &egui::Context) {
        with_app(|fig| {
            if fig.run("file.new", json!({})).is_some() {
                fig.ui.home = false;
            }
        });
        ctx.request_repaint();
    }

    fn set_language(&mut self, ctx: &egui::Context, lang: vulcanio_host::Lang) {
        with_app(|fig| fig.session.prefs.language = lang_code(lang));
        ctx.request_repaint();
    }

    fn set_theme(&mut self, ctx: &egui::Context, dark: bool) {
        with_app(|fig| fig.session.prefs.theme = if dark { "dark" } else { "light" }.to_string());
        ctx.request_repaint();
    }

    fn is_idle(&self) -> bool {
        with_app(|fig| fig.session.docs.is_empty()).unwrap_or(true)
    }
}
fn opens_new_file(first_run: bool, start_on_home: bool, control: bool) -> bool {
    !first_run && (!start_on_home || control)
}
