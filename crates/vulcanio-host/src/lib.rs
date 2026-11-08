//! What Vulcanio and its modules share: the [`Module`] trait every embedded app implements, the
//! interface language, and a per-module [`ModuleStorage`] that keeps each app's eframe settings in
//! the file it always used, so preferences carry over from the standalone apps.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
#![forbid(unsafe_code)]

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

pub use eframe;
pub use egui;

/// The interface language Vulcanio shows, and asks every module to show.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Lang {
    #[default]
    #[serde(rename = "en")]
    En,
    #[serde(rename = "vi")]
    Vi,
}

impl Lang {
    pub const ALL: [Self; 2] = [Self::En, Self::Vi];

    /// BCP 47 code.
    pub const fn code(self) -> &'static str {
        match self {
            Self::En => "en",
            Self::Vi => "vi",
        }
    }

    /// The language's own name.
    pub const fn name(self) -> &'static str {
        match self {
            Self::En => "English",
            Self::Vi => "Tiếng Việt",
        }
    }

    /// The language of the system locale, English unless it is Vietnamese.
    pub fn from_locale(locale: &str) -> Self {
        if locale.to_ascii_lowercase().starts_with("vi") { Self::Vi } else { Self::En }
    }

    /// Pick the English or Vietnamese text.
    pub fn pick<'a>(self, en: &'a str, vi: &'a str) -> &'a str {
        match self {
            Self::En => en,
            Self::Vi => vi,
        }
    }
}

/// Everything a module gets when it is created.
pub struct HostContext<'a> {
    /// The creation context for the module: the shared egui context, the window's wgpu state and
    /// window handles, and the module's own storage.
    pub cc: &'a eframe::CreationContext<'a>,
    /// Files to open right away.
    pub files: Vec<PathBuf>,
    /// The interface language chosen in Vulcanio.
    pub lang: Lang,
    /// The present mode of the window (vsync or low latency), for modules that cap frames.
    pub present_mode: eframe::wgpu::PresentMode,
    /// The depth/stencil format of the window's render pass (one window serves every module, and
    /// CAD needs a stencil buffer). Pipelines that draw inside that pass must declare it.
    pub depth_format: Option<eframe::wgpu::TextureFormat>,
}

/// One embedded app. The app keeps its own `eframe::App` implementation (its frame logic, UI,
/// input hook, saving and exit); Vulcanio adds what the shell needs on top.
pub trait Module {
    /// The app as eframe drives it.
    fn app(&mut self) -> &mut dyn eframe::App;

    /// Open documents (or import photos, for the photo library).
    fn open_paths(&mut self, ctx: &egui::Context, paths: &[PathBuf]);

    /// Start a new document the way the app's own File › New does.
    fn new_document(&mut self, ctx: &egui::Context);

    /// Show the interface in this language (the module's own preference follows).
    fn set_language(&mut self, ctx: &egui::Context, lang: Lang);

    /// Whether a document has unsaved changes (shown as a dot on the rail).
    fn has_unsaved_changes(&self) -> bool {
        false
    }

    /// Follow Vulcanio's theme: every app light, or every app dark.
    fn set_theme(&mut self, ctx: &egui::Context, dark: bool) {
        let _ = (ctx, dark);
    }

    /// Nothing open and nothing to lose: Vulcanio may close the app to give its memory back while
    /// it is in the background (it starts again in moments when it is wanted).
    fn is_idle(&self) -> bool {
        false
    }
}

/// eframe's key-value storage for one module, kept in a RON map file like eframe's own
/// `app.ron`, at the path the standalone app used.
pub struct ModuleStorage {
    path: Option<PathBuf>,
    kv: BTreeMap<String, String>,
    dirty: bool,
}

impl ModuleStorage {
    /// Reads `path` if it exists; a missing or unreadable file starts empty (as eframe does).
    pub fn open(path: Option<PathBuf>) -> Self {
        let kv = path
            .as_deref()
            .and_then(|p| std::fs::read_to_string(p).ok())
            .and_then(|text| ron::from_str::<BTreeMap<String, String>>(&text).ok())
            .unwrap_or_default();
        Self { path, kv, dirty: false }
    }

    /// eframe's storage file for an app id (`<data dir>/<app id>/app.ron`).
    pub fn for_app_id(app_id: &str) -> Self {
        Self::open(eframe::storage_dir(app_id).map(|d| d.join("app.ron")))
    }

    pub fn path(&self) -> Option<&Path> {
        self.path.as_deref()
    }
}

impl eframe::Storage for ModuleStorage {
    fn get_string(&self, key: &str) -> Option<String> {
        self.kv.get(key).cloned()
    }

    fn set_string(&mut self, key: &str, value: String) {
        if self.kv.get(key) != Some(&value) {
            self.kv.insert(key.to_owned(), value);
            self.dirty = true;
        }
    }

    fn remove_string(&mut self, key: &str) {
        if self.kv.remove(key).is_some() {
            self.dirty = true;
        }
    }

    fn flush(&mut self) {
        if !self.dirty {
            return;
        }
        let Some(path) = &self.path else { return };
        if let Some(dir) = path.parent()
            && let Err(e) = std::fs::create_dir_all(dir)
        {
            log::warn!("settings folder {}: {e}", dir.display());
            return;
        }
        match ron::ser::to_string_pretty(&self.kv, ron::ser::PrettyConfig::default()) {
            Ok(text) => {
                // Written beside and renamed, so a crash never leaves half a file.
                let tmp = path.with_extension("ron.tmp");
                match std::fs::write(&tmp, text).and_then(|()| std::fs::rename(&tmp, path)) {
                    Ok(()) => self.dirty = false,
                    Err(e) => log::warn!("settings {}: {e}", path.display()),
                }
            }
            Err(e) => log::warn!("settings {}: {e}", path.display()),
        }
    }
}

/// The lower-case extension of a path, without the dot.
pub fn extension(path: &Path) -> Option<String> {
    path.extension().map(|e| e.to_string_lossy().to_ascii_lowercase())
}
