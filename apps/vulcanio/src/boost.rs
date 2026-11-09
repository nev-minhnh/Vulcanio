//! Startup boost, as in Microsoft Edge: shortly after Windows signs in, a background run of
//! `vulcanio.exe --startup-boost` reads the program once and wakes the graphics driver, then quits.
//! The first real start after the computer boots then finds the program in memory, already scanned
//! by the antivirus, and the driver loaded, instead of doing all of that while you wait.

use std::io::Read as _;
use std::time::Duration;

/// The command-line switch for the background run.
pub const ARG: &str = "--startup-boost";

/// The background run. Low CPU and disk priority throughout; never opens a window.
pub fn run() {
    background_priority();
    // Let sign-in and the desktop settle first.
    std::thread::sleep(Duration::from_secs(if std::env::var_os("VULCANIO_BOOST_NOW").is_some() { 0 } else { 25 }));
    if let Ok(exe) = std::env::current_exe() {
        read_through(&exe);
    }
    warm_graphics();
}

/// Read a file start to end and drop the bytes: Windows keeps them in its file cache and the
/// antivirus scans the file now rather than at the next start.
fn read_through(path: &std::path::Path) {
    let Ok(mut f) = std::fs::File::open(path) else { return };
    let mut buf = vec![0u8; 1 << 20];
    while let Ok(n) = f.read(&mut buf) {
        if n == 0 {
            break;
        }
    }
}

/// Create (and drop) a graphics device the way the window does, so the driver's files are loaded.
fn warm_graphics() {
    use eframe::wgpu;
    let backends = if cfg!(windows) { wgpu::Backends::DX12 } else { wgpu::Backends::PRIMARY };
    let mut desc = wgpu::InstanceDescriptor::new_without_display_handle();
    desc.backends = backends;
    desc.backend_options.dx12.shader_compiler = wgpu::Dx12Compiler::Fxc;
    let instance = wgpu::Instance::new(desc);
    pollster::block_on(async {
        if let Ok(adapter) = instance.request_adapter(&wgpu::RequestAdapterOptions::default()).await {
            let _ = adapter.request_device(&wgpu::DeviceDescriptor::default()).await;
        }
    });
}

#[cfg(windows)]
fn background_priority() {
    use windows_sys::Win32::System::Threading::{GetCurrentProcess, PROCESS_MODE_BACKGROUND_BEGIN, SetPriorityClass};
    // SAFETY: the pseudo-handle of this process; background mode lowers CPU, I/O and memory priority.
    unsafe {
        SetPriorityClass(GetCurrentProcess(), PROCESS_MODE_BACKGROUND_BEGIN);
    }
}

#[cfg(not(windows))]
fn background_priority() {}

#[cfg(windows)]
mod run_key {
    use windows_sys::Win32::Foundation::ERROR_SUCCESS;
    use windows_sys::Win32::System::Registry::{HKEY_CURRENT_USER, RRF_RT_REG_SZ, REG_SZ, RegDeleteKeyValueW, RegGetValueW, RegSetKeyValueW};

    const KEY: &str = r"Software\Microsoft\Windows\CurrentVersion\Run";
    const VALUE: &str = "Vulcanio Startup Boost";

    fn wide(s: &str) -> Vec<u16> {
        s.encode_utf16().chain(std::iter::once(0)).collect()
    }

    fn current() -> Option<String> {
        let (key, value) = (wide(KEY), wide(VALUE));
        let mut buf = vec![0u16; 2048];
        let mut len = (buf.len() * 2) as u32;
        // SAFETY: valid NUL-terminated strings and a buffer of `len` bytes.
        let r = unsafe { RegGetValueW(HKEY_CURRENT_USER, key.as_ptr(), value.as_ptr(), RRF_RT_REG_SZ, std::ptr::null_mut(), buf.as_mut_ptr().cast(), &mut len) };
        if r != ERROR_SUCCESS {
            return None;
        }
        let n = (len as usize / 2).saturating_sub(1);
        Some(String::from_utf16_lossy(&buf[..n.min(buf.len())]))
    }

    pub fn set(command: Option<&str>) {
        let (key, value) = (wide(KEY), wide(VALUE));
        match command {
            Some(cmd) => {
                if current().as_deref() == Some(cmd) {
                    return;
                }
                let data = wide(cmd);
                // SAFETY: valid NUL-terminated strings; `data` is a REG_SZ including its NUL.
                unsafe {
                    RegSetKeyValueW(HKEY_CURRENT_USER, key.as_ptr(), value.as_ptr(), REG_SZ, data.as_ptr().cast(), (data.len() * 2) as u32);
                }
            }
            None => {
                if current().is_some() {
                    // SAFETY: valid NUL-terminated strings.
                    unsafe {
                        RegDeleteKeyValueW(HKEY_CURRENT_USER, key.as_ptr(), value.as_ptr());
                    }
                }
            }
        }
    }
}

/// Turn the sign-in background run on or off for the current user (Windows; elsewhere nothing).
pub fn register(enabled: bool) {
    #[cfg(windows)]
    {
        let command = std::env::current_exe().ok().map(|exe| format!("\"{}\" {ARG}", exe.display()));
        run_key::set(if enabled { command.as_deref() } else { None });
    }
    #[cfg(not(windows))]
    let _ = enabled;
}
