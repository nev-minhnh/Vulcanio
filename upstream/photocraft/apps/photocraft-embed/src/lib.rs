//! Krasis Photo as a Krasis module: the desktop app's frame logic and UI unchanged, created inside
//! the Krasis window instead of its own.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
#![allow(dead_code, unused_imports)]

mod app_dirs;
mod app_icon;
mod appearance;
#[cfg(target_os = "macos")]
mod apple_events;
mod caps_lock;
mod caption_style;
mod control_server;
mod crash_guard;
mod cursor;
mod file_dialog_probe;
mod gpu_startup;
#[cfg(target_os = "linux")]
mod icon_theme;
mod launch_args;
#[cfg(target_os = "macos")]
mod mac_menu;
#[cfg(target_os = "macos")]
mod mac_window;
// Pure logic is tested on every platform; only Linux runs the check.
#[cfg(not(target_arch = "wasm32"))]
mod external_svg;
#[cfg(any(target_os = "linux", test))]
mod linux_libs;
mod logging;
mod monitor_profile;
mod photoshop_settings;
// Save As through the desktop portal, which reports the file type picked (#6121).
#[cfg(target_os = "linux")]
mod portal_save;
mod presentation;
// The print portal needs the session bus (zbus, Linux only); its pure helpers are tested everywhere.
#[cfg(any(target_os = "linux", test))]
mod print_portal;
mod screen_color;
mod services;
// macOS gets one instance from LaunchServices (open-documents Apple events).
#[cfg(any(not(target_os = "macos"), test))]
mod single_instance;
mod startup_report;
// Tablet support (macOS AppKit monitor, Linux X11/Wayland, Windows barrel button).
mod tablet;
mod ui_state;
mod updates;
mod window_frame;

use photocraft_engine::Session;
use photocraft_ui_egui::PhotocraftApp;

use photocraft_ui_egui::os_window::APP_ID;

/// Windows and Linux can use the app's title bar, with its own caption buttons and edge resizing
/// (`photocraft_ui_egui::titlebar`); the preference below selects the mode. macOS always keeps its
/// system traffic lights over the integrated title strip.
const CUSTOM_TITLEBAR: bool = !cfg!(target_os = "macos");

/// Whether this start draws its own title bar: Windows and Linux do, unless Preferences ›
/// Interface › System Title Bar asks for the system's (#1271, #1316). Read from the saved
/// preferences before the window opens; a missing or unreadable file keeps the default.
fn custom_titlebar(prefs_file: Option<&std::path::Path>) -> bool {
    let prefs: photocraft_engine::prefs::Preferences =
        prefs_file.and_then(|p| std::fs::read_to_string(p).ok()).and_then(|t| serde_json::from_str(&t).ok()).unwrap_or_default();
    CUSTOM_TITLEBAR && !prefs.interface.system_title_bar
}

/// Preferences › Integrations, read before the window opens the way [`custom_titlebar`] is:
/// whether agents may drive this launch at all, and the port to listen on when no `--control`
/// was given (0 = only with `--control`). A missing or unreadable file keeps the defaults.
///
/// The section is read on its own, and when it doesn't read as a whole each value on its own,
/// as the app's lenient load reads them (#6611): a wrong-typed value elsewhere in the file, or
/// in the other field, must not turn Allow Agent Control back on while the dialog shows it off.
fn integrations(prefs_file: Option<&std::path::Path>) -> photocraft_engine::prefs::Integrations {
    use serde_json::Value;
    let mut out = photocraft_engine::prefs::Preferences::default().integrations;
    let file: Option<Value> = prefs_file.and_then(|p| std::fs::read_to_string(p).ok()).and_then(|t| serde_json::from_str(&t).ok());
    let Some(section) = file.as_ref().and_then(|v| v.get("integrations")) else {
        return out;
    };
    if let Ok(whole) = serde_json::from_value(section.clone()) {
        return whole;
    }
    if let Some(allow) = section.get("allowAgentControl").and_then(Value::as_bool) {
        out.allow_agent_control = allow;
    }
    if let Some(port) = section.get("controlPort").and_then(Value::as_u64).and_then(|p| u32::try_from(p).ok()) {
        out.control_port = port;
    }
    out
}

/// The control port this launch listens on, and the warning to print when a request was refused.
/// `--control` (or `PHOTOCRAFT_CONTROL_PORT`) wins over the preference; Allow Agent Control off
/// refuses both, and says so rather than ignoring the flag silently. A preferred port outside
/// 1..=65535 is refused the same way.
fn control_port_for(cli: Option<u16>, prefs: &photocraft_engine::prefs::Integrations) -> (Option<u16>, Option<String>) {
    if !prefs.allow_agent_control {
        let warning = cli.map(|p| format!("photocraft: Preferences › Integrations › Allow Agent Control is off: not listening on port {p}\n"));
        return (None, warning);
    }
    if let Some(port) = cli {
        return (Some(port), None);
    }
    match prefs.control_port {
        0 => (None, None),
        p => match u16::try_from(p).ok().filter(|p| *p != 0) {
            Some(port) => (Some(port), None),
            None => (None, Some(format!("photocraft: Preferences › Integrations › Control Port {p} is not a usable port: not listening\n"))),
        },
    }
}

/// The main window: 1440 × 900 (shrunk to fit the monitor, and maximized on the first frame
/// when it still doesn't fit, `work_area::fit_window`), centred on the main monitor. Without
/// `centered`, Windows cascades each new window from the top-left corner, so it opened at a
/// different offset every launch (#419). A later launch restores the saved position instead
/// (`main` turns `centered` off, #4826). Wayland compositors place windows themselves.
fn control_setup(
    cli_port: Option<u16>,
    auth: photocraft_automation::args::ControlArgs,
    automation: &photocraft_automation::args::AutomationArgs,
) -> Option<(std::net::TcpListener, String, photocraft_automation::workspace::AuthorizedWorkspace)> {
    let (port, warning) = control_port_for(cli_port, &integrations(services::prefs_file().as_deref()));
    if let Some(warning) = &warning {
        eprint!("{warning}");
    }
    let port = port?;
    let listener = match std::net::TcpListener::bind(("127.0.0.1", port)) {
        Ok(listener) => listener,
        Err(error) => {
            eprintln!("photocraft: control server failed to bind 127.0.0.1:{port}: {error}");
            std::process::exit(2);
        }
    };
    let (supplied, token_file) = (auth.control_token, auth.control_token_file);
    let token = match photocraft_automation::security::server_token(supplied.as_deref(), token_file.as_deref()) {
        Ok(token) => token,
        Err(e) => {
            eprintln!("photocraft: cannot configure control authentication: {e}");
            std::process::exit(2);
        }
    };
    if let Some(path) = token_file {
        eprintln!("photocraft: control token file: {}", path.display());
    } else if supplied.is_none() {
        eprintln!("photocraft: control token: {token}");
    } else {
        eprintln!("photocraft: using supplied control token");
    }
    let workspace = match automation.workspace() {
        Ok(workspace) => workspace,
        Err(error) => {
            eprintln!("photocraft: cannot configure automation workspace: {error}");
            std::process::exit(2);
        }
    };
    Some((listener, token, workspace))
}

fn attach_log_file(_logger: Option<&'static logging::AppLogger>) {}

struct App(PhotocraftApp);

fn lang_pref(lang: krasis_host::Lang) -> &'static str {
    lang.code()
}

/// Create Krasis Photo inside the Krasis window.
pub fn create(host: &krasis_host::HostContext<'_>) -> Box<dyn krasis_host::Module> {
    let cc = host.cc;
    photocraft_engine::malloc_tuning::apply();
    crash_guard::install_hook();
    photocraft_ui_egui::type_tool::warm_fonts();
    let files: Vec<String> = host.files.iter().map(|p| p.to_string_lossy().into_owned()).collect();
    let control_port = std::env::var("PHOTOCRAFT_CONTROL_PORT").ok().and_then(|p| p.trim().parse::<u16>().ok());
    let control = control_setup(
        control_port,
        photocraft_automation::args::ControlArgs { control_token: None, control_token_file: None },
        &photocraft_automation::args::AutomationArgs { automation_read_root: None, automation_write_root: None },
    );
    #[cfg(target_os = "linux")]
    let native_session = linux_libs::session_from_env(|name| std::env::var(name).ok());
    #[cfg(target_os = "linux")]
    let monitor = monitor_profile::detect_async_for_session(native_session);
    #[cfg(not(target_os = "linux"))]
    let monitor = monitor_profile::detect_async();
    let presets = services::presets_dir().map(photocraft_engine::preset_store::open_dir_async);
    let mode = gpu_startup::read_rendering_prefs(services::prefs_file().as_deref()).1;
    let uncapped = host.present_mode == eframe::wgpu::PresentMode::AutoNoVsync || host.present_mode == eframe::wgpu::PresentMode::Immediate;

    let automation = control.as_ref().map(|(_, _, workspace)| workspace.clone());
    let mut services = services::native(automation);
    services.preset_store = presets;
    let session = Session::new();
    PhotocraftApp::install_type_reset_hooks(&cc.egui_ctx);
    let mut app = PhotocraftApp::new(session, services);
    app.min_frame_ms = uncapped.then_some(1000.0 / 240.0);
    // Krasis draws the window frame (the system's), so Krasis Photo's own title bar stays off.
    app.integrated_titlebar = false;
    app.custom_titlebar = false;
    app.background_jobs = std::env::var_os("PHOTOCRAFT_INLINE_JOBS").is_none();
    if let Some(rx) = monitor {
        #[cfg(target_os = "linux")]
        {
            app.services.read_displays = Some(std::sync::Arc::new(move || monitor_profile::detect_async_for_session(native_session)));
        }
        #[cfg(not(target_os = "linux"))]
        {
            app.services.read_displays = Some(std::sync::Arc::new(monitor_profile::detect_async));
        }
        match rx.recv_timeout(std::time::Duration::from_millis(400)) {
            Ok(r) => photocraft_ui_egui::monitor_status::apply(&mut app, r),
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => photocraft_ui_egui::monitor_status::pending(&mut app, rx),
            Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                photocraft_ui_egui::monitor_status::apply(&mut app, Err("the display profile reader stopped without an answer".into()));
            }
        }
    }
    app.perf.gpu_info.canvas = "cpu".into();
    app.perf.gpu_info.selected = "krasis".into();
    if let Some(rs) = cc.wgpu_render_state.clone() {
        app.perf.gpu_info.set_adapter(&rs.adapter.get_info());
        app.perf.gpu_info.set_available(&rs.available_adapters);
        photocraft_ui_egui::texel_guard::install(&cc.egui_ctx, &rs.adapter.get_info());
        let software_window = rs.adapter.get_info().device_type == eframe::wgpu::DeviceType::Cpu;
        if !software_window
            && std::env::var_os("PHOTOCRAFT_CPU_CANVAS").is_none()
            && app.session.prefs().performance.effective_rendering_mode() != photocraft_engine::prefs::RenderingMode::Cpu
            && mode != photocraft_engine::prefs::RenderingMode::Cpu
        {
            app.set_wgpu(rs);
            app.start_compositor(&cc.egui_ctx);
        } else {
            app.watch_window_device(&rs.device);
        }
    }
    if let Some((listener, token, _)) = control {
        let rx = control_server::start(listener, token, cc.egui_ctx.clone());
        app = app.with_control(rx);
    }
    app.services.cursor_pos = cursor::service(cc);
    app.services.window_frame = window_frame::service(cc, false);
    app.services.caps_lock = caps_lock::service(cc);
    app.services.screen_pick = screen_color::service(cc, false);
    #[cfg(target_os = "windows")]
    {
        if let Some(monitor) = tablet::install_windows() {
            // Kept for the life of the process, as the standalone app keeps it for its window.
            std::mem::forget(monitor);
        }
        if let Some(poll) =
            tablet::install_wintab(cc, &app.stylus.feed, app.session.prefs().tools.tablet_input == photocraft_engine::prefs::TabletInput::WinTab)
        {
            app.services.tablet_poll = Some(poll);
        }
    }
    app.services.right_button_down = tablet::right_button_service();
    app.services.pen_end = tablet::pen_end_service();
    set_language_pref(&mut app, &cc.egui_ctx, host.lang);
    app.open_paths(&files);
    if let Some(w) = &app_dirs::current().warning {
        photocraft_ui_egui::notices::post(&mut app, "Portable mode is off", vec![w.clone()], false, None);
    }
    Box::new(App(app))
}

fn set_language_pref(app: &mut PhotocraftApp, _ctx: &egui::Context, lang: krasis_host::Lang) {
    let code = lang_pref(lang);
    if app.session.prefs().interface.language != code
        && let Err(e) = app.run("prefs.set", serde_json::json!({"path": "interface.language", "value": code}))
    {
        log::warn!("Krasis Photo interface language {code}: {e}");
    }
    photocraft_ui_egui::i18n::set_current(photocraft_ui_egui::i18n::Lang::from_pref(code));
}

impl krasis_host::Module for App {
    fn app(&mut self) -> &mut dyn eframe::App {
        &mut self.0
    }

    fn open_paths(&mut self, ctx: &egui::Context, paths: &[std::path::PathBuf]) {
        let files: Vec<String> = paths.iter().map(|p| p.to_string_lossy().into_owned()).collect();
        self.0.open_paths(&files);
        ctx.request_repaint();
    }

    fn new_document(&mut self, ctx: &egui::Context) {
        if let Err(e) = photocraft_ui_egui::menus::invoke(&mut self.0, ctx, "file.new", serde_json::json!({})) {
            log::warn!("New: {e}");
        }
        ctx.request_repaint();
    }

    fn set_language(&mut self, ctx: &egui::Context, lang: krasis_host::Lang) {
        set_language_pref(&mut self.0, ctx, lang);
        ctx.request_repaint();
    }

    fn set_theme(&mut self, ctx: &egui::Context, dark: bool) {
        let mode = if dark { "dark" } else { "light" };
        if self.0.session.prefs().interface.appearance_mode.name() != mode
            && let Err(e) = self.0.run("prefs.set", serde_json::json!({"path": "interface.appearanceMode", "value": mode}))
        {
            log::warn!("Photo theme {mode}: {e}");
        }
        ctx.request_repaint();
    }

    fn is_idle(&self) -> bool {
        self.0.session.documents().is_empty()
    }
}
