//! The Settings window: a sidebar of pages (General, Appearance, Motion, Files, Performance,
//! About) and, on each page, cards of rows — a title and a line of explanation on the left, the
//! control on the right. Every change takes effect at once; there is nothing to apply or save.

use egui::{Align, Align2, Color32, CornerRadius, FontId, Id, Layout, Rect, Sense, Stroke, StrokeKind, UiBuilder, pos2, vec2};
use vulcanio_host::Lang;

use crate::i18n::tr;
use crate::icons::{self, Icon};
use crate::modules::SPECS;
use crate::settings::ThemeChoice;
use crate::shell::{GLOW, Palette, Vulcanio};

/// A page of the Settings window.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Page {
    #[default]
    General,
    Appearance,
    Motion,
    Files,
    Performance,
    About,
}

const PAGES: [Page; 6] = [Page::General, Page::Appearance, Page::Motion, Page::Files, Page::Performance, Page::About];

impl Page {
    fn title(self, lang: Lang) -> &'static str {
        tr(
            lang,
            match self {
                Page::General => "General",
                Page::Appearance => "Appearance",
                Page::Motion => "Motion and smoothness",
                Page::Files => "Files",
                Page::Performance => "Performance and memory",
                Page::About => "About Vulcanio",
            },
        )
    }

    fn nav_label(self, lang: Lang) -> &'static str {
        tr(
            lang,
            match self {
                Page::General => "General",
                Page::Appearance => "Appearance",
                Page::Motion => "Motion",
                Page::Files => "Files",
                Page::Performance => "Performance",
                Page::About => "About",
            },
        )
    }

    fn subtitle(self, lang: Lang) -> &'static str {
        tr(
            lang,
            match self {
                Page::General => "Language and what happens when Vulcanio starts.",
                Page::Appearance => "One theme for Vulcanio and all six apps.",
                Page::Motion => "Scrolling, zooming and transitions, tuned to feel like Microsoft Edge.",
                Page::Files => "Which files open in Vulcanio, and in which app.",
                Page::Performance => "Keep Vulcanio light on memory and battery.",
                Page::About => "The all-in-one studio",
            },
        )
    }

    fn icon(self) -> Icon {
        match self {
            Page::General => Icon::Settings,
            Page::Appearance => Icon::Palette,
            Page::Motion => Icon::Motion,
            Page::Files => Icon::Open,
            Page::Performance => Icon::Gauge,
            Page::About => Icon::Info,
        }
    }
}

/// Show the Settings window while `app.show_settings` is set.
pub fn show(app: &mut Vulcanio, ctx: &egui::Context) {
    let p = app.palette;
    let screen = ctx.content_rect();
    let size = vec2((screen.width() - 64.0).clamp(560.0, 900.0), (screen.height() - 64.0).clamp(420.0, 640.0));
    let frame = egui::Frame::new()
        .fill(p.bg)
        .stroke(Stroke::new(1.0, p.line))
        .corner_radius(CornerRadius::same(18))
        .shadow(ctx.global_style().visuals.window_shadow);
    let mut close = false;
    let modal = egui::Modal::new(Id::unique("vulcanio_settings")).frame(frame).show(ctx, |ui| {
        let (rect, _) = ui.allocate_exact_size(size, Sense::hover());
        let nav_w = 212.0;
        let nav = Rect::from_min_size(rect.min, vec2(nav_w, rect.height()));
        let body = Rect::from_min_max(pos2(nav.right(), rect.top()), rect.max);
        // The sidebar, a shade off the page, behind a hairline.
        let nav_fill = if p.dark { Color32::from_rgb(0x14, 0x14, 0x19) } else { Color32::from_rgb(0xEE, 0xEC, 0xF4) };
        ui.painter().rect_filled(nav, CornerRadius { nw: 18, sw: 18, ne: 0, se: 0 }, nav_fill);
        ui.painter().line_segment([nav.right_top(), nav.right_bottom()], Stroke::new(1.0, p.line));
        ui.scope_builder(UiBuilder::new().max_rect(nav), |ui| sidebar(app, ui, nav));
        ui.scope_builder(UiBuilder::new().max_rect(body), |ui| {
            if page(app, ui, body) {
                close = true;
            }
        });
    });
    if close || modal.should_close() {
        app.show_settings = false;
    }
}

fn sidebar(app: &mut Vulcanio, ui: &mut egui::Ui, rect: Rect) {
    let p = app.palette;
    let lang = app.lang;
    let painter = ui.painter().clone();
    if let Some(logo) = &app.logo {
        let r = Rect::from_min_size(rect.min + vec2(22.0, 24.0), vec2(30.0, 30.0));
        painter.image(logo.id(), r, Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)), Color32::WHITE);
    }
    painter.text(rect.min + vec2(62.0, 39.0), Align2::LEFT_CENTER, tr(lang, "Settings"), crate::shell::strong(ui.ctx(), 19.0), p.text);
    let item_h = 40.0;
    let top = rect.top() + 82.0;
    let index = PAGES.iter().position(|x| *x == app.settings_page).unwrap_or(0) as f32;
    let anim = if app.settings.animations { 0.18 } else { 0.0 };
    let y = ui.ctx().animate_value_with_time(Id::unique("vulcanio_settings_nav_pill"), index, anim);
    let pill = Rect::from_min_size(pos2(rect.left() + 12.0, top + y * item_h), vec2(rect.width() - 24.0, item_h - 4.0));
    painter.rect_filled(pill, 10.0, p.accent.gamma_multiply(if p.dark { 0.22 } else { 0.12 }));
    crate::shell::gradient_rect(&painter, Rect::from_min_size(pos2(pill.left(), pill.center().y - 9.0), vec2(3.0, 18.0)), &[GLOW[0], GLOW[1], GLOW[3]], true);
    for (i, pg) in PAGES.iter().enumerate() {
        let r = Rect::from_min_size(pos2(rect.left() + 12.0, top + i as f32 * item_h), vec2(rect.width() - 24.0, item_h - 4.0));
        let resp = ui.interact(r, Id::unique(("vulcanio_settings_nav", i)), Sense::click());
        let selected = *pg == app.settings_page;
        let hover = ui.ctx().animate_bool_with_time(resp.id.with("h"), resp.hovered() && !selected, 0.1);
        if hover > 0.0 {
            painter.rect_filled(r, 10.0, p.text.gamma_multiply(0.06 * hover));
        }
        let color = if selected { p.accent } else { p.text.gamma_multiply(0.82) };
        icons::paint(&painter, Rect::from_center_size(pos2(r.left() + 22.0, r.center().y), vec2(18.0, 18.0)), &pg.icon(), color);
        let font = if selected { crate::shell::strong(ui.ctx(), 14.0) } else { FontId::proportional(14.0) };
        painter.text(pos2(r.left() + 42.0, r.center().y), Align2::LEFT_CENTER, pg.nav_label(lang), font, color);
        if resp.clicked() {
            app.settings_page = *pg;
        }
    }
    // Where the settings are kept, small at the bottom.
    painter.text(
        pos2(rect.left() + 22.0, rect.bottom() - 22.0),
        Align2::LEFT_CENTER,
        format!("Vulcanio {}", env!("CARGO_PKG_VERSION")),
        FontId::proportional(12.0),
        p.muted,
    );
}

/// The page on the right. True when its close button is clicked.
fn page(app: &mut Vulcanio, ui: &mut egui::Ui, rect: Rect) -> bool {
    let p = app.palette;
    let lang = app.lang;
    let pg = app.settings_page;
    let painter = ui.painter().clone();
    let pad = 32.0;
    painter.text(pos2(rect.left() + pad, rect.top() + 40.0), Align2::LEFT_CENTER, pg.title(lang), crate::shell::strong(ui.ctx(), 22.0), p.text);
    painter.text(pos2(rect.left() + pad, rect.top() + 66.0), Align2::LEFT_CENTER, pg.subtitle(lang), FontId::proportional(13.5), p.muted);
    let close = close_button(ui, &p, Rect::from_center_size(pos2(rect.right() - 30.0, rect.top() + 32.0), vec2(34.0, 34.0)), tr(lang, "Close"));
    let content = Rect::from_min_max(pos2(rect.left(), rect.top() + 92.0), pos2(rect.right(), rect.bottom() - 8.0));
    ui.scope_builder(UiBuilder::new().max_rect(content), |ui| {
        egui::ScrollArea::vertical().id_salt(("vulcanio_settings_page", pg as u8)).auto_shrink([false, false]).show(ui, |ui| {
            ui.add_space(4.0);
            let inner = Rect::from_min_size(ui.cursor().min + vec2(pad, 0.0), vec2(content.width() - pad * 2.0, 0.0));
            ui.scope_builder(UiBuilder::new().max_rect(Rect::from_min_size(inner.min, vec2(inner.width(), f32::INFINITY))), |ui| {
                ui.set_width(inner.width());
                ui.spacing_mut().item_spacing.y = 0.0;
                match pg {
                    Page::General => general(app, ui),
                    Page::Appearance => appearance(app, ui),
                    Page::Motion => motion(app, ui),
                    Page::Files => files(app, ui),
                    Page::Performance => performance(app, ui),
                    Page::About => about(app, ui),
                }
                ui.add_space(24.0);
            });
        });
    });
    close
}

// ----- Pages -----

fn general(app: &mut Vulcanio, ui: &mut egui::Ui) {
    let (p, lang) = (app.palette, app.lang);
    section(ui, &p, tr(lang, "Language"));
    card(ui, &p, |ui| {
        row(ui, &p, tr(lang, "Interface language"), Some(tr(lang, "For Vulcanio and all six apps.")), 300.0, |ui| {
            let mut options = vec![(None, tr(lang, "Same as Windows").to_string())];
            options.extend(Lang::ALL.iter().map(|l| (Some(*l), l.name().to_string())));
            segmented(ui, &p, "lang", &mut app.settings.lang, &options, app.settings.animations);
        });
    });
    section(ui, &p, tr(lang, "Startup"));
    card(ui, &p, |ui| {
        row(ui, &p, tr(lang, "Reopen the last app on start"), Some(tr(lang, "Otherwise Vulcanio starts on Home.")), 60.0, |ui| {
            toggle(ui, &p, &mut app.settings.reopen_last);
        });
        divider(ui, &p);
        row(ui, &p, tr(lang, "Startup boost"), Some(tr(lang, "Gets Vulcanio ready in the background after Windows signs in, so the first start after turning on the computer is quick.")), 60.0, |ui| {
            if toggle(ui, &p, &mut app.settings.startup_boost) {
                crate::boost::register(app.settings.startup_boost);
            }
        });
    });
}

fn appearance(app: &mut Vulcanio, ui: &mut egui::Ui) {
    let (p, lang) = (app.palette, app.lang);
    section(ui, &p, tr(lang, "Theme"));
    card(ui, &p, |ui| {
        ui.add_space(16.0);
        let w = ui.available_width();
        let tile = vec2(((w - 32.0 - 2.0 * 14.0) / 3.0).min(190.0), 0.0);
        let (row_rect, _) = ui.allocate_exact_size(vec2(w, tile.x * 0.62 + 34.0), Sense::hover());
        let choices = [(ThemeChoice::System, tr(lang, "System")), (ThemeChoice::Light, tr(lang, "Light")), (ThemeChoice::Dark, tr(lang, "Dark"))];
        for (i, (choice, label)) in choices.iter().enumerate() {
            let r = Rect::from_min_size(pos2(row_rect.left() + 16.0 + i as f32 * (tile.x + 14.0), row_rect.top()), vec2(tile.x, tile.x * 0.62));
            let resp = ui.interact(r.union(r.translate(vec2(0.0, 28.0))), Id::unique(("vulcanio_theme_tile", i)), Sense::click());
            let selected = app.settings.theme == *choice;
            let hover = ui.ctx().animate_bool_with_time(resp.id.with("h"), resp.hovered(), 0.12);
            theme_preview(ui.painter(), r, *choice);
            let ring = if selected { p.accent } else { p.line.lerp_to_gamma(p.accent, 0.4 * hover) };
            ui.painter().rect_stroke(r.expand(if selected { 2.0 } else { 0.5 }), 12.0, Stroke::new(if selected { 2.5 } else { 1.0 }, ring), StrokeKind::Outside);
            if selected {
                let c = pos2(r.right() - 14.0, r.top() + 14.0);
                ui.painter().circle_filled(c, 9.0, p.accent);
                ui.painter().add(egui::Shape::line(vec![c + vec2(-4.0, 0.0), c + vec2(-1.0, 3.0), c + vec2(4.5, -3.0)], Stroke::new(1.8, Color32::WHITE)));
            }
            ui.painter().text(pos2(r.center().x, r.bottom() + 17.0), Align2::CENTER_CENTER, *label, if selected { crate::shell::strong(ui.ctx(), 13.5) } else { FontId::proportional(13.5) }, p.text);
            if resp.clicked() {
                app.settings.theme = *choice;
            }
        }
        ui.add_space(14.0);
    });
    section(ui, &p, tr(lang, "App bar"));
    card(ui, &p, |ui| {
        row(ui, &p, tr(lang, "Hide the app bar while an app is open"), Some(tr(lang, "A thin coloured line stays at the left edge; point at it to bring the bar back.")), 60.0, |ui| {
            if toggle(ui, &p, &mut app.settings.rail_autohide) {
                app.rail_open = false;
            }
        });
    });
}

fn motion(app: &mut Vulcanio, ui: &mut egui::Ui) {
    let (p, lang) = (app.palette, app.lang);
    section(ui, &p, tr(lang, "Scrolling and zooming"));
    card(ui, &p, |ui| {
        row(ui, &p, tr(lang, "Smooth scrolling and zooming"), Some(tr(lang, "Each wheel notch glides instead of jumping.")), 60.0, |ui| {
            toggle(ui, &p, &mut app.settings.smooth_scrolling);
        });
        divider(ui, &p);
        row(ui, &p, tr(lang, "Glide time"), Some(tr(lang, "Shorter feels snappier, longer feels softer.")), 260.0, |ui| {
            ui.add_enabled_ui(app.settings.smooth_scrolling, |ui| {
                ui.spacing_mut().slider_width = 170.0;
                ui.add(egui::Slider::new(&mut app.settings.ease_ms, 60..=320).suffix(" ms"));
            });
        });
    });
    section(ui, &p, tr(lang, "Animation"));
    card(ui, &p, |ui| {
        row(ui, &p, tr(lang, "Animated transitions"), Some(tr(lang, "Switching apps, panels sliding in, menus and tooltips fading.")), 60.0, |ui| {
            toggle(ui, &p, &mut app.settings.animations);
        });
    });
    section(ui, &p, tr(lang, "Frame pacing"));
    card(ui, &p, |ui| {
        row(ui, &p, tr(lang, "Presentation"), Some(tr(lang, "Takes effect the next time Vulcanio starts.")), 320.0, |ui| {
            let options = [(false, tr(lang, "Smooth (VSync)").to_string()), (true, tr(lang, "Lowest latency").to_string())];
            segmented(ui, &p, "latency", &mut app.settings.low_latency, &options, app.settings.animations);
        });
    });
}

fn files(app: &mut Vulcanio, ui: &mut egui::Ui) {
    let (p, lang) = (app.palette, app.lang);
    section(ui, &p, tr(lang, "File Explorer"));
    card(ui, &p, |ui| {
        let desc = app.assoc_note.clone().unwrap_or_else(|| tr(lang, "Adds Vulcanio to Open with for the file types it opens.").to_string());
        row(ui, &p, tr(lang, "Make Vulcanio the app for these file types"), Some(&desc), 160.0, |ui| {
            if primary_button(ui, &p, tr(lang, "Register")) {
                app.assoc_note = Some(match crate::associations::register() {
                    Ok(()) => tr(lang, "Registered for the file types Vulcanio opens.").to_string(),
                    Err(e) => e,
                });
            }
        });
    });
    section(ui, &p, tr(lang, "Remembered choices"));
    card(ui, &p, |ui| {
        if app.settings.open_with.is_empty() {
            row(ui, &p, tr(lang, "None yet."), Some(tr(lang, "When a file type opens in more than one app, the app you pick with “Always use” is listed here.")), 0.0, |_| {});
            return;
        }
        let mut forget = None;
        let entries: Vec<_> = app.settings.open_with.iter().map(|(e, m)| (e.clone(), *m)).collect();
        for (i, (ext, id)) in entries.iter().enumerate() {
            if i > 0 {
                divider(ui, &p);
            }
            let title = format!(".{ext}");
            row(ui, &p, &title, Some(SPECS[id.index()].title(lang)), 100.0, |ui| {
                if secondary_button(ui, &p, tr(lang, "Forget")) {
                    forget = Some(ext.clone());
                }
            });
        }
        if let Some(ext) = forget {
            app.settings.open_with.remove(&ext);
        }
    });
}

fn performance(app: &mut Vulcanio, ui: &mut egui::Ui) {
    let (p, lang) = (app.palette, app.lang);
    section(ui, &p, tr(lang, "Memory"));
    card(ui, &p, |ui| {
        row(ui, &p, tr(lang, "Close idle apps"), Some(tr(lang, "An app with nothing open closes after a minute in the background and gives its memory back.")), 60.0, |ui| {
            toggle(ui, &p, &mut app.settings.free_idle_apps);
        });
    });
    section(ui, &p, tr(lang, "Diagnostics"));
    card(ui, &p, |ui| {
        row(ui, &p, tr(lang, "Log files"), Some(tr(lang, "Startup time, graphics errors and crashes are written here.")), 160.0, |ui| {
            if secondary_button(ui, &p, tr(lang, "Open folder")) {
                if let Some(dir) = crate::data_dir().map(|d| d.join("logs")) {
                    let _ = open::that_detached(dir);
                }
            }
        });
    });
}

fn about(app: &mut Vulcanio, ui: &mut egui::Ui) {
    let (p, lang) = (app.palette, app.lang);
    card(ui, &p, |ui| {
        let w = ui.available_width();
        let (r, _) = ui.allocate_exact_size(vec2(w, 132.0), Sense::hover());
        let painter = ui.painter();
        painter.rect_filled(r.shrink(1.0), CornerRadius::same(13), p.rail);
        if let Some(logo) = app.mark.as_ref().or(app.logo.as_ref()) {
            painter.image(logo.id(), Rect::from_center_size(pos2(r.left() + 70.0, r.center().y), vec2(92.0, 92.0)), Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)), Color32::WHITE);
        }
        painter.text(pos2(r.left() + 132.0, r.top() + 44.0), Align2::LEFT_CENTER, "Vulcanio", crate::shell::strong(ui.ctx(), 28.0), p.on_rail);
        painter.text(pos2(r.left() + 133.0, r.top() + 76.0), Align2::LEFT_CENTER, tr(lang, "The all-in-one studio"), FontId::proportional(15.0), p.spark);
        painter.text(pos2(r.left() + 133.0, r.top() + 102.0), Align2::LEFT_CENTER, format!("{} {}", tr(lang, "Version"), env!("CARGO_PKG_VERSION")), FontId::proportional(12.5), p.on_rail.gamma_multiply(0.7));
    });
    section(ui, &p, tr(lang, "Apps"));
    card(ui, &p, |ui| {
        for (i, s) in SPECS.iter().enumerate() {
            if i > 0 {
                divider(ui, &p);
            }
            let w = ui.available_width();
            let (r, _) = ui.allocate_exact_size(vec2(w, 44.0), Sense::hover());
            let tile = Rect::from_center_size(pos2(r.left() + 30.0, r.center().y), vec2(28.0, 28.0));
            ui.painter().rect_filled(tile, 8.0, s.accent().gamma_multiply(0.16));
            icons::paint(ui.painter(), tile.shrink(5.0), &Icon::Module(s.id), s.accent());
            ui.painter().text(pos2(r.left() + 56.0, r.center().y), Align2::LEFT_CENTER, s.title(lang), FontId::proportional(14.0), p.text);
        }
    });
    section(ui, &p, tr(lang, "License"));
    card(ui, &p, |ui| {
        row(ui, &p, tr(lang, "Open-source components under MIT OR Apache-2.0; see NOTICE.md."), Some(tr(lang, "Vulcanio brings six open-source creative apps together in one window.")), 0.0, |_| {});
    });
}

// ----- Building blocks -----

/// A small heading above a card.
fn section(ui: &mut egui::Ui, p: &Palette, text: &str) {
    ui.add_space(18.0);
    let (r, _) = ui.allocate_exact_size(vec2(ui.available_width(), 18.0), Sense::hover());
    ui.painter().text(r.left_center() + vec2(4.0, 0.0), Align2::LEFT_CENTER, text.to_uppercase(), crate::shell::strong(ui.ctx(), 11.5), p.muted);
    ui.add_space(8.0);
}

/// A rounded card holding rows.
fn card(ui: &mut egui::Ui, p: &Palette, add: impl FnOnce(&mut egui::Ui)) {
    let w = ui.available_width();
    egui::Frame::new().fill(p.card).stroke(Stroke::new(1.0, p.line)).corner_radius(CornerRadius::same(14)).show(ui, |ui| {
        ui.set_width(w - 2.0);
        ui.spacing_mut().item_spacing.y = 0.0;
        add(ui);
    });
}

/// A hairline between two rows of a card.
fn divider(ui: &mut egui::Ui, p: &Palette) {
    let (r, _) = ui.allocate_exact_size(vec2(ui.available_width(), 1.0), Sense::hover());
    ui.painter().line_segment([pos2(r.left() + 18.0, r.center().y), pos2(r.right() - 18.0, r.center().y)], Stroke::new(1.0, p.line));
}

/// A row: title and explanation on the left, `control` (about `control_w` wide) on the right.
fn row(ui: &mut egui::Ui, p: &Palette, title: &str, desc: Option<&str>, control_w: f32, control: impl FnOnce(&mut egui::Ui)) {
    let w = ui.available_width();
    let text_w = (w - control_w - 18.0 * 2.0 - if control_w > 0.0 { 24.0 } else { 0.0 }).max(120.0);
    let painter = ui.painter().clone();
    let title_g = painter.layout(title.to_string(), crate::shell::strong(ui.ctx(), 14.0), p.text, text_w);
    let desc_g = desc.map(|d| painter.layout(d.to_string(), FontId::proportional(12.5), p.muted, text_w));
    let text_h = title_g.size().y + desc_g.as_ref().map_or(0.0, |g| 4.0 + g.size().y);
    let h = (text_h + 28.0).max(60.0);
    let (rect, _) = ui.allocate_exact_size(vec2(w, h), Sense::hover());
    let mut y = rect.center().y - text_h / 2.0;
    let th = title_g.size().y;
    painter.galley(pos2(rect.left() + 18.0, y), title_g, p.text);
    y += th + 4.0;
    if let Some(g) = desc_g {
        painter.galley(pos2(rect.left() + 18.0, y), g, p.muted);
    }
    if control_w > 0.0 {
        let c = Rect::from_min_max(pos2(rect.right() - 18.0 - control_w, rect.top()), pos2(rect.right() - 18.0, rect.bottom()));
        ui.scope_builder(UiBuilder::new().max_rect(c).layout(Layout::right_to_left(Align::Center)), control);
    }
}

/// An on/off switch. True when it was flipped.
fn toggle(ui: &mut egui::Ui, p: &Palette, on: &mut bool) -> bool {
    let (rect, resp) = ui.allocate_exact_size(vec2(44.0, 24.0), Sense::click());
    if resp.clicked() {
        *on = !*on;
    }
    let t = ui.ctx().animate_bool_with_time(resp.id, *on, 0.16);
    let off = if p.dark { Color32::from_rgb(0x3A, 0x39, 0x44) } else { Color32::from_rgb(0xD5, 0xD2, 0xDE) };
    let fill = off.lerp_to_gamma(p.accent, t);
    let hover = ui.ctx().animate_bool_with_time(resp.id.with("h"), resp.hovered(), 0.1);
    ui.painter().rect_filled(rect, 12.0, fill.gamma_multiply(1.0 - 0.08 * hover));
    let x = egui::lerp((rect.left() + 12.0)..=(rect.right() - 12.0), t);
    ui.painter().circle_filled(pos2(x, rect.center().y + 0.6), 9.6, Color32::from_black_alpha(40));
    ui.painter().circle_filled(pos2(x, rect.center().y), 9.0 + hover, Color32::WHITE);
    resp.clicked()
}

/// A segmented control: one pill per option, the chosen one raised. True when it changed.
fn segmented<T: PartialEq + Copy>(ui: &mut egui::Ui, p: &Palette, key: &str, value: &mut T, options: &[(T, String)], animate: bool) -> bool {
    let font = FontId::proportional(13.0);
    let galleys: Vec<_> = options.iter().map(|(_, l)| ui.painter().layout_no_wrap(l.clone(), font.clone(), p.text)).collect();
    let widths: Vec<f32> = galleys.iter().map(|g| g.size().x + 26.0).collect();
    let total: f32 = widths.iter().sum::<f32>() + 8.0;
    let (rect, _) = ui.allocate_exact_size(vec2(total, 34.0), Sense::hover());
    let track = if p.dark { Color32::from_rgb(0x24, 0x23, 0x2C) } else { Color32::from_rgb(0xEC, 0xEA, 0xF2) };
    ui.painter().rect_filled(rect, 10.0, track);
    let mut x = rect.left() + 4.0;
    let mut cells = Vec::new();
    for w in &widths {
        cells.push(Rect::from_min_size(pos2(x, rect.top() + 4.0), vec2(*w, rect.height() - 8.0)));
        x += w;
    }
    let chosen = options.iter().position(|(v, _)| v == value).unwrap_or(0);
    let id = Id::unique(("vulcanio_segmented", key));
    let time = if animate { 0.18 } else { 0.0 };
    let left = ui.ctx().animate_value_with_time(id.with("l"), cells[chosen].left(), time);
    let right = ui.ctx().animate_value_with_time(id.with("r"), cells[chosen].right(), time);
    let pill = Rect::from_min_max(pos2(left, cells[chosen].top()), pos2(right, cells[chosen].bottom()));
    ui.painter().rect_filled(pill.translate(vec2(0.0, 1.0)), 8.0, Color32::from_black_alpha(if p.dark { 60 } else { 18 }));
    ui.painter().rect_filled(pill, 8.0, if p.dark { Color32::from_rgb(0x3A, 0x37, 0x48) } else { Color32::WHITE });
    let mut changed = false;
    for (i, ((v, _), g)) in options.iter().zip(galleys).enumerate() {
        let resp = ui.interact(cells[i], id.with(i), Sense::click());
        let color = if i == chosen { p.text } else { p.text.gamma_multiply(if resp.hovered() { 0.9 } else { 0.65 }) };
        ui.painter().galley(cells[i].center() - g.size() / 2.0, g, color);
        if resp.clicked() && i != chosen {
            *value = *v;
            changed = true;
        }
    }
    changed
}

fn primary_button(ui: &mut egui::Ui, p: &Palette, label: &str) -> bool {
    button(ui, p, label, true)
}

fn secondary_button(ui: &mut egui::Ui, p: &Palette, label: &str) -> bool {
    button(ui, p, label, false)
}

fn button(ui: &mut egui::Ui, p: &Palette, label: &str, primary: bool) -> bool {
    let ink = if primary { Color32::WHITE } else { p.text };
    let g = ui.painter().layout_no_wrap(label.to_string(), FontId::proportional(13.5), ink);
    let (rect, resp) = ui.allocate_exact_size(vec2(g.size().x + 32.0, 34.0), Sense::click());
    let hover = ui.ctx().animate_bool_with_time(resp.id.with("h"), resp.hovered(), 0.1);
    if primary {
        ui.painter().rect_filled(rect, 10.0, p.accent.linear_multiply(1.0 - 0.12 * hover));
    } else {
        let fill = if p.dark { Color32::from_rgb(0x2A, 0x29, 0x33) } else { Color32::from_rgb(0xF0, 0xEE, 0xF5) };
        ui.painter().rect_filled(rect, 10.0, fill.lerp_to_gamma(p.accent, 0.12 * hover));
        ui.painter().rect_stroke(rect, 10.0, Stroke::new(1.0, p.line), StrokeKind::Inside);
    }
    ui.painter().galley(rect.center() - g.size() / 2.0, g, ink);
    resp.clicked()
}

/// The round × in a page's corner. True when clicked.
fn close_button(ui: &mut egui::Ui, p: &Palette, rect: Rect, tip: &str) -> bool {
    let resp = ui.interact(rect, Id::unique("vulcanio_settings_close"), Sense::click()).on_hover_text(tip);
    let hover = ui.ctx().animate_bool_with_time(resp.id.with("h"), resp.hovered(), 0.1);
    if hover > 0.0 {
        ui.painter().circle_filled(rect.center(), rect.width() / 2.0, p.text.gamma_multiply(0.08 * hover));
    }
    let (c, d) = (rect.center(), 5.5);
    let stroke = Stroke::new(1.7, p.text.gamma_multiply(0.8));
    ui.painter().line_segment([c + vec2(-d, -d), c + vec2(d, d)], stroke);
    ui.painter().line_segment([c + vec2(-d, d), c + vec2(d, -d)], stroke);
    resp.clicked()
}

/// A small picture of Vulcanio in `choice`'s theme.
fn theme_preview(painter: &egui::Painter, r: Rect, choice: ThemeChoice) {
    let draw = |painter: &egui::Painter, dark: bool| {
        let p = Palette::new(dark);
        painter.rect_filled(r, 12.0, p.bg);
        let rail = Rect::from_min_size(r.min, vec2(r.width() * 0.13, r.height()));
        painter.rect_filled(rail, CornerRadius { nw: 12, sw: 12, ne: 0, se: 0 }, p.rail);
        for k in 0..4 {
            painter.circle_filled(pos2(rail.center().x, rail.top() + 16.0 + k as f32 * 14.0), 3.0, p.on_rail.gamma_multiply(if k == 1 { 1.0 } else { 0.45 }));
        }
        let hero = Rect::from_min_size(pos2(rail.right() + 10.0, r.top() + 10.0), vec2(r.width() - rail.width() - 20.0, r.height() * 0.3));
        painter.rect_filled(hero, 6.0, p.rail);
        crate::shell::gradient_rect(painter, Rect::from_min_size(hero.left_center() + vec2(10.0, -2.0), vec2(hero.width() * 0.4, 4.0)), &[GLOW[0], GLOW[2]], false);
        let cw = (hero.width() - 12.0) / 3.0;
        for k in 0..3 {
            let c = Rect::from_min_size(pos2(hero.left() + k as f32 * (cw + 6.0), hero.bottom() + 8.0), vec2(cw, r.bottom() - hero.bottom() - 18.0));
            painter.rect_filled(c, 5.0, p.card);
            painter.rect_stroke(c, 5.0, Stroke::new(1.0, p.line), StrokeKind::Inside);
            painter.rect_filled(Rect::from_min_size(c.min + vec2(6.0, 6.0), vec2(10.0, 10.0)), 3.0, GLOW[[0, 3, 4][k]].gamma_multiply(0.8));
        }
    };
    match choice {
        ThemeChoice::Light => draw(painter, false),
        ThemeChoice::Dark => draw(painter, true),
        ThemeChoice::System => {
            draw(&painter.with_clip_rect(Rect::from_min_max(r.min, pos2(r.center().x, r.bottom()))), false);
            draw(&painter.with_clip_rect(Rect::from_min_max(pos2(r.center().x, r.top()), r.max)), true);
            painter.line_segment([pos2(r.center().x, r.top()), pos2(r.center().x, r.bottom())], Stroke::new(1.0, Color32::from_gray(128).gamma_multiply(0.5)));
        }
    }
}
