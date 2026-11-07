//! The Krasis window: the app rail, the home screen, and the six apps hosted one at a time in
//! the space beside the rail.
//!
//! Every hosted app keeps its own frame logic, UI, input hook and save/exit code (its
//! `eframe::App`); only the app in front runs, as a sleeping tab does in Edge. Each app keeps its
//! own look: its styles, theme, zoom and fonts are put back when it comes to the front.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::mpsc::Receiver;

use egui::{Align2, Color32, CornerRadius, FontId, Id, LayerId, Order, Rect, Sense, Stroke, Style, ThemePreference, ViewportCommand, pos2, vec2};
use krasis_host::{HostContext, Lang, Module, ModuleStorage};

use crate::i18n::{tr, trf};
use crate::icons::{self, Icon};
use crate::instance::Handoff;
use crate::modules::{self, ModuleId, SPECS};
use crate::settings::{Settings, ThemeChoice};

pub const RAIL_WIDTH: f32 = 72.0;
const TRANSITION_SECONDS: f32 = 0.22;

/// Krasis' colours, from the logo.
#[derive(Clone, Copy)]
pub struct Palette {
    pub dark: bool,
    pub teal: Color32,
    pub teal_deep: Color32,
    pub cream: Color32,
    pub gold: Color32,
    pub bg: Color32,
    pub card: Color32,
    pub card_hover: Color32,
    pub text: Color32,
    pub muted: Color32,
    pub line: Color32,
}

impl Palette {
    pub fn new(dark: bool) -> Self {
        let teal = Color32::from_rgb(0x06, 0x60, 0x5F);
        let cream = Color32::from_rgb(0xFA, 0xF5, 0xE8);
        let gold = Color32::from_rgb(0xE9, 0xC2, 0x6E);
        if dark {
            Self {
                dark,
                teal,
                teal_deep: Color32::from_rgb(0x03, 0x33, 0x33),
                cream,
                gold,
                bg: Color32::from_rgb(0x0D, 0x17, 0x17),
                card: Color32::from_rgb(0x15, 0x24, 0x24),
                card_hover: Color32::from_rgb(0x1B, 0x2F, 0x2F),
                text: Color32::from_rgb(0xEC, 0xF2, 0xEF),
                muted: Color32::from_rgb(0x9A, 0xAE, 0xAA),
                line: Color32::from_rgb(0x24, 0x38, 0x38),
            }
        } else {
            Self {
                dark,
                teal,
                teal_deep: Color32::from_rgb(0x04, 0x47, 0x46),
                cream,
                gold,
                bg: Color32::from_rgb(0xF6, 0xF2, 0xE9),
                card: Color32::from_rgb(0xFF, 0xFE, 0xFB),
                card_hover: Color32::WHITE,
                text: Color32::from_rgb(0x16, 0x26, 0x25),
                muted: Color32::from_rgb(0x5D, 0x6E, 0x6B),
                line: Color32::from_rgb(0xE4, 0xDD, 0xCE),
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum View {
    Home,
    Module(ModuleId),
}

/// What an app left in the shared egui context: put back when it comes to the front.
#[derive(Clone)]
struct Look {
    dark: Arc<Style>,
    light: Arc<Style>,
    theme: ThemePreference,
    zoom: f32,
    fonts: egui::FontDefinitions,
}

fn capture(ctx: &egui::Context) -> Look {
    let (dark, light, theme, zoom) = ctx.options(|o| (o.dark_style.clone(), o.light_style.clone(), o.theme_preference, o.zoom_factor));
    let fonts = ctx.effective_font_definitions();
    Look { dark, light, theme, zoom, fonts }
}

fn same_fonts(a: &egui::FontDefinitions, b: &egui::FontDefinitions) -> bool {
    a.families == b.families && a.font_data.len() == b.font_data.len() && a.font_data.iter().zip(b.font_data.iter()).all(|((ka, va), (kb, vb))| ka == kb && Arc::ptr_eq(va, vb))
}

fn apply(ctx: &egui::Context, look: &Look) {
    ctx.options_mut(|o| {
        o.dark_style = look.dark.clone();
        o.light_style = look.light.clone();
        o.theme_preference = look.theme;
    });
    if (ctx.zoom_factor() - look.zoom).abs() > f32::EPSILON {
        ctx.set_zoom_factor(look.zoom);
    }
    let current = ctx.effective_font_definitions();
    if !same_fonts(&current, &look.fonts) {
        ctx.set_fonts(look.fonts.clone());
    }
}

/// egui's own defaults, for an app about to be created (it sets its look from there, as in its
/// own window).
fn fresh_look() -> Look {
    Look {
        dark: Arc::new(egui::Theme::Dark.default_style()),
        light: Arc::new(egui::Theme::Light.default_style()),
        theme: ThemePreference::System,
        zoom: 1.0,
        fonts: egui::FontDefinitions::default(),
    }
}

struct Slot {
    module: Option<Box<dyn Module>>,
    storage: ModuleStorage,
    look: Option<Look>,
    /// The window title the app last asked for.
    title: Option<String>,
    /// When the app went to the background (it may be closed to free memory when idle).
    left_at: Option<f64>,
}

/// Starting an app: shown for a frame first so the click answers at once.
struct Loading {
    id: ModuleId,
    files: Vec<PathBuf>,
    new_document: bool,
    shown: u8,
}

/// Choosing which app opens a file.
pub struct OpenWith {
    pub files: Vec<PathBuf>,
    pub ext: String,
    pub candidates: Vec<ModuleId>,
    pub remember: bool,
}

/// Closing apps one by one, each asking about unsaved work first.
struct Closing {
    queue: Vec<ModuleId>,
    quit_after: bool,
    /// A close request was handed to the app in front this frame.
    asked: bool,
}

pub struct Krasis {
    pub settings: Settings,
    pub lang: Lang,
    slots: Vec<Slot>,
    pub view: View,
    shell_look: Look,
    transition_start: f64,
    loading: Option<Loading>,
    pub open_with: Option<OpenWith>,
    inbox: Option<Receiver<Handoff>>,
    closing: Option<Closing>,
    inject_close: bool,
    exiting: bool,
    title: String,
    pub show_settings: bool,
    pub show_about: bool,
    pub logo: Option<egui::TextureHandle>,
    pub mark: Option<egui::TextureHandle>,
    present_mode: eframe::wgpu::PresentMode,
    pub notice: Option<(String, f64)>,
    pub assoc_note: Option<String>,
    pub palette: Palette,
    startup_files: Vec<PathBuf>,
    startup_module: Option<ModuleId>,
    /// Apps asked for while another was starting: each starts in turn, nothing is dropped.
    queue: Vec<Loading>,
    /// The tucked-away rail is showing.
    rail_open: bool,
    /// The theme last given to the apps (`true`: dark).
    theme_applied: Option<bool>,
    /// Since when the window has been minimized or in the background.
    unfocused_since: Option<f64>,
    trimmed: bool,
}

fn load_texture(ctx: &egui::Context, name: &str, png: &[u8]) -> Option<egui::TextureHandle> {
    let img = image::load_from_memory(png).ok()?.to_rgba8();
    let size = [img.width() as usize, img.height() as usize];
    let color = egui::ColorImage::from_rgba_unmultiplied(size, img.as_raw());
    Some(ctx.load_texture(name, color, egui::TextureOptions::LINEAR))
}

/// Inter for Krasis' own text: every Vietnamese letter (egui's built-in font lacks many), with
/// egui's fonts behind it for symbols and emoji.
fn install_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert("Inter".into(), Arc::new(egui::FontData::from_static(include_bytes!("../assets/fonts/Inter-Regular.ttf"))));
    fonts.font_data.insert("Inter SemiBold".into(), Arc::new(egui::FontData::from_static(include_bytes!("../assets/fonts/Inter-SemiBold.ttf"))));
    if let Some(list) = fonts.families.get_mut(&egui::FontFamily::Proportional) {
        list.insert(0, "Inter".into());
    }
    if let Some(list) = fonts.families.get_mut(&egui::FontFamily::Monospace) {
        list.push("Inter".into());
    }
    let mut strong = vec!["Inter SemiBold".to_string()];
    strong.extend(fonts.families.get(&egui::FontFamily::Proportional).cloned().unwrap_or_default());
    fonts.families.insert(egui::FontFamily::Name("krasis-strong".into()), strong);
    ctx.set_fonts(fonts);
}

/// Krasis' heading font, or the proportional one in a frame where an app's fonts are still
/// loaded (a font change takes effect on the next frame).
pub fn strong(ctx: &egui::Context, size: f32) -> FontId {
    let family = egui::FontFamily::Name("krasis-strong".into());
    if ctx.fonts(|f| f.definitions().families.contains_key(&family)) { FontId::new(size, family) } else { FontId::proportional(size) }
}

impl Krasis {
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        files: Vec<PathBuf>,
        module: Option<ModuleId>,
        inbox: Option<Receiver<Handoff>>,
        present_mode: eframe::wgpu::PresentMode,
    ) -> Self {
        let settings = Settings::load(cc.storage);
        let lang = settings.resolved_lang();
        let ctx = &cc.egui_ctx;
        crate::set_wake_context(ctx.clone());
        egui::set_wheel_smoothing_seconds(settings.ease_seconds());
        let palette = Palette::new(false);
        let mut me = Self {
            lang,
            slots: SPECS.iter().map(|s| Slot { module: None, storage: (s.storage)(), look: None, title: None, left_at: None }).collect(),
            view: View::Home,
            shell_look: fresh_look(),
            transition_start: 0.0,
            loading: None,
            open_with: None,
            inbox,
            closing: None,
            inject_close: false,
            exiting: false,
            title: String::new(),
            show_settings: false,
            show_about: false,
            logo: load_texture(ctx, "krasis-logo", include_bytes!("../assets/krasis-256.png")),
            mark: load_texture(ctx, "krasis-mark", include_bytes!("../assets/krasis-mark.png")),
            present_mode,
            notice: None,
            assoc_note: None,
            palette,
            startup_files: files,
            startup_module: module,
            queue: Vec::new(),
            rail_open: false,
            theme_applied: None,
            unfocused_since: None,
            trimmed: false,
            settings,
        };
        install_fonts(ctx);
        me.style_shell(ctx);
        me.shell_look = capture(ctx);
        me
    }

    /// Krasis' own look for the home screen and its dialogs.
    fn style_shell(&mut self, ctx: &egui::Context) {
        let pref = match self.settings.theme {
            ThemeChoice::System => ThemePreference::System,
            ThemeChoice::Light => ThemePreference::Light,
            ThemeChoice::Dark => ThemePreference::Dark,
        };
        ctx.set_theme(pref);
        let anim = if self.settings.animations { 0.16 } else { 0.0 };
        ctx.all_styles_mut(|s| {
            s.animation_time = anim;
            s.visuals.selection.bg_fill = Color32::from_rgb(0x06, 0x60, 0x5F);
            s.visuals.hyperlink_color = Color32::from_rgb(0x0A, 0x86, 0x84);
            s.spacing.button_padding = vec2(12.0, 6.0);
            s.visuals.widgets.inactive.corner_radius = CornerRadius::same(8);
            s.visuals.widgets.hovered.corner_radius = CornerRadius::same(8);
            s.visuals.widgets.active.corner_radius = CornerRadius::same(8);
            s.visuals.window_corner_radius = CornerRadius::same(14);
        });
    }

    fn dark_now(&self, ctx: &egui::Context) -> bool {
        match self.settings.theme {
            ThemeChoice::Dark => true,
            ThemeChoice::Light => false,
            ThemeChoice::System => ctx.system_theme().is_none_or(|t| t == egui::Theme::Dark) && ctx.system_theme().is_some(),
        }
    }

    pub fn spec_loaded(&self, id: ModuleId) -> bool {
        self.slots[id.index()].module.is_some()
    }

    pub fn has_unsaved(&self, id: ModuleId) -> bool {
        self.slots[id.index()].module.as_ref().is_some_and(|m| m.has_unsaved_changes())
    }

    // ---- routing -------------------------------------------------------------------------

    /// Start an app (one at a time; a request for an app already starting joins it).
    fn request(&mut self, id: ModuleId, files: Vec<PathBuf>, new_document: bool) {
        if let Some(l) = self.loading.as_mut().filter(|l| l.id == id) {
            l.files.extend(files);
            l.new_document |= new_document;
        } else if let Some(q) = self.queue.iter_mut().find(|q| q.id == id) {
            q.files.extend(files);
            q.new_document |= new_document;
        } else if self.loading.is_none() {
            self.loading = Some(Loading { id, files, new_document, shown: 0 });
        } else {
            self.queue.push(Loading { id, files, new_document, shown: 0 });
        }
    }

    /// Bring an app to the front, starting it if it isn't running.
    pub fn activate(&mut self, ctx: &egui::Context, id: ModuleId) {
        self.cancel_closing_if_user_moved(View::Module(id));
        if self.spec_loaded(id) {
            self.switch(ctx, View::Module(id));
        } else {
            self.request(id, Vec::new(), false);
        }
    }

    pub fn go_home(&mut self, ctx: &egui::Context) {
        self.cancel_closing_if_user_moved(View::Home);
        self.switch(ctx, View::Home);
    }

    /// File › New in an app.
    pub fn new_in(&mut self, ctx: &egui::Context, id: ModuleId) {
        if self.spec_loaded(id) {
            self.switch(ctx, View::Module(id));
            if let Some(m) = self.slots[id.index()].module.as_mut() {
                m.new_document(ctx);
            }
        } else {
            self.request(id, Vec::new(), true);
        }
    }

    /// Open files in an app.
    pub fn open_in(&mut self, ctx: &egui::Context, id: ModuleId, files: Vec<PathBuf>) {
        for f in &files {
            self.settings.remember(f.clone(), id);
        }
        if self.spec_loaded(id) {
            self.switch(ctx, View::Module(id));
            if let Some(m) = self.slots[id.index()].module.as_mut() {
                m.open_paths(ctx, &files);
            }
        } else {
            self.request(id, files, false);
        }
    }

    /// Open files in the app that suits them, asking when more than one does.
    pub fn route(&mut self, ctx: &egui::Context, files: Vec<PathBuf>) {
        // Group by extension: one question per kind of file.
        let mut groups: Vec<(String, Vec<PathBuf>)> = Vec::new();
        for f in files {
            if !f.exists() {
                let name = f.display().to_string();
                self.notify(ctx, trf(self.lang, "Couldn't open {file}: {reason}", &[("file", &name), ("reason", tr(self.lang, "the file does not exist"))]));
                continue;
            }
            let ext = krasis_host::extension(&f).unwrap_or_default();
            match groups.iter_mut().find(|(e, _)| *e == ext) {
                Some((_, v)) => v.push(f),
                None => groups.push((ext, vec![f])),
            }
        }
        for (ext, files) in groups {
            let candidates = modules::candidates(&ext);
            if let Some(id) = self.settings.open_with.get(&ext).copied().filter(|id| candidates.contains(id)) {
                self.open_in(ctx, id, files);
                continue;
            }
            match candidates.as_slice() {
                [] => self.notify(ctx, trf(self.lang, "No app in Krasis opens .{ext} files.", &[("ext", &ext)])),
                [only] => self.open_in(ctx, *only, files),
                // The natural home of the type opens it; the app in front keeps a type it also opens.
                [first, ..] => {
                    let natural = SPECS[first.index()].primary.contains(&ext.as_str());
                    if natural && candidates.iter().filter(|c| SPECS[c.index()].primary.contains(&ext.as_str())).count() == 1 {
                        self.open_in(ctx, *first, files);
                    } else if let View::Module(front) = self.view
                        && candidates.contains(&front)
                        && self.open_with.is_none()
                    {
                        self.open_in(ctx, front, files);
                    } else {
                        self.open_with = Some(OpenWith { files, ext, candidates, remember: false });
                    }
                }
            }
        }
    }

    pub fn pick_and_open(&mut self, ctx: &egui::Context, only: Option<ModuleId>) {
        let exts: Vec<&str> = match only {
            Some(id) => SPECS[id.index()].opens.to_vec(),
            None => modules::all_extensions(),
        };
        let mut dialog = rfd::FileDialog::new().add_filter(tr(self.lang, "All supported files"), &exts);
        if only.is_none() {
            for s in &SPECS {
                dialog = dialog.add_filter(s.title(self.lang), s.opens);
            }
        }
        if let Some(files) = dialog.add_filter(tr(self.lang, "All files"), &["*"]).pick_files() {
            match only {
                Some(id) => self.open_in(ctx, id, files),
                None => self.route(ctx, files),
            }
        }
    }

    pub fn notify(&mut self, ctx: &egui::Context, text: String) {
        log::info!("{text}");
        self.notice = Some((text, ctx.input(|i| i.time)));
    }

    // ---- switching -----------------------------------------------------------------------

    fn switch(&mut self, ctx: &egui::Context, to: View) {
        if self.view == to {
            return;
        }
        let look = capture(ctx);
        let now = ctx.input(|i| i.time);
        match self.view {
            View::Home => self.shell_look = look,
            View::Module(m) => {
                self.slots[m.index()].look = Some(look);
                self.slots[m.index()].left_at = Some(now);
            }
        }
        if let View::Module(m) = to {
            self.slots[m.index()].left_at = None;
        }
        self.view = to;
        match to {
            View::Home => apply(ctx, &self.shell_look.clone()),
            View::Module(m) => {
                let look = self.slots[m.index()].look.clone().unwrap_or_else(fresh_look);
                apply(ctx, &look);
                self.settings.last_module = Some(m);
            }
        }
        crate::set_front(match to {
            View::Home => None,
            View::Module(m) => Some(m),
        });
        self.transition_start = ctx.input(|i| i.time);
        ctx.request_repaint();
    }

    fn create(&mut self, ctx: &egui::Context, frame: &eframe::Frame, id: ModuleId, files: Vec<PathBuf>) {
        let started = std::time::Instant::now();
        // The app starts from egui's defaults, as in its own window.
        self.switch(ctx, View::Module(id));
        let spec = &SPECS[id.index()];
        let module = {
            let storage: &dyn eframe::Storage = &self.slots[id.index()].storage;
            let cc = eframe::CreationContext::for_embedded(ctx.clone(), frame, Some(storage));
            let host = HostContext { cc: &cc, files, lang: self.lang, present_mode: self.present_mode };
            let guarded = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| (spec.create)(&host)));
            match guarded {
                Ok(m) => m,
                Err(_) => {
                    log::error!("{} could not start", spec.title(Lang::En));
                    self.switch(ctx, View::Home);
                    self.notify(ctx, format!("{} could not start. See the log in {}.", spec.title(self.lang), crate::data_dir().map(|d| d.join("logs").display().to_string()).unwrap_or_default()));
                    return;
                }
            }
        };
        log::info!("{} started in {:.0} ms", spec.title(Lang::En), started.elapsed().as_secs_f64() * 1000.0);
        self.slots[id.index()].module = Some(module);
        if let Some(m) = self.slots[id.index()].module.as_mut() {
            m.set_theme(ctx, self.theme_applied.unwrap_or(true));
        }
        ctx.request_repaint();
    }

    /// Close an app (it asks about unsaved work first), or every app before quitting.
    pub fn close_module(&mut self, ctx: &egui::Context, id: ModuleId) {
        if self.spec_loaded(id) {
            self.closing = Some(Closing { queue: vec![id], quit_after: false, asked: false });
            self.switch(ctx, View::Module(id));
            self.inject_close = true;
            ctx.request_repaint();
        }
    }

    fn unload(&mut self, ctx: &egui::Context, id: ModuleId) {
        let slot = &mut self.slots[id.index()];
        if let Some(mut m) = slot.module.take() {
            m.app().save(&mut slot.storage);
            eframe::Storage::flush(&mut slot.storage);
            m.app().on_exit();
            log::info!("{} closed", SPECS[id.index()].title(Lang::En));
        }
        slot.look = None;
        slot.title = None;
        if self.view == View::Module(id) {
            // The closed app's look goes with it; home starts from Krasis' own.
            self.view = View::Home;
            apply(ctx, &self.shell_look.clone());
            crate::set_front(None);
            self.transition_start = ctx.input(|i| i.time);
        }
    }

    fn cancel_closing_if_user_moved(&mut self, to: View) {
        if let Some(c) = &self.closing
            && c.queue.first().map(|m| View::Module(*m)) != Some(to)
        {
            self.closing = None;
        }
    }

    fn loaded(&self) -> Vec<ModuleId> {
        ModuleId::ALL.into_iter().filter(|m| self.spec_loaded(*m)).collect()
    }

    /// Close apps with nothing open after a minute in the background, and page memory out
    /// while Krasis is minimized or in the background (as Edge does with sleeping tabs).
    fn release_memory(&mut self, ctx: &egui::Context) {
        let now = ctx.input(|i| i.time);
        if self.settings.free_idle_apps && self.closing.is_none() {
            let mut freed = false;
            let mut waiting = false;
            for id in ModuleId::ALL {
                let slot = &self.slots[id.index()];
                let Some(left) = slot.left_at else { continue };
                let Some(module) = &slot.module else { continue };
                if self.view == View::Module(id) || !module.is_idle() {
                    continue;
                }
                if now - left >= 60.0 {
                    self.unload(ctx, id);
                    freed = true;
                } else {
                    waiting = true;
                }
            }
            if freed {
                crate::memory::compact();
            }
            if waiting {
                ctx.request_repaint_after(std::time::Duration::from_secs(10));
            }
        }
        let (focused, minimized) = ctx.input(|i| (i.viewport().focused.unwrap_or(true), i.viewport().minimized.unwrap_or(false)));
        if focused && !minimized {
            self.unfocused_since = None;
            self.trimmed = false;
        } else {
            let since = *self.unfocused_since.get_or_insert(now);
            if !self.trimmed {
                if now - since >= 8.0 || minimized {
                    crate::memory::trim_working_set();
                    self.trimmed = true;
                } else {
                    ctx.request_repaint_after(std::time::Duration::from_secs(9));
                }
            }
        }
    }

    // ---- per frame -----------------------------------------------------------------------

    fn poll_inbox(&mut self, ctx: &egui::Context) {
        let mut handoffs = Vec::new();
        if let Some(rx) = &self.inbox {
            while let Ok(h) = rx.try_recv() {
                handoffs.push(h);
            }
        }
        for h in handoffs {
            ctx.send_viewport_cmd(ViewportCommand::Minimized(false));
            ctx.send_viewport_cmd(ViewportCommand::Focus);
            let module = h.module.as_deref().and_then(ModuleId::from_key);
            match (module, h.files.is_empty()) {
                (Some(id), true) => self.activate(ctx, id),
                (Some(id), false) => self.open_in(ctx, id, h.files),
                (None, false) => self.route(ctx, h.files),
                (None, true) => {}
            }
        }
    }

    fn first_frame(&mut self, ctx: &egui::Context) {
        let files = std::mem::take(&mut self.startup_files);
        let module = self.startup_module.take();
        match (module, files.is_empty()) {
            (Some(id), true) => self.activate(ctx, id),
            (Some(id), false) => self.open_in(ctx, id, files),
            (None, false) => self.route(ctx, files),
            (None, true) => {
                if self.settings.reopen_last
                    && let Some(id) = self.settings.last_module
                {
                    self.activate(ctx, id);
                }
            }
        }
    }

    /// Commands the app in front gave the window this frame: its title becomes Krasis' title, a
    /// Close ends the app (or the next step of quitting), and the rest go through.
    fn filter_commands(&mut self, ctx: &egui::Context, front: Option<ModuleId>) {
        let mut title = None;
        let mut close = false;
        let mut cancel = false;
        ctx.retain_viewport_commands(|c| match c {
            ViewportCommand::Title(t) => {
                title = Some(t.clone());
                false
            }
            ViewportCommand::Close => {
                close = true;
                false
            }
            ViewportCommand::CancelClose => {
                cancel = true;
                false
            }
            // Krasis keeps the system frame; an app's own title bar is off.
            ViewportCommand::Decorations(_) => false,
            _ => true,
        });
        if let (Some(t), Some(m)) = (title, front) {
            self.slots[m.index()].title = Some(t);
        }
        let close_asked = ctx.input(|i| i.viewport().close_requested());
        let mut keep_open = false;
        match (&mut self.closing, front) {
            (Some(closing), Some(m)) if closing.queue.first() == Some(&m) => {
                // The app accepted closing: it sent Close, or got the request and didn't cancel.
                let accepted = close || (close_asked && !cancel);
                if accepted {
                    let quit_after = closing.quit_after;
                    closing.queue.remove(0);
                    let next = closing.queue.first().copied();
                    self.unload(ctx, m);
                    match next {
                        Some(n) => {
                            self.switch(ctx, View::Module(n));
                            self.inject_close = true;
                            keep_open = true;
                        }
                        None => {
                            self.closing = None;
                            if quit_after {
                                self.exiting = true;
                                if !close_asked {
                                    ctx.send_viewport_cmd(ViewportCommand::Close);
                                }
                            } else {
                                keep_open = close_asked;
                            }
                        }
                    }
                } else {
                    // Its unsaved-changes question is up.
                    keep_open = true;
                }
            }
            (Some(_), None) => keep_open = true,
            (None, Some(m)) if close && !close_asked => {
                // File › Exit inside an app: that app closes, Krasis stays.
                self.unload(ctx, m);
            }
            (None, Some(_)) if cancel => keep_open = true,
            _ => {}
        }
        if keep_open && !self.exiting {
            ctx.send_viewport_cmd(ViewportCommand::CancelClose);
        }
    }

    fn update_title(&mut self, ctx: &egui::Context) {
        let title = match self.view {
            View::Home => "Krasis".to_string(),
            View::Module(m) => match &self.slots[m.index()].title {
                Some(t) if !t.is_empty() => format!("{t} · Krasis"),
                _ => format!("{} · Krasis", SPECS[m.index()].title(self.lang)),
            },
        };
        if title != self.title {
            ctx.send_viewport_cmd(ViewportCommand::Title(title.clone()));
            self.title = title;
        }
    }

    // ---- drawing -------------------------------------------------------------------------

    fn rail_fill(&self) -> Color32 {
        if self.palette.dark { self.palette.teal_deep } else { self.palette.teal }
    }

    /// The app rail, pinned beside the content.
    fn rail(&mut self, ui: &mut egui::Ui) {
        let fill = self.rail_fill();
        egui::Panel::left("krasis_rail").exact_size(RAIL_WIDTH).resizable(false).frame(egui::Frame::NONE.fill(fill)).show(ui, |ui| self.rail_body(ui));
    }

    /// The app rail tucked away: it slides in over the app when the pointer reaches the window's
    /// left edge, and slides out when the pointer leaves it.
    fn rail_overlay(&mut self, ui: &mut egui::Ui) {
        let ctx = ui.ctx().clone();
        let window = ui.max_rect();
        let pointer = ctx.input(|i| i.pointer.latest_pos());
        let at_edge = pointer.is_some_and(|p| p.x <= window.left() + 4.0 && p.y >= window.top() && p.y <= window.bottom());
        let over = pointer.is_some_and(|p| p.x <= window.left() + RAIL_WIDTH + 12.0);
        let menu_open = egui::Popup::is_any_open(&ctx);
        self.rail_open = at_edge || (self.rail_open && (over || menu_open));
        let time = if self.settings.animations { 0.18 } else { 0.0 };
        let t = ctx.animate_bool_with_time(Id::unique("krasis_rail_reveal"), self.rail_open, time);
        if t <= 0.0 {
            // A hairline hint where the rail waits.
            let painter = ctx.layer_painter(LayerId::new(Order::Foreground, Id::unique("krasis_rail_hint")));
            painter.rect_filled(Rect::from_min_size(window.min, vec2(2.0, window.height())), 0.0, self.palette.teal.gamma_multiply(0.55));
            return;
        }
        let ease = 1.0 - (1.0 - t).powi(3);
        let x = window.left() - RAIL_WIDTH * (1.0 - ease);
        let fill = self.rail_fill();
        egui::Area::new(Id::unique("krasis_rail_overlay")).order(Order::Foreground).fixed_pos(pos2(x, window.top())).show(&ctx, |ui| {
            let rect = Rect::from_min_size(pos2(x, window.top()), vec2(RAIL_WIDTH, window.height()));
            for k in 1..=6 {
                let shadow = Rect::from_min_size(rect.right_top(), vec2(k as f32 * 2.0, rect.height()));
                ui.painter().rect_filled(shadow, 0.0, Color32::from_black_alpha((26.0 * ease / k as f32) as u8));
            }
            ui.painter().rect_filled(rect, 0.0, fill);
            ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
                ui.set_min_size(rect.size());
                self.rail_body(ui);
            });
        });
    }

    fn rail_body(&mut self, ui: &mut egui::Ui) {
        let p = self.palette;
        let ctx = ui.ctx().clone();
        let rect = ui.max_rect();
        let painter = ui.painter().clone();
        // The mark at the top: home.
        let logo_rect = Rect::from_center_size(pos2(rect.center().x, rect.top() + 34.0), vec2(30.0, 34.0));
        let logo_resp = ui.interact(logo_rect.expand(8.0), Id::unique("krasis_logo"), Sense::click());
        if let Some(mark) = &self.mark {
            let grow = ctx.animate_bool_with_time(Id::unique("krasis_logo_hover"), logo_resp.hovered(), 0.15);
            painter.image(mark.id(), logo_rect.expand(grow * 2.0), Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)), Color32::WHITE);
        }
        if logo_resp.on_hover_text("Krasis").clicked() {
            self.go_home(&ctx);
        }
        let mut items: Vec<(View, Icon, String)> = vec![(View::Home, Icon::Home, tr(self.lang, "Home").to_string())];
        for s in &SPECS {
            items.push((View::Module(s.id), Icon::Module(s.id), s.short(self.lang).to_string()));
        }
        let item_h = 58.0;
        let top = rect.top() + 72.0;
        // The pill behind the app in front slides to it.
        let active_index = items.iter().position(|(v, _, _)| *v == self.view).unwrap_or(0) as f32;
        let loading_index = self.loading.as_ref().and_then(|l| items.iter().position(|(v, _, _)| *v == View::Module(l.id))).map(|i| i as f32);
        let target = loading_index.unwrap_or(active_index);
        let anim = if self.settings.animations { 0.22 } else { 0.0 };
        let y = ctx.animate_value_with_time(Id::unique("krasis_rail_pill"), target, anim);
        let pill = Rect::from_min_size(pos2(rect.left() + 8.0, top + y * item_h + 3.0), vec2(RAIL_WIDTH - 16.0, item_h - 6.0));
        painter.rect_filled(pill, 12.0, Color32::from_white_alpha(if p.dark { 26 } else { 34 }));
        let bar = Rect::from_min_size(pos2(rect.left(), pill.center().y - 12.0), vec2(3.5, 24.0));
        painter.rect_filled(bar, 2.0, p.gold);
        for (i, (view, icon, label)) in items.iter().enumerate() {
            let r = Rect::from_min_size(pos2(rect.left(), top + i as f32 * item_h), vec2(RAIL_WIDTH, item_h));
            let resp = ui.interact(r, Id::unique(("krasis_rail_item", i)), Sense::click());
            let hover = ctx.animate_bool_with_time(Id::unique(("krasis_rail_hover", i)), resp.hovered(), 0.12);
            if hover > 0.0 && *view != self.view {
                painter.rect_filled(pill.translate(vec2(0.0, (i as f32 - y) * item_h)), 12.0, Color32::from_white_alpha((14.0 * hover) as u8));
            }
            let active = *view == self.view;
            let color = if active { p.cream } else { Color32::from_rgba_unmultiplied(0xFA, 0xF5, 0xE8, (175.0 + 80.0 * hover) as u8) };
            let icon_rect = Rect::from_center_size(pos2(r.center().x, r.top() + 22.0 - hover), vec2(24.0, 24.0));
            icons::paint(&painter, icon_rect, icon, color);
            painter.text(pos2(r.center().x, r.top() + 44.0), Align2::CENTER_CENTER, label, FontId::proportional(10.5), color);
            if let View::Module(m) = view {
                if self.spec_loaded(*m) {
                    // Running: a small gold dot.
                    painter.circle_filled(pos2(r.right() - 16.0, r.top() + 10.0), 3.0, p.gold);
                }
                if self.loading.as_ref().is_some_and(|l| l.id == *m) {
                    let t = ctx.input(|i| i.time) as f32;
                    let a = t * 6.0;
                    let c = pos2(r.right() - 16.0, r.top() + 10.0);
                    painter.add(egui::Shape::line(
                        (0..10).map(|k| c + vec2((a + k as f32 * 0.45).cos(), (a + k as f32 * 0.45).sin()) * 5.0).collect(),
                        Stroke::new(1.5, p.gold),
                    ));
                    ctx.request_repaint();
                }
            }
            let tip = match view {
                View::Home => tr(self.lang, "Home").to_string(),
                View::Module(m) => SPECS[m.index()].title(self.lang).to_string(),
            };
            let resp = resp.on_hover_text(tip);
            if resp.clicked() {
                match view {
                    View::Home => self.go_home(&ctx),
                    View::Module(m) => self.activate(&ctx, *m),
                }
            }
            if let View::Module(m) = view {
                let m = *m;
                let loaded = self.spec_loaded(m);
                resp.context_menu(|ui| {
                    if ui.button(tr(self.lang, "Open…")).clicked() {
                        ui.close();
                        self.pick_and_open(&ctx, Some(m));
                    }
                    if ui.button(SPECS[m.index()].new_label(self.lang)).clicked() {
                        ui.close();
                        self.new_in(&ctx, m);
                    }
                    if loaded {
                        ui.separator();
                        if ui.button(tr(self.lang, "Close app")).clicked() {
                            ui.close();
                            self.close_module(&ctx, m);
                        }
                    }
                });
            }
        }
        // Tuck away (auto-hide), Settings and About at the bottom.
        let tuck = if self.settings.rail_autohide { tr(self.lang, "Keep the app bar open") } else { tr(self.lang, "Hide the app bar while an app is open") };
        let bottom = [(Icon::Sidebar, tuck, 2u8), (Icon::Settings, tr(self.lang, "Settings"), 0u8), (Icon::Info, tr(self.lang, "About Krasis"), 1u8)];
        for (k, (icon, label, which)) in bottom.iter().enumerate() {
            let r = Rect::from_min_size(pos2(rect.left(), rect.bottom() - 48.0 * (3 - k) as f32 - 6.0), vec2(RAIL_WIDTH, 46.0));
            let resp = ui.interact(r, Id::unique(("krasis_rail_bottom", k)), Sense::click());
            let hover = ctx.animate_bool_with_time(Id::unique(("krasis_rail_bottom_hover", k)), resp.hovered(), 0.12);
            let color = Color32::from_rgba_unmultiplied(0xFA, 0xF5, 0xE8, (165.0 + 90.0 * hover) as u8);
            icons::paint(&painter, Rect::from_center_size(r.center(), vec2(22.0, 22.0)), icon, color);
            if resp.on_hover_text(*label).clicked() {
                match which {
                    0 => self.show_settings = true,
                    1 => self.show_about = true,
                    _ => {
                        self.settings.rail_autohide = !self.settings.rail_autohide;
                        self.rail_open = false;
                    }
                }
            }
        }
    }

    fn loading_screen(&self, ui: &mut egui::Ui, id: ModuleId) {
        let p = self.palette;
        let rect = ui.available_rect_before_wrap();
        let painter = ui.painter();
        painter.rect_filled(rect, 0.0, p.bg);
        let spec = &SPECS[id.index()];
        let c = rect.center();
        icons::paint(painter, Rect::from_center_size(c - vec2(0.0, 28.0), vec2(56.0, 56.0)), &Icon::Module(id), spec.accent());
        painter.text(
            c + vec2(0.0, 30.0),
            Align2::CENTER_CENTER,
            trf(self.lang, "Starting {app}…", &[("app", spec.title(self.lang))]),
            FontId::proportional(16.0),
            p.muted,
        );
    }

    fn fade(&self, ctx: &egui::Context, content: Rect) {
        if !self.settings.animations {
            return;
        }
        let t = ((ctx.input(|i| i.time) - self.transition_start) as f32 / TRANSITION_SECONDS).clamp(0.0, 1.0);
        if t >= 1.0 {
            return;
        }
        // Ease out: quick to show, gentle to settle (Fluent's decelerate curve).
        let k = 1.0 - (1.0 - t).powi(3);
        let alpha = ((1.0 - k) * 255.0) as u8;
        let painter = ctx.layer_painter(LayerId::new(Order::Debug, Id::unique("krasis_fade")));
        let bg = self.palette.bg;
        painter.rect_filled(content, 0.0, Color32::from_rgba_unmultiplied(bg.r(), bg.g(), bg.b(), alpha));
        ctx.request_repaint();
    }

    fn notice_toast(&mut self, ctx: &egui::Context, content: Rect) {
        let Some((text, at)) = self.notice.clone() else { return };
        let age = ctx.input(|i| i.time) - at;
        if age > 6.0 {
            self.notice = None;
            return;
        }
        let a = (age.min(6.0 - age).min(0.25) / 0.25) as f32;
        let p = self.palette;
        let painter = ctx.layer_painter(LayerId::new(Order::Tooltip, Id::unique("krasis_toast")));
        let galley = painter.layout(text, FontId::proportional(14.0), p.cream, 520.0);
        let size = galley.size() + vec2(32.0, 20.0);
        let rect = Rect::from_center_size(pos2(content.center().x, content.bottom() - 40.0 - size.y / 2.0 + (1.0 - a) * 12.0), size);
        painter.rect_filled(rect, 12.0, Color32::from_rgba_unmultiplied(0x0B, 0x2E, 0x2D, (235.0 * a) as u8));
        painter.galley(rect.min + vec2(16.0, 10.0), galley, p.cream);
        ctx.request_repaint_after(std::time::Duration::from_millis(100));
    }

    fn closing_banner(&mut self, ctx: &egui::Context, content: Rect) {
        let Some(closing) = &self.closing else { return };
        if !closing.quit_after {
            return;
        }
        let mut cancel = false;
        egui::Area::new(Id::unique("krasis_closing"))
            .order(Order::Tooltip)
            .fixed_pos(pos2(content.center().x - 260.0, content.top() + 12.0))
            .show(ctx, |ui| {
                egui::Frame::NONE.fill(self.palette.teal).corner_radius(10.0).inner_margin(egui::Margin::symmetric(14, 8)).show(ui, |ui| {
                    ui.set_width(520.0);
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new(tr(self.lang, "Closing Krasis: each app asks about unsaved work first.")).color(self.palette.cream));
                        if ui.button(tr(self.lang, "Keep working")).clicked() {
                            cancel = true;
                        }
                    });
                });
            });
        if cancel {
            self.closing = None;
        }
    }
}

impl eframe::App for Krasis {
    fn logic(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if ctx.cumulative_frame_nr() == 0 {
            self.first_frame(ctx);
        }
        self.poll_inbox(ctx);
        // An app being started: shown for a frame, then created.
        if let Some(l) = &mut self.loading {
            if l.shown < 2 {
                l.shown += 1;
                ctx.request_repaint();
            } else if let Some(l) = self.loading.take() {
                self.create(ctx, frame, l.id, l.files);
                if l.new_document
                    && let Some(m) = self.slots[l.id.index()].module.as_mut()
                {
                    m.new_document(ctx);
                }
                if !self.queue.is_empty() {
                    self.loading = Some(self.queue.remove(0));
                }
            }
        }
        self.release_memory(ctx);
        if let View::Module(m) = self.view
            && let Some(module) = self.slots[m.index()].module.as_mut()
        {
            module.app().logic(ctx, frame);
        }
    }

    fn raw_input_hook(&mut self, ctx: &egui::Context, raw: &mut egui::RawInput) {
        let root = raw.viewports.entry(egui::ViewportId::ROOT).or_default();
        let user_close = root.events.contains(&egui::ViewportEvent::Close);
        if self.inject_close {
            self.inject_close = false;
            if !user_close {
                root.events.push(egui::ViewportEvent::Close);
            }
        } else if user_close && !self.exiting && self.closing.is_none() {
            let loaded = self.loaded();
            if !loaded.is_empty() {
                // The app in front first, then the others.
                let mut queue = loaded;
                if let View::Module(front) = self.view {
                    queue.retain(|m| *m != front);
                    queue.insert(0, front);
                }
                let first = queue[0];
                self.closing = Some(Closing { queue, quit_after: true, asked: false });
                if self.view != View::Module(first) {
                    // Home saw the request: the first app gets it next frame.
                    self.switch(ctx, View::Module(first));
                    root.events.retain(|e| *e != egui::ViewportEvent::Close);
                    self.inject_close = true;
                    ctx.send_viewport_cmd(ViewportCommand::CancelClose);
                }
            }
        }
        if let Some(c) = &mut self.closing {
            c.asked = raw.viewports.get(&egui::ViewportId::ROOT).is_some_and(|v| v.events.contains(&egui::ViewportEvent::Close));
        }
        // Dropped files: the app in front takes the kinds it opens; Krasis routes the rest.
        if !raw.dropped_files.is_empty() {
            let front = match self.view {
                View::Module(m) => Some(m),
                View::Home => None,
            };
            let mut others = Vec::new();
            raw.dropped_files.retain(|f| {
                let path = f.path().to_path_buf();
                let ext = krasis_host::extension(&path).unwrap_or_default();
                let keep = front.is_some_and(|m| SPECS[m.index()].opens_ext(&ext));
                if !keep {
                    others.push(path);
                }
                keep
            });
            if let Some(m) = front {
                for f in &raw.dropped_files {
                    self.settings.remember(f.path().to_path_buf(), m);
                }
            }
            if !others.is_empty() {
                self.route(ctx, others);
            }
        }
        if let View::Module(m) = self.view
            && let Some(module) = self.slots[m.index()].module.as_mut()
        {
            module.app().raw_input_hook(ctx, raw);
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        egui::set_wheel_smoothing_seconds(self.settings.ease_seconds());
        let lang = self.settings.resolved_lang();
        if lang != self.lang {
            self.lang = lang;
            for slot in &mut self.slots {
                if let Some(m) = slot.module.as_mut() {
                    m.set_language(&ctx, lang);
                }
            }
        }
        // One theme for Krasis and every app.
        let dark = self.dark_now(&ctx);
        if self.theme_applied != Some(dark) {
            self.theme_applied = Some(dark);
            self.palette = Palette::new(dark);
            for slot in &mut self.slots {
                if let Some(m) = slot.module.as_mut() {
                    m.set_theme(&ctx, dark);
                }
            }
        }
        let tucked = self.settings.rail_autohide && matches!(self.view, View::Module(_)) && self.loading.is_none();
        if tucked {
            self.rail_overlay(ui);
        } else {
            self.rail_open = false;
            self.rail(ui);
        }
        let content = ui.available_rect_before_wrap();
        let front = match self.view {
            View::Module(m) => Some(m),
            View::Home => None,
        };
        if let Some(l) = &self.loading {
            let id = l.id;
            self.loading_screen(ui, id);
        } else {
            match front {
                Some(m) => {
                    if let Some(module) = self.slots[m.index()].module.as_mut() {
                        let t = ((ctx.input(|i| i.time) - self.transition_start) as f32 / TRANSITION_SECONDS).clamp(0.0, 1.0);
                        let animate = self.settings.animations && t < 1.0;
                        // Each app in a scope of its own: two apps' panels with the same name keep
                        // their own sizes and state.
                        ui.scope_builder(egui::UiBuilder::new().id_salt(("krasis_module", m.key())), |ui| {
                            if animate {
                                // The app rises into place as it fades in.
                                let lift = (1.0 - (1.0 - (1.0 - t).powi(3))) * 14.0;
                                ui.with_visual_transform(egui::emath::TSTransform::from_translation(vec2(0.0, lift)), |ui| {
                                    module.app().ui(ui, frame);
                                });
                            } else {
                                module.app().ui(ui, frame);
                            }
                        });
                    }
                }
                None => crate::home::show(self, ui),
            }
        }
        self.filter_commands(&ctx, front);
        crate::dialogs::show(self, &ctx);
        self.fade(&ctx, content);
        self.notice_toast(&ctx, content);
        self.closing_banner(&ctx, content);
        self.update_title(&ctx);
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        self.settings.save(storage);
        for slot in &mut self.slots {
            if let Some(m) = slot.module.as_mut() {
                m.app().save(&mut slot.storage);
                eframe::Storage::flush(&mut slot.storage);
            }
        }
    }

    fn on_exit(&mut self) {
        for slot in &mut self.slots {
            if let Some(mut m) = slot.module.take() {
                m.app().save(&mut slot.storage);
                eframe::Storage::flush(&mut slot.storage);
                m.app().on_exit();
            }
        }
        crate::instance::release();
    }

    fn clear_color(&self, visuals: &egui::Visuals) -> [f32; 4] {
        match self.view {
            View::Module(m) => match &self.slots[m.index()].module {
                // The hosted app's own clear colour (`clear_color` needs `&self`).
                Some(_) => visuals.panel_fill.to_normalized_gamma_f32(),
                None => self.palette.bg.to_normalized_gamma_f32(),
            },
            View::Home => self.palette.bg.to_normalized_gamma_f32(),
        }
    }

    fn persist_egui_memory(&self) -> bool {
        false
    }
}

/// A path's file name for display.
pub fn file_name(path: &Path) -> String {
    path.file_name().map_or_else(|| path.display().to_string(), |n| n.to_string_lossy().into_owned())
}
