//! Krasis — Your creative oasis.
//!
//! One window for six creative apps: PhotoCraft (photo editing), LightCraft (photo library and
//! RAW), VectorCraft (vector illustration), DesignCraft (page layout), FigCraft (interface design)
//! and CADCraft (CAD drafting).
//!
//! Usage: `krasis [--module photo|light|vector|design|fig|cad] [--new-instance] [files…]`
#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

mod associations;
mod dialogs;
mod home;
mod i18n;
mod icons;
mod instance;
mod logging;
mod memory;
mod modules;
mod settings;
mod shell;

use std::path::PathBuf;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicU8, Ordering};

use modules::ModuleId;

/// The eframe app id: Krasis' settings folder and window state.
pub const APP_ID: &str = "Krasis";

/// Krasis' data folder (settings, logs, the running instance's port).
pub fn data_dir() -> Option<PathBuf> {
    eframe::storage_dir(APP_ID)
}

static WAKE: OnceLock<egui::Context> = OnceLock::new();

pub fn set_wake_context(ctx: egui::Context) {
    let _ = WAKE.set(ctx);
}

/// Wake the window from another thread.
pub fn wake() {
    if let Some(ctx) = WAKE.get() {
        ctx.request_repaint();
    }
}

/// The app in front (`ModuleId::index() + 1`, 0 for home), for the Windows message hook.
static FRONT: AtomicU8 = AtomicU8::new(0);

pub fn set_front(module: Option<ModuleId>) {
    FRONT.store(module.map_or(0, |m| m.index() as u8 + 1), Ordering::Relaxed);
}

#[cfg_attr(not(windows), allow(dead_code))]
fn front() -> Option<ModuleId> {
    match FRONT.load(Ordering::Relaxed) {
        0 => None,
        n => ModuleId::ALL.get(usize::from(n) - 1).copied(),
    }
}

/// The hosted apps update with Krasis, never on their own: a self-update would replace Krasis'
/// executable with one app's. The Craft update library skips an app the shared version file says
/// something else manages; Krasis names itself for all six.
fn pin_versions() {
    let Some(dir) = data_dir() else { return };
    let path = dir.join("versions.json");
    let body = serde_json::json!({ "apps": {
        "photocraft": "managed-by-krasis", "lightcraft": "managed-by-krasis", "vectorcraft": "managed-by-krasis",
        "designcraft": "managed-by-krasis", "figcraft": "managed-by-krasis", "cadcraft": "managed-by-krasis"
    }});
    if std::fs::create_dir_all(&dir).is_ok() && std::fs::write(&path, body.to_string()).is_ok() {
        // SAFETY: called first thing in `main`, before any other thread exists.
        #[allow(unsafe_code)]
        unsafe {
            std::env::set_var("CRAFT_VERSIONS_FILE", &path);
        }
    }
}

const HELP: &str = "\
Krasis — Your creative oasis

Usage: krasis [options] [files…]

Options:
  --module <name>   Open this app: photo, light, vector, design, fig or cad
  --new-instance    Start another Krasis instead of handing the files to the running one
  --version         Print the version
  -h, --help        Print this help
";

struct Args {
    files: Vec<PathBuf>,
    module: Option<ModuleId>,
    new_instance: bool,
}

fn parse_args() -> Result<Args, String> {
    let mut args = Args { files: Vec::new(), module: None, new_instance: false };
    let mut it = std::env::args_os().skip(1);
    while let Some(a) = it.next() {
        match a.to_str() {
            Some("--module") => {
                let name = it.next().and_then(|n| n.into_string().ok()).unwrap_or_default();
                args.module = Some(ModuleId::from_key(&name).ok_or_else(|| format!("unknown app {name:?}"))?);
            }
            Some("--new-instance") => args.new_instance = true,
            Some("--version") => {
                println!("krasis {}", env!("CARGO_PKG_VERSION"));
                std::process::exit(0);
            }
            Some("-h" | "--help") => {
                print!("{HELP}");
                std::process::exit(0);
            }
            Some(flag) if flag.starts_with("--") => return Err(format!("unknown option {flag}")),
            _ => {
                let p = PathBuf::from(a);
                args.files.push(std::path::absolute(&p).unwrap_or(p));
            }
        }
    }
    Ok(args)
}

fn icon() -> Option<egui::IconData> {
    eframe::icon_data::from_png_bytes(include_bytes!("../assets/krasis-256.png")).ok()
}

fn native_options(low_latency: bool) -> eframe::NativeOptions {
    let viewport = egui::ViewportBuilder::default()
        .with_title("Krasis")
        .with_app_id(APP_ID)
        .with_inner_size([1480.0, 920.0])
        .with_min_inner_size([960.0, 600.0])
        .with_drag_and_drop(true)
        .with_decorations(true);
    let viewport = match icon() {
        Some(i) => viewport.with_icon(i),
        None => viewport,
    };
    let mut options = eframe::NativeOptions {
        viewport,
        renderer: eframe::Renderer::Wgpu,
        persist_window: true,
        centered: true,
        // Every app installs its own font chain; no system-font discovery behind their backs.
        system_font_fallback: false,
        // CADCraft clips layout viewports with the stencil buffer.
        stencil_buffer: cadcraft_embed::STENCIL_BITS,
        ..Default::default()
    };
    // Presentation: vsync with two frames in flight paces every frame to the display, as Edge
    // does; the low-latency choice presents at once for pen work.
    let surface = &mut options.wgpu_options.surface;
    if low_latency {
        surface.present_mode = eframe::wgpu::PresentMode::AutoNoVsync;
        surface.desired_maximum_frame_latency = Some(1);
    } else {
        surface.present_mode = eframe::wgpu::PresentMode::AutoVsync;
        surface.desired_maximum_frame_latency = Some(2);
    }
    if let eframe::egui_wgpu::WgpuSetup::CreateNew(create) = &mut options.wgpu_options.wgpu_setup {
        // Windows: DirectX 12 (FXC shaders, no extra DLLs), then Vulkan, then OpenGL.
        let preferred = if cfg!(windows) {
            eframe::wgpu::Backends::DX12 | eframe::wgpu::Backends::VULKAN | eframe::wgpu::Backends::GL
        } else {
            eframe::wgpu::Backends::PRIMARY | eframe::wgpu::Backends::GL
        };
        create.instance_descriptor.backends = eframe::wgpu::Backends::from_env().unwrap_or(preferred);
        create.instance_descriptor.backend_options.dx12.shader_compiler =
            eframe::wgpu::Dx12Compiler::from_env().unwrap_or(eframe::wgpu::Dx12Compiler::Fxc);
        create.power_preference = eframe::wgpu::PowerPreference::from_env().unwrap_or(eframe::wgpu::PowerPreference::HighPerformance);
        // One device for every app: the adapter's own limits (LightCraft develops on it, CADCraft
        // and DesignCraft need its largest buffers and textures) and PhotoCraft's GPU timing.
        let base = create.device_descriptor.clone();
        create.device_descriptor = std::sync::Arc::new(move |adapter: &eframe::wgpu::Adapter| {
            let mut d = base(adapter);
            if adapter.get_info().backend != eframe::wgpu::Backend::Gl {
                d.required_limits = adapter.limits();
            } else {
                d.required_limits.max_texture_dimension_2d = d.required_limits.max_texture_dimension_2d.max(adapter.limits().max_texture_dimension_2d);
            }
            let timing = eframe::wgpu::Features::TIMESTAMP_QUERY | eframe::wgpu::Features::TIMESTAMP_QUERY_INSIDE_ENCODERS;
            d.required_features |= adapter.features() & timing;
            // Smaller GPU memory blocks: integrated graphics share the computer's RAM.
            d.memory_hints = eframe::wgpu::MemoryHints::MemoryUsage;
            d.label = Some("krasis window");
            d
        });
    }
    #[cfg(windows)]
    {
        use winit::platform::windows::EventLoopBuilderExtWindows as _;
        options.event_loop_builder = Some(Box::new(|builder| {
            builder.with_msg_hook(|msg| {
                // VectorCraft's own hook (Alt + keypad codes, the stylus), while it is in front.
                if front() == Some(ModuleId::Vector) { vectorcraft_embed::msg_hook(msg) } else { false }
            });
        }));
    }
    // winit has file drag-and-drop on X11 only: run through XWayland when there is one.
    #[cfg(all(unix, not(target_os = "macos")))]
    if std::env::var_os("DISPLAY").is_some() && std::env::var_os("KRASIS_WAYLAND").is_none() {
        options.event_loop_builder = Some(Box::new(|b| {
            use winit::platform::x11::EventLoopBuilderExtX11;
            b.with_x11();
        }));
    }
    options
}

fn main() -> std::process::ExitCode {
    // A copy of Krasis started to read or write a DWG file (CADCraft) does only that.
    if let Some(code) = cadcraft_embed::worker_main() {
        std::process::exit(code);
    }
    pin_versions();
    let args = match parse_args() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("krasis: {e}\n\n{HELP}");
            return std::process::ExitCode::from(2);
        }
    };
    let handoff = instance::Handoff { files: args.files.clone(), module: args.module.map(|m| m.key().to_string()) };
    let inbox = match instance::start(&handoff, args.new_instance) {
        instance::Start::Forwarded => return std::process::ExitCode::SUCCESS,
        instance::Start::Primary(rx, _tx) => rx,
    };
    logging::install();
    log::info!("Krasis {} ({} {})", env!("CARGO_PKG_VERSION"), std::env::consts::OS, std::env::consts::ARCH);
    let settings = settings::Settings::load_early();
    let options = native_options(settings.low_latency);
    let present_mode = options.wgpu_options.surface.present_mode;
    let result = eframe::run_native(
        APP_ID,
        options,
        Box::new(move |cc| {
            if let Some(rs) = &cc.wgpu_render_state {
                let info = rs.adapter.get_info();
                log::info!("graphics: {} on {:?} ({:?}, driver {} {})", info.name, info.backend, info.device_type, info.driver, info.driver_info);
            }
            Ok(Box::new(shell::Krasis::new(cc, args.files, args.module, inbox, present_mode)))
        }),
    );
    instance::release();
    match result {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(e) => {
            log::error!("Krasis stopped: {e}");
            std::process::ExitCode::FAILURE
        }
    }
}
