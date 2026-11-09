//! Vulcanio Vector as a Vulcanio module: the desktop app's frame logic and UI unchanged, created inside
//! the Vulcanio window instead of its own.
#![allow(dead_code, unused_imports)]

mod automation_args;
mod caps_lock;
mod clipboard;
#[cfg(feature = "wgpu")]
mod compositor_gpu;
mod control_server;
mod cursor_theme;
#[cfg(feature = "wgpu")]
mod gpu;
#[cfg(target_os = "linux")]
mod linux_stylus;
mod logging;
#[cfg(all(target_os = "macos", feature = "wgpu"))]
mod mac_color_space;
#[cfg(target_os = "macos")]
mod mac_fonts;
#[cfg(target_os = "macos")]
mod mac_menu;
#[cfg(target_os = "macos")]
mod mac_save_format;
#[cfg(target_os = "macos")]
mod mac_screen_color;
mod mac_tablet;
#[cfg(target_os = "macos")]
mod mac_window;
#[cfg(target_os = "macos")]
mod open_documents;
#[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "dragonfly", target_os = "netbsd", target_os = "openbsd", test))]
mod portal_save;
mod prefs_dir;
mod printing;
#[cfg(not(target_os = "macos"))]
mod single_instance;
#[cfg(windows)]
mod system_fonts;
mod tablet;
mod window;
mod window_frame;
mod windows_alt_codes;
#[cfg(windows)]
mod windows_cores;
mod windows_stylus;
mod xwayland;

use vectorcraft_engine::Session;
use vectorcraft_engine::cmd::fileio;
use vectorcraft_engine::file_access::{self, AutomationRoots};
use vectorcraft_ui_egui::graphics::GraphicsLoss;
use vectorcraft_ui_egui::picks::PickRequest;
use vectorcraft_ui_egui::{ClipboardProbeFactory, FilePick, Services, VectorcraftApp};

mod app;
mod host_services;
mod prefs;
mod titlebar;

#[cfg(all(test, feature = "wgpu"))]
use app::bring_window_forward;
use app::native_input_hook;
use app::open_files;
use host_services::services;
#[cfg(all(test, feature = "wgpu"))]
use prefs::{KEEP_UI_PREFS, decode_saved_prefs, keep_prefs_copy, prefs_bytes};
use prefs::{load_prefs, log_dir, prefs_enabled, prefs_path, read_prefs};
#[cfg(all(test, feature = "wgpu", any(target_os = "macos", target_os = "windows")))]
use titlebar::CUSTOM_TITLEBAR;
#[cfg(feature = "wgpu")]
use titlebar::adapter_summary;
#[cfg(all(test, feature = "wgpu"))]
use titlebar::custom_titlebar_from;
#[cfg(all(test, feature = "wgpu"))]
use titlebar::opted_into_system_title_bar;
#[cfg(windows)]
use titlebar::set_taskbar_icon;
use titlebar::{custom_titlebar, native_options};

struct App {
    app: VectorcraftApp,
    host_caps_lock: caps_lock::HostCapsLock,
    /// Reported by wgpu when the window's graphics device is lost (a driver reset).
    graphics_loss: GraphicsLoss,
    /// The graphics device was lost and the unsaved changes are kept for Data Recovery.
    graphics_lost: bool,
    /// Frames the UI has run (see [`end_before_teardown`]).
    frames: u64,
    /// Keeps the custom title bar's window borderless (Windows, #1758).
    window_frame: Option<window_frame::WindowFrameFn>,
    /// The main window; used to read and set maximized state the UI layer does not see on Linux (#3635).
    winit_window: Option<std::sync::Arc<winit::window::Window>>,
    /// A saved or fitted geometry asked for maximize once the window is mapped (Linux).
    pending_maximize: bool,
    /// Frames left to retry [`window::apply_pending_maximize`].
    maximize_tries: u8,
}

/// `vectorcraft --help`. Headless work (convert, run, MCP) is `vectorcraft-cli`'s.

/// Vulcanio Vector's Windows message hook (Alt + keypad codes, the stylus): Vulcanio's window calls it
/// for every message, as Vulcanio Vector's own window did. `true` consumes the message.
#[cfg(windows)]
pub fn msg_hook(msg: *const std::ffi::c_void) -> bool {
    windows_alt_codes::msg_hook(msg);
    windows_stylus::msg_hook(msg)
}

/// Process-wide set-up, once, before the window opens (fonts and the system language).
pub fn startup() {
    vectorcraft_ui_egui::i18n::detect_system_lang_in_background();
    #[cfg(windows)]
    windows_cores::install();
    #[cfg(windows)]
    system_fonts::install();
    prefs_dir::install_fonts();
}

/// Create Vulcanio Vector inside the Vulcanio window.
pub fn create(host: &vulcanio_host::HostContext<'_>) -> Box<dyn vulcanio_host::Module> {
    static STARTED: std::sync::Once = std::sync::Once::new();
    STARTED.call_once(startup);
    let cc = host.cc;
    let saved = read_prefs();
    let mut session = Session::new();
    prefs_dir::attach_profile_store(&mut session);
    prefs_dir::attach_plugin_store(&mut session);
    VectorcraftApp::install_ime_reset_hooks(&cc.egui_ctx);
    let mut app = VectorcraftApp::new(session, services(cc.winit_window().cloned(), cc.egui_ctx.clone()));
    // No update check: Vulcanio updates as one app, never Vulcanio Vector's executable on its own.
    load_prefs(&mut app, saved);
    prefs_dir::attach_pdf_preset_store(&mut app.session);
    let winit_window = cc.winit_window().cloned();
    let swatches = prefs_path().and_then(|p| Some(p.parent()?.join("Swatches").to_string_lossy().to_string()));
    app.session.swatch_libraries.set_user_dir(swatches);
    let styles = prefs_path().and_then(|p| Some(p.parent()?.join("Graphic Styles").to_string_lossy().to_string()));
    app.session.style_libraries.set_user_dir(styles);
    let symbols = prefs_path().and_then(|p| Some(p.parent()?.join("Symbols").to_string_lossy().to_string()));
    app.session.symbol_libraries.set_user_dir(symbols);
    let brushes = prefs_path().and_then(|p| Some(p.parent()?.join("Brushes").to_string_lossy().to_string()));
    app.session.brush_libraries.set_user_dir(brushes);
    vectorcraft_ui_egui::panels::library_panel::restore(&mut app);
    if prefs_enabled() {
        let libraries = prefs_path().and_then(|p| Some(p.parent()?.join("Libraries").to_string_lossy().to_string()));
        app.session.libraries.set_dir(libraries);
    }
    if std::env::var_os("VECTORCRAFT_NO_PREFS").is_none() {
        let recovery = prefs_path().and_then(|p| Some(p.parent()?.join("Data Recovery").to_string_lossy().to_string()));
        app.session.recovery.set_default_folder(recovery);
    }
    let graphics_loss = GraphicsLoss::default();
    #[cfg(feature = "wgpu")]
    if let Some(rs) = &cc.wgpu_render_state {
        let summary = adapter_summary(&rs.adapter.get_info());
        app.graphics_adapter = Some(summary);
        let (loss, ctx) = (graphics_loss.clone(), cc.egui_ctx.clone());
        rs.device.set_device_lost_callback(move |reason, msg| loss.report(&ctx, format!("{reason:?}: {msg}")));
    }
    // Vulcanio draws the window frame (the system's), so Vulcanio Vector's own title bar stays off.
    app.custom_titlebar = false;
    app.session.prefs.interface_language = host.lang.code().to_string();
    // Inside Vulcanio every app starts on its home page.
    app.session.prefs.show_home_screen = true;
    if let Some(port) = automation_args::env_control_port(std::env::var_os("VECTORCRAFT_CONTROL_PORT")).ok().flatten() {
        let rx = control_server::start(port, cc.egui_ctx.clone());
        app = app.with_control(rx);
    }
    app.services.native_input_hook = native_input_hook(&app);
    let files: Vec<String> = host.files.iter().map(|p| p.to_string_lossy().into_owned()).collect();
    file_access::unconfined(|| open_files(&mut app, files));
    let window_frame = window_frame::service(cc, false);
    Box::new(App {
        host_caps_lock: caps_lock::HostCapsLock::new(winit_window.as_deref()),
        app,
        graphics_loss,
        graphics_lost: false,
        frames: 0,
        window_frame,
        winit_window,
        pending_maximize: false,
        maximize_tries: 0,
    })
}

impl vulcanio_host::Module for App {
    fn app(&mut self) -> &mut dyn eframe::App {
        self
    }

    fn open_paths(&mut self, ctx: &egui::Context, paths: &[std::path::PathBuf]) {
        let files: Vec<String> = paths.iter().map(|p| p.to_string_lossy().into_owned()).collect();
        file_access::unconfined(|| open_files(&mut self.app, files));
        ctx.request_repaint();
    }

    fn new_document(&mut self, ctx: &egui::Context) {
        if let Err(e) = self.app.run("file.newDialog", serde_json::json!({})) {
            log::warn!("New: {e}");
        }
        ctx.request_repaint();
    }

    fn set_language(&mut self, ctx: &egui::Context, lang: vulcanio_host::Lang) {
        self.app.session.prefs.interface_language = lang.code().to_string();
        ctx.request_repaint();
    }

    fn set_theme(&mut self, ctx: &egui::Context, dark: bool) {
        use vectorcraft_ui_egui::theme::Brightness;
        self.app.ui.brightness = if dark { Brightness::MediumDark } else { Brightness::Light };
        ctx.request_repaint();
    }

    fn is_idle(&self) -> bool {
        self.app.session.documents().is_empty()
    }
}
