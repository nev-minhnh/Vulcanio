//! Vulcanio's own preferences, kept in its eframe storage.

use std::collections::BTreeMap;
use std::path::PathBuf;

use vulcanio_host::Lang;
use serde::{Deserialize, Serialize};

use crate::modules::ModuleId;

pub const KEY: &str = "vulcanio.settings";

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThemeChoice {
    #[default]
    System,
    Light,
    Dark,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Recent {
    pub path: PathBuf,
    pub module: ModuleId,
}

impl Default for Recent {
    fn default() -> Self {
        Self { path: PathBuf::new(), module: ModuleId::Photo }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    /// `None`: follow the system language.
    pub lang: Option<Lang>,
    pub theme: ThemeChoice,
    pub smooth_scrolling: bool,
    /// How long a wheel notch takes to reach 90% of its scroll or zoom.
    pub ease_ms: u32,
    pub animations: bool,
    /// Present without waiting for vsync (lower latency, may tear).
    pub low_latency: bool,
    /// Extension (lower case, no dot) to the module that opens it without asking.
    pub open_with: BTreeMap<String, ModuleId>,
    pub recent: Vec<Recent>,
    pub reopen_last: bool,
    pub last_module: Option<ModuleId>,
    /// The app rail tucks away while an app is open and slides in when the pointer reaches the
    /// window's left edge.
    pub rail_autohide: bool,
    /// Apps with nothing open close after a minute in the background, giving their memory back.
    pub free_idle_apps: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            lang: None,
            theme: ThemeChoice::System,
            smooth_scrolling: true,
            ease_ms: 160,
            animations: true,
            low_latency: false,
            open_with: BTreeMap::new(),
            recent: Vec::new(),
            reopen_last: false,
            last_module: None,
            rail_autohide: true,
            free_idle_apps: true,
        }
    }
}

impl Settings {
    pub fn load(storage: Option<&dyn eframe::Storage>) -> Self {
        storage.and_then(|s| s.get_string(KEY)).and_then(|j| serde_json::from_str(&j).ok()).unwrap_or_default()
    }

    /// Read before the window exists (the present mode is chosen then).
    pub fn load_early() -> Self {
        let path = eframe::storage_dir(crate::APP_ID).map(|d| d.join("app.ron"));
        let kv = path
            .and_then(|p| std::fs::read_to_string(p).ok())
            .and_then(|t| ron::from_str::<std::collections::HashMap<String, String>>(&t).ok());
        kv.and_then(|kv| kv.get(KEY).and_then(|j| serde_json::from_str(j).ok())).unwrap_or_default()
    }

    pub fn save(&self, storage: &mut dyn eframe::Storage) {
        if let Ok(j) = serde_json::to_string(self) {
            storage.set_string(KEY, j);
        }
    }

    /// The language shown: the chosen one, or the system's.
    pub fn resolved_lang(&self) -> Lang {
        self.lang.unwrap_or_else(|| sys_locale::get_locale().map(|l| Lang::from_locale(&l)).unwrap_or_default())
    }

    pub fn remember(&mut self, path: PathBuf, module: ModuleId) {
        self.recent.retain(|r| r.path != path);
        self.recent.insert(0, Recent { path, module });
        self.recent.truncate(24);
    }

    pub fn ease_seconds(&self) -> f32 {
        if self.smooth_scrolling { self.ease_ms.clamp(40, 400) as f32 / 1000.0 } else { 0.001 }
    }
}
