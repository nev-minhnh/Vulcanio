//! Vulcanio Layout as a Vulcanio module: the desktop app's frame logic and UI unchanged, created inside
//! the Vulcanio window instead of its own.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
#![allow(dead_code)]

#[cfg(target_os = "macos")]
mod apple_events;
mod control_server;
mod gpu;
mod gpu_limits;
mod logging;
#[cfg(target_os = "macos")]
mod native_menu;
#[cfg(target_os = "macos")]
mod pointer;
mod profile;
mod window_frame;

use designcraft_engine::Session;
use designcraft_ui_egui::{DesignApp, Services};

struct App {
    app: DesignApp,
    #[cfg(target_os = "macos")]
    menu: Option<native_menu::NativeMenu>,
    /// The window's start: the adapter and backend chosen for it; see `gpu`.
    startup: std::sync::Arc<gpu::Startup>,
    /// Documents and quit requests from Finder, the Dock and Open With.
    #[cfg(target_os = "macos")]
    apple_events: fmv_macos_events::Inbox,
    /// The window's `movable` flag as last set; see `craft_window::title_bar`.
    #[cfg(target_os = "macos")]
    movable: Option<bool>,
    /// Frames begun so far.
    frames: u32,
    /// The device whose uncaptured errors are logged once the window is past its start-up frames
    /// (`gpu_limits::log_uncaptured_errors`). Until then wgpu's default handler panics, so a
    /// failing adapter still makes `gpu` start again on the next one.
    log_errors_after_startup: Option<eframe::wgpu::Device>,
}

impl eframe::App for App {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.startup.enter();
        // Queue/deadline work only. A hidden `run_logic` tick also reports pass index zero, so
        // startup counters and native-menu polling must not live here.
        self.app.logic(ctx);
        self.startup.leave();
    }
    fn raw_input_hook(&mut self, ctx: &egui::Context, raw: &mut egui::RawInput) {
        self.app.raw_input_hook(ctx, raw);
    }
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        self.startup.enter();
        // `App::ui` is the only signal that a real layout pass ran. A discarded multipass first
        // pass reports index 0 once, so these once-per-frame counters/menu polls still run once.
        if ui.ctx().current_pass_index() == 0 {
            if let Some(note) = self.startup.take_notice() {
                self.app.status(note);
            }
            // The third UI callback: two frames went through `ui`, so the first was presented and
            // the driver survived it (a fault at the first present is what `gpu` falls back for).
            if self.frames == 2 {
                self.startup.presented();
            }
            self.frames = self.frames.saturating_add(1);
            if u64::from(self.frames) >= gpu::STARTUP_FRAMES
                && let Some(device) = self.log_errors_after_startup.take()
            {
                gpu_limits::log_uncaptured_errors(&device);
            }
            #[cfg(target_os = "macos")]
            {
                if self.menu.is_none() && std::env::var_os("DESIGNCRAFT_NO_NATIVE_MENU").is_none() {
                    self.menu = Some(native_menu::NativeMenu::install(&mut self.app, ui.ctx()));
                }
                if let Some(m) = &mut self.menu {
                    m.poll(&mut self.app, ui.ctx());
                }
                // Noted for a start again after a graphics failure, which reopens them (`gpu::restart`).
                self.startup.opening(apple_events::poll(&self.apple_events, &mut self.app, ui.ctx()));
            }
        }
        self.app.ui(ui);
        self.startup.leave();
        #[cfg(target_os = "macos")]
        if self.app.take_ime_discard() {
            discard_marked_text();
        }
        #[cfg(target_os = "macos")]
        craft_window::title_bar::sync_movable(ui.ctx(), &mut self.movable);
        // After the frame, never inside it: the dialog answers on a later frame. On macOS Save and
        // message boxes are sheets on this window.
        self.app.show_file_dialogs(Some(&*frame));
    }
    fn on_exit(&mut self) {
        // A normal exit before the third frame is not a crash.
        self.startup.presented();
        if let Err(e) = preserve_documents_on_exit(&mut self.app) {
            log::error!("Couldn't preserve documents on exit: {e}");
            eprintln!("designcraft: couldn't preserve documents on exit: {e}");
        }
        save_prefs(&self.app);
    }
}

/// Tell the macOS input method to drop its composition (the Type tool kept the marked text as
/// typed). winit's IME toggle only clears its own copy, so the IME would type it again.
#[cfg(target_os = "macos")]
fn discard_marked_text() {
    if let Some(mtm) = objc2::MainThreadMarker::new()
        && let Some(ic) = objc2_app_kit::NSTextInputContext::currentInputContext(mtm)
    {
        ic.discardMarkedText();
    }
}

/// Orderly OS termination can bypass egui's cancelable close event. Finish only a complete,
/// current quit plan; otherwise preserve every live document, including edits after the timer.
fn preserve_documents_on_exit(app: &mut DesignApp) -> Result<(), String> {
    app.finish_confirmed_quit()?;
    if app.session.documents().is_empty() {
        return Ok(());
    }
    if app.session.recovery_dir.is_none() {
        return if app.session.documents().iter().any(|d| d.is_dirty()) {
            Err("Recovery is unavailable; unsaved documents could not be preserved on exit".into())
        } else {
            Ok(())
        };
    }
    // Also discard stale recovery for clean documents (for example, Undo back to the saved
    // state after the last timer tick). Never choose a fallback recovery path here.
    app.run("file.recovery.save", serde_json::json!({})).map(|_| ())
}

/// `DESIGNCRAFT_NO_PREFS`: read and write nothing in the settings directory (tests, demos).
fn no_prefs() -> bool {
    std::env::var_os("DESIGNCRAFT_NO_PREFS").is_some()
}

fn prefs_path() -> Option<std::path::PathBuf> {
    profile::config_dir().map(|b| b.join("ui.json"))
}

/// Set when `ui.json` could not be read in full and no copy of it could be kept: the file is
/// then left as it is rather than overwritten.
static KEEP_UI_PREFS: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Copy an unreadable (or partly unreadable) `ui.json` beside it, as `ui.unreadable.json` or the
/// first free `ui.unreadable-N.json`, before the next save replaces it. A newer Vulcanio Layout may
/// have written it.
fn keep_unreadable_prefs(path: &std::path::Path, bytes: &[u8]) {
    use std::io::Write as _;
    for n in 1..100u32 {
        let name = if n == 1 { "ui.unreadable.json".to_string() } else { format!("ui.unreadable-{n}.json") };
        match std::fs::OpenOptions::new().write(true).create_new(true).open(path.with_file_name(name)) {
            Ok(mut file) => {
                if file.write_all(bytes).is_ok() {
                    return;
                }
                break;
            }
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {}
            Err(_) => break,
        }
    }
    KEEP_UI_PREFS.store(true, std::sync::atomic::Ordering::Relaxed);
}

fn load_prefs(app: &mut DesignApp) {
    if no_prefs() {
        return;
    }
    if let Some(p) = prefs_path()
        && let Ok(bytes) = std::fs::read(&p)
    {
        match designcraft_ui_egui::UiState::read_preferences(&bytes) {
            Ok(read) => {
                app.ui = read.ui;
                if !read.intact {
                    keep_unreadable_prefs(&p, &bytes);
                }
            }
            Err(_) => keep_unreadable_prefs(&p, &bytes),
        }
        app.sync_keymap();
    }
    // Engine preferences (Preferences dialog, favourites…) live beside the UI state.
    if let Some(p) = prefs_path().map(|p| p.with_file_name("prefs.json"))
        && let Ok(bytes) = std::fs::read(&p)
        && let Ok(prefs) = serde_json::from_slice::<designcraft_engine::Prefs>(&bytes)
    {
        app.session.prefs = prefs;
    }
}

fn save_prefs(app: &DesignApp) {
    if no_prefs() {
        return;
    }
    if let Some(p) = prefs_path() {
        let _ = std::fs::create_dir_all(p.parent().unwrap_or(std::path::Path::new(".")));
        if !KEEP_UI_PREFS.load(std::sync::atomic::Ordering::Relaxed)
            && let Ok(bytes) = serde_json::to_vec_pretty(&app.ui)
        {
            let _ = std::fs::write(&p, bytes);
        }
        if let Ok(bytes) = serde_json::to_vec_pretty(&app.session.prefs) {
            let _ = std::fs::write(p.with_file_name("prefs.json"), bytes);
        }
    }
}

fn services() -> Services {
    Services {
        read: Some(Box::new(|p: &str| std::fs::read(p).map_err(|e| e.to_string()))),
        write: Some(Box::new(|p: &str, b: &[u8]| designcraft_engine::atomic::write_atomic(std::path::Path::new(p), b).map_err(|e| e.to_string()))),
        clipboard_text: Some(Box::new(|| arboard::Clipboard::new().and_then(|mut c| c.get_text()).ok())),
        #[cfg(target_os = "macos")]
        pointer: Some(Box::new(pointer::screen_position)),
        ..Default::default()
    }
}

/// Desktop app ID: the Linux `.desktop` file name and hicolor icon name (Wayland matches windows by it).
const APP_ID: &str = "ai.storyteller.designcraft";

/// The window, Dock, taskbar and Alt-Tab icon. macOS gets Apple's icon grid (transparent margin);
/// elsewhere the full tile. Regenerate with `packaging/icons.sh`.
fn app_icon() -> Option<egui::IconData> {
    #[cfg(target_os = "macos")]
    let png: &[u8] = include_bytes!("../../../assets/app-icon/designcraft-macos-512.png");
    #[cfg(not(target_os = "macos"))]
    let png: &[u8] = include_bytes!("../../../assets/app-icon/hicolor/256x256/apps/ai.storyteller.designcraft.png");
    eframe::icon_data::from_png_bytes(png).map_err(|e| log::warn!("app icon: {e}")).ok()
}

/// The control port from `--control <port>` or `DESIGNCRAFT_CONTROL_PORT` (`source`); a value that
/// is not a port is logged and ignored instead of silently starting no control channel.
fn control_port_from(source: &str, value: Option<String>) -> Option<u16> {
    let value = value?;
    let port = value.trim().parse().ok();
    if port.is_none() {
        log::warn!("{source}: {value:?} is not a port number; the control channel is off");
    }
    port
}


/// Create Vulcanio Layout inside the Vulcanio window.
pub fn create(host: &vulcanio_host::HostContext<'_>) -> Box<dyn vulcanio_host::Module> {
    let cc = host.cc;
    // Vulcanio chose the adapter and backend for the shared window; Vulcanio Layout's start-up record
    // only counts frames and carries notes here.
    let startup = std::sync::Arc::new(gpu::Startup::new(None, Vec::new()));
    let mut device = None;
    if let Some(rs) = &cc.wgpu_render_state {
        device = Some(rs.device.clone());
        let info = rs.adapter.get_info();
        log::info!("Vulcanio Layout graphics: {} on {:?}", info.name, info.backend);
        startup.started_with(info.backend);
        startup.created(&cc.egui_ctx);
    }
    let mut session = Session::new();
    session.recovery_dir = profile::recovery_dir();
    let recovered = session.execute("file.recovery.open", &serde_json::json!({})).ok();
    let mut app = DesignApp::new(session, services());
    // No update check: Vulcanio updates as one app, never Vulcanio Layout's executable on its own.
    if let Some(n) = recovered.as_ref().and_then(|r| r["opened"].as_array()).map(Vec::len).filter(|n| *n > 0) {
        app.status(format!("Recovered {n} unsaved document{} from the last session.", if n == 1 { "" } else { "s" }));
    }
    load_prefs(&mut app);
    if let Some(note) = profile::unwritable_portable_dir()
        .map(|dir| format!("Portable mode is off: {} cannot be written, so settings are kept in the usual place.", dir.display()))
    {
        app.status(note);
    }
    app.ui.language = host.lang.code().to_string();
    // Vulcanio draws the window frame (the system's), so Vulcanio Layout's own title bar stays off.
    app.integrated_titlebar = false;
    app.custom_titlebar = false;
    app.services.window_frame = window_frame::service(cc, false);
    if let Some(port) = control_port_from("DESIGNCRAFT_CONTROL_PORT", std::env::var("DESIGNCRAFT_CONTROL_PORT").ok()) {
        let rx = control_server::start(port, cc.egui_ctx.clone());
        app = app.with_control(rx);
    }
    for f in &host.files {
        let f = f.to_string_lossy();
        if let Err(e) = app.open_file("file.open", serde_json::json!({"path": f})) {
            log::warn!("{f}: {e}");
        }
    }
    startup.ready();
    Box::new(App {
        app,
        #[cfg(target_os = "macos")]
        menu: None,
        startup,
        #[cfg(target_os = "macos")]
        apple_events: fmv_macos_events::Inbox::default(),
        #[cfg(target_os = "macos")]
        movable: None,
        frames: 0,
        log_errors_after_startup: device,
    })
}

impl vulcanio_host::Module for App {
    fn app(&mut self) -> &mut dyn eframe::App {
        self
    }

    fn open_paths(&mut self, ctx: &egui::Context, paths: &[std::path::PathBuf]) {
        for p in paths {
            let f = p.to_string_lossy();
            if let Err(e) = self.app.open_file("file.open", serde_json::json!({"path": f})) {
                self.app.status(format!("{f}: {e}"));
            }
        }
        ctx.request_repaint();
    }

    fn new_document(&mut self, ctx: &egui::Context) {
        self.app.ui.dialog = Some(designcraft_ui_egui::dialogs::Dialog::new("newDocument", serde_json::json!({})));
        ctx.request_repaint();
    }

    fn set_language(&mut self, ctx: &egui::Context, lang: vulcanio_host::Lang) {
        self.app.ui.language = lang.code().to_string();
        ctx.request_repaint();
    }

    fn set_theme(&mut self, ctx: &egui::Context, dark: bool) {
        use designcraft_ui_egui::theme::Brightness;
        self.app.ui.brightness = if dark { Brightness::MediumDark } else { Brightness::Light };
        ctx.request_repaint();
    }

    fn is_idle(&self) -> bool {
        self.app.session.documents().is_empty()
    }
}
