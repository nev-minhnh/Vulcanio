//! Vulcanio CAD as a Vulcanio module: the desktop app's frame logic and UI unchanged, created inside
//! the Vulcanio window instead of its own.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
#![allow(dead_code)]

mod control_server;
mod desktop_theme;
mod env;
#[cfg(any(target_os = "windows", test))]
mod graphics;
mod launch_args;
mod logging;
#[cfg(target_os = "macos")]
mod native_menu;
mod spacemouse;
mod system_animations;
mod workspace;

use cadcraft_engine::Session;
use cadcraft_ui_egui::switch::Switch;
use cadcraft_ui_egui::{CadApp, Services, i18n};

struct App {
    cad: CadApp,
    spacemouse: spacemouse::SpaceMouse,
    /// The desktop's light/dark choice where winit reports none (Linux, the BSDs).
    desktop_theme: desktop_theme::DesktopTheme,
    /// Whether the OS shows animations, for the System choice of View ▸ Accessibility ▸ Animation Effects.
    system_animations: system_animations::SystemAnimations,
    workspace: workspace::Workspace,
    workspace_error: Option<String>,
    #[cfg(target_os = "macos")]
    native_menu: Option<native_menu::NativeMenu>,
    /// `CADCRAFT_NO_NATIVE_MENU`: the menus stay inside the window.
    #[cfg(target_os = "macos")]
    menus_in_window: bool,
    /// The `movable` flag last set on the main window.
    #[cfg(target_os = "macos")]
    movable: Option<bool>,
}

impl App {
    /// `ctx` is `None` only for the final attempt at exit.
    fn persist_workspace(&mut self, ctx: Option<&egui::Context>) {
        let result = match ctx {
            None => self.workspace.save_on_exit(&self.cad.ui),
            Some(ctx) => self.workspace.save_if_changed(&self.cad.ui, std::time::Instant::now()).map(|wait| {
                if let Some(wait) = wait {
                    ctx.request_repaint_after(wait);
                }
            }),
        };
        if let Err(error) = result {
            if self.workspace_error.as_ref() != Some(&error) {
                eprintln!("Vulcanio CAD workspace: {error}");
                self.cad.set_status(format!("Workspace could not be saved: {error}"));
                self.workspace_error = Some(error);
            }
        } else {
            self.workspace_error = None;
        }
    }
}

/// View ▸ Theme ▸ System resolves against `raw.system_theme`, which winit leaves empty on Linux
/// and the BSDs: the desktop's answer, where it gives one, stands in for it.
fn follow_desktop(raw: &mut egui::RawInput, desktop: Option<egui::Theme>) {
    if desktop.is_some() {
        raw.system_theme = desktop;
    }
}

impl eframe::App for App {
    /// eframe logic callback: syncs the OS animation setting and runs `CadApp::logic`; when the pass index is 0 (the first pass or a hidden tick) it also installs/polls the macOS native menu, polls the spacemouse and persists the workspace; a quit request closes the viewport.
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.cad.system_animations = self.system_animations.current();
        // Consume control/open/screenshot queues and time-based autosave, including hidden ticks.
        self.cad.guarded_logic(ctx);
        // Host frame work runs here, once per frame, and so also on a hidden logic-only tick (which
        // never reaches `App::ui`): native menus stay current, the spacemouse still navigates and a
        // workspace changed while hidden is persisted. `current_pass_index()` is zero on the first
        // pass and on every hidden tick, and non-zero on later passes of a multi-pass frame.
        if ctx.current_pass_index() == 0 {
            #[cfg(target_os = "macos")]
            {
                if self.native_menu.is_none() && !self.menus_in_window {
                    self.native_menu = Some(native_menu::NativeMenu::install(&mut self.cad, ctx));
                }
                if let Some(m) = &mut self.native_menu {
                    m.poll(&mut self.cad, ctx);
                }
            }
            self.spacemouse.poll(&mut self.cad, ctx);
            // After `cad.logic`, so a control command's workspace change is saved in the same frame.
            self.persist_workspace(Some(ctx));
        }
        if std::mem::take(&mut self.cad.quit_requested) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }
    /// Apply the desktop-theme fallback, forward raw input to `CadApp`, and capture Tab at the command line.
    fn raw_input_hook(&mut self, ctx: &egui::Context, raw: &mut egui::RawInput) {
        follow_desktop(raw, self.desktop_theme.current());
        self.cad.raw_input_hook(ctx, raw);
        cadcraft_ui_egui::cmdline::capture_tab(ctx, raw);
    }
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.cad.guarded_ui(ui);
        #[cfg(target_os = "macos")]
        craft_window::title_bar::sync_movable(ui.ctx(), &mut self.movable);
    }
    /// A normal exit: the workspace is saved, and this run's autosave files are not needed (a
    /// crash leaves them for recovery).
    fn on_exit(&mut self) {
        self.persist_workspace(None);
        self.cad.cancel_screenshots();
        self.cad.shutdown_autosave();
    }
    /// Preferences (the interface theme) survive restarts; eframe saves the window state itself
    /// (`persist_window`); egui state is not kept.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        storage.set_string(cadcraft_ui_egui::options::PROFILE_KEY, self.cad.profile_json());
        storage.set_string(cadcraft_ui_egui::PREFS_KEY, self.cad.prefs_json());
        storage.set_string(cadcraft_ui_egui::autosave::PREFS_KEY, cadcraft_ui_egui::autosave::prefs_json());
    }
    fn persist_egui_memory(&self) -> bool {
        false
    }
}

/// File format hooks for the engine (it stays I/O-agnostic).
fn install_io() {
    cadcraft_engine::cmd::file::set_io(cadcraft_engine::cmd::file::IoHooks {
        read: |b, name| cadcraft_io::read(b, name).map_err(|e| e.to_string()),
        write: |d, name| cadcraft_io::write(d, name).map_err(|e| e.to_string()),
        plot: Some(|d, space, opts| cadcraft_io::plotter::plot(d, space, opts).map_err(|e| e.to_string())),
    });
    cadcraft_engine::cmd::file::set_image_decoder(|b| cadcraft_io::raster::decode(b).map_err(|e| e.to_string()));
    cadcraft_engine::cmd::file::set_write_as(|d, name, o| {
        cadcraft_io::write_with(d, name, &cadcraft_io::SaveOptions { version: o.version, binary: o.binary }).map_err(|e| e.to_string())
    });
    // AutoLISP's entget, entmake and entmod see entities as DXF groups.
    cadcraft_engine::cmd::file::set_entity_codec(cadcraft_engine::cmd::file::EntityCodec {
        groups: cadcraft_io::entity_groups,
        entity: cadcraft_io::entity_from_groups,
    });
}

fn services() -> Services {
    Services {
        pick_open: Some(Box::new(|| {
            rfd::FileDialog::new()
                .add_filter(i18n::t("Drawings (DWG, DXF)"), &["dwg", "dxf", "dwt"])
                .add_filter(i18n::t("SVG drawings (SVG, SVGZ)"), &["svg", "svgz"])
                .add_filter(i18n::t("All files"), &["*"])
                .pick_file()
                .map(|p| p.to_string_lossy().to_string())
        })),
        pick_save: Some(Box::new(|name: &str| {
            filters_for_save(name)
                .iter()
                .fold(rfd::FileDialog::new().set_file_name(name), |d, (label, ext)| d.add_filter(i18n::t(label), &[*ext]))
                .save_file()
                .map(|p| p.to_string_lossy().to_string())
        })),
        // APPLOAD: AutoLISP routines only, starting where the last one was loaded from.
        pick_lisp: Some(Box::new(|dir: &str| {
            let dialog = rfd::FileDialog::new().add_filter(i18n::t("AutoLISP (LSP)"), &["lsp"]).add_filter(i18n::t("All files"), &["*"]);
            let dialog = if dir.is_empty() { dialog } else { dialog.set_directory(dir) };
            dialog.pick_file().map(|p| p.to_string_lossy().to_string())
        })),
        no_file_system: false,
        pick_pgp: Some(Box::new(|dir: Option<&str>| {
            let dialog = rfd::FileDialog::new().add_filter(i18n::t("Alias files (PGP)"), &["pgp"]);
            let dialog = match dir {
                Some(dir) => dialog.set_directory(dir),
                None => dialog,
            };
            dialog.pick_file().map(|p| p.to_string_lossy().to_string())
        })),
    }
}

/// File types for the Save dialog, the suggested name's type first: Windows adds the first
/// filter's extension, and macOS accepts only listed extensions, replacing any other with the
/// first one. DWG must be listed for a `.dwg` name to survive, and a drawing opened from DWG
/// then defaults to DWG; other names keep DXF first.
fn filters_for_save(name: &str) -> [(&'static str, &'static str); 6] {
    let mut filters =
        [("Drawing (DXF)", "dxf"), ("Drawing (DWG)", "dwg"), ("Drawing Template (DWT)", "dwt"), ("SVG", "svg"), ("PNG", "png"), ("PDF", "pdf")];
    let lower = name.to_ascii_lowercase();
    filters.sort_by_key(|(_, ext)| !lower.ends_with(&format!(".{ext}")));
    filters
}


const APP_ID: &str = "ai.storyteller.cadcraft";
/// Stencil bits of the window's render pass (Vulcanio asks for them for every module).
pub const STENCIL_BITS: u8 = 8;

/// A copy of Vulcanio started to read or write a DWG file does only that: the exit code.
pub fn worker_main() -> Option<i32> {
    cadcraft_io::dwg_worker::serve_if_asked()
}

/// Process-wide set-up, once, before the module is created.
fn startup(vars: &env::Env) {
    cadcraft_io::dwg_worker::use_own_worker();
    install_io();
    cadcraft_engine::settings_dir::set(vars.config_dir.clone());
    if let Some(name) = &vars.font_alt {
        cadcraft_engine::fonts::ttf::set_font_alt(name);
    }
    if let Some(list) = &vars.font_fallback {
        cadcraft_engine::fonts::ttf::set_fallback_fonts(list);
    }
}

/// eframe storage for Vulcanio CAD: the standalone app's own `app.ron`.
pub fn storage() -> vulcanio_host::ModuleStorage {
    vulcanio_host::ModuleStorage::for_app_id(APP_ID)
}

fn preference(lang: vulcanio_host::Lang) -> i18n::Preference {
    match lang {
        vulcanio_host::Lang::En => i18n::Preference::English,
        vulcanio_host::Lang::Vi => i18n::Preference::Vietnamese,
    }
}

/// Create Vulcanio CAD inside the Vulcanio window.
pub fn create(host: &vulcanio_host::HostContext<'_>) -> Box<dyn vulcanio_host::Module> {
    let cc = host.cc;
    let vars = env::Env::read();
    startup(&vars);
    let mut app = CadApp::new(Session::empty(), services());
    let (workspace, restored, workspace_error) = workspace::Workspace::load(vars.config_dir.as_deref());
    if let Some(ui) = restored {
        app.ui = ui;
    }
    app.set_system_cursor_default(vars.system_cursor == Switch::On);
    if let Some(error) = &workspace_error {
        log::warn!("Vulcanio CAD workspace: {error}");
        app.set_status(error);
    }
    app.system_languages = vars.locale.clone().map(|s| vec![s]).unwrap_or_else(|| sys_locale::get_locales().collect());
    if let Some(profile) = cc.storage.and_then(|s| s.get_string(cadcraft_ui_egui::options::PROFILE_KEY)) {
        app.load_profile(&profile);
    }
    if let Some(prefs) = cc.storage.and_then(|s| s.get_string(cadcraft_ui_egui::PREFS_KEY)) {
        app.load_prefs(&prefs);
    }
    if let Some(prefs) = cc.storage.and_then(|s| s.get_string(cadcraft_ui_egui::autosave::PREFS_KEY)) {
        cadcraft_ui_egui::autosave::load_prefs(&prefs);
    }
    // Vulcanio's language wins over the one Vulcanio CAD remembered.
    app.ui.interface_language = preference(host.lang);
    if let Some(dir) = &vars.config_dir {
        app.enable_autosave(&dir.join("autosave"));
        // No update check: Vulcanio updates as one app, never Vulcanio CAD's executable on its own.
        let support = dir.join("support");
        if let Err(e) = std::fs::create_dir_all(&support) {
            log::warn!("no LISP support folder {}: {e}", support.display());
        }
        app.session.set_lisp_support_dir(Some(support));
    }
    app.integrated_titlebar = false;
    if host.present_mode == eframe::wgpu::PresentMode::AutoNoVsync {
        app.frame_cap = Some(cadcraft_ui_egui::gpu::FrameCap::new(240));
    }
    if let Some(rs) = &cc.wgpu_render_state {
        app.set_wgpu(rs, eframe::egui_wgpu::depth_format_from_bits(0, STENCIL_BITS));
    }
    if let Some(port) = std::env::var("CADCRAFT_CONTROL_PORT").ok().and_then(|p| p.trim().parse::<u16>().ok()) {
        let rx = control_server::start(port, cc.egui_ctx.clone());
        app = app.with_control(rx);
    }
    for f in &host.files {
        app.open_path(f);
    }
    // Inside Vulcanio CAD starts on its Start page, not a blank drawing.
    let ctx = cc.egui_ctx.clone();
    let desktop_theme = desktop_theme::DesktopTheme::start(move || ctx.request_repaint());
    let ctx = cc.egui_ctx.clone();
    let system_animations = system_animations::SystemAnimations::start(move || ctx.request_repaint());
    Box::new(App {
        desktop_theme,
        system_animations,
        cad: app,
        spacemouse: spacemouse::SpaceMouse::start(cc.egui_ctx.clone()),
        workspace,
        workspace_error,
        #[cfg(target_os = "macos")]
        native_menu: None,
        #[cfg(target_os = "macos")]
        menus_in_window: true,
        #[cfg(target_os = "macos")]
        movable: None,
    })
}

impl vulcanio_host::Module for App {
    fn app(&mut self) -> &mut dyn eframe::App {
        self
    }

    fn open_paths(&mut self, ctx: &egui::Context, paths: &[std::path::PathBuf]) {
        for p in paths {
            self.cad.open_path(p);
        }
        ctx.request_repaint();
    }

    fn new_document(&mut self, ctx: &egui::Context) {
        self.cad.start("new");
        ctx.request_repaint();
    }

    fn set_language(&mut self, ctx: &egui::Context, lang: vulcanio_host::Lang) {
        self.cad.ui.interface_language = preference(lang);
        ctx.request_repaint();
    }

    fn set_theme(&mut self, ctx: &egui::Context, dark: bool) {
        use cadcraft_ui_egui::theme::ThemePref;
        self.cad.ui.theme = if dark { ThemePref::Dark } else { ThemePref::Light };
        ctx.request_repaint();
    }
}
