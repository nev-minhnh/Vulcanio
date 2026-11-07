//! One log for Krasis and every module: standard error and `<data dir>/logs/krasis.log`.
//! `KRASIS_LOG` (or `RUST_LOG`) sets the level: `error`, `warn`, `info` (default), `debug`, `trace`.

use std::io::Write;
use std::sync::Mutex;

struct Logger {
    level: log::LevelFilter,
    file: Mutex<Option<std::fs::File>>,
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata<'_>) -> bool {
        // The noisiest graphics crates only say what is wrong.
        let noisy = ["wgpu", "naga", "egui_wgpu", "winit", "calloop", "sctk"].iter().any(|p| metadata.target().starts_with(p));
        metadata.level() <= if noisy { self.level.min(log::LevelFilter::Warn) } else { self.level }
    }

    fn log(&self, record: &log::Record<'_>) {
        if !self.enabled(record.metadata()) {
            return;
        }
        let line = format!("{} {:<5} {}: {}\n", chrono::Local::now().format("%H:%M:%S%.3f"), record.level(), record.target(), record.args());
        let _ = std::io::stderr().write_all(line.as_bytes());
        if let Ok(mut f) = self.file.lock()
            && let Some(f) = f.as_mut()
        {
            let _ = f.write_all(line.as_bytes());
        }
    }

    fn flush(&self) {
        if let Ok(mut f) = self.file.lock()
            && let Some(f) = f.as_mut()
        {
            let _ = f.flush();
        }
    }
}

pub fn install() {
    let level = std::env::var("KRASIS_LOG")
        .or_else(|_| std::env::var("RUST_LOG"))
        .ok()
        .and_then(|v| v.parse::<log::LevelFilter>().ok())
        .unwrap_or(log::LevelFilter::Info);
    let file = crate::data_dir().map(|d| d.join("logs")).and_then(|dir| {
        std::fs::create_dir_all(&dir).ok()?;
        let path = dir.join("krasis.log");
        // Keep one previous session's log.
        if path.metadata().is_ok_and(|m| m.len() > 0) {
            let _ = std::fs::rename(&path, dir.join("krasis.previous.log"));
        }
        std::fs::File::create(path).ok()
    });
    let logger = Box::leak(Box::new(Logger { level, file: Mutex::new(file) }));
    if log::set_logger(logger).is_ok() {
        log::set_max_level(level);
    }
}
