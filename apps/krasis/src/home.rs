//! The home screen: the logo and slogan, the six apps, and recent files.

use egui::{Align2, Color32, FontId, Id, Rect, Sense, Stroke, StrokeKind, pos2, vec2};

use crate::i18n::tr;
use crate::icons::{self, Icon};
use crate::modules::{ModuleId, SPECS};
use crate::shell::{Krasis, file_name};

enum Click {
    Activate(ModuleId),
    New(ModuleId),
    Open(Option<ModuleId>),
    Recent(usize),
    Forget(usize),
    Reveal(usize),
}

fn greeting(lang: krasis_host::Lang) -> &'static str {
    use chrono::Timelike;
    let h = chrono::Local::now().hour();
    match h {
        5..=11 => tr(lang, "Good morning"),
        12..=17 => tr(lang, "Good afternoon"),
        _ => tr(lang, "Good evening"),
    }
}

pub fn show(app: &mut Krasis, ui: &mut egui::Ui) {
    let p = app.palette;
    let lang = app.lang;
    let ctx = ui.ctx().clone();
    let full = ui.available_rect_before_wrap();
    ui.painter().rect_filled(full, 0.0, p.bg);
    let mut clicks = Vec::new();
    egui::CentralPanel::default().frame(egui::Frame::NONE.fill(p.bg)).show(ui, |ui| {
        egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
            let width = (ui.available_width() - 64.0).clamp(320.0, 1120.0);
            let left = ui.max_rect().left() + (ui.available_width() - width) / 2.0;
            ui.add_space(28.0);

            // The hero: logo, name, slogan, greeting and Open.
            let (hero, _) = ui.allocate_exact_size(vec2(ui.available_width(), 176.0), Sense::hover());
            let hero = Rect::from_min_size(pos2(left, hero.top()), vec2(width, 176.0));
            let painter = ui.painter();
            painter.rect_filled(hero, 22.0, p.teal);
            // A soft gold glow behind the mark, like the dot in the logo.
            for (r, a) in [(120.0, 10u8), (84.0, 16), (54.0, 22)] {
                painter.circle_filled(pos2(hero.right() - 140.0, hero.center().y), r, Color32::from_rgba_unmultiplied(0xE9, 0xC2, 0x6E, a));
            }
            // The logo's mark on the banner's teal, which is the logo's own background.
            if let Some(mark) = &app.mark {
                let r = Rect::from_center_size(hero.left_center() + vec2(88.0, 0.0), vec2(92.0, 104.0));
                painter.image(mark.id(), r, Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)), Color32::WHITE);
            }
            let tx = hero.left() + 172.0;
            painter.text(pos2(tx, hero.top() + 50.0), Align2::LEFT_CENTER, "Krasis", crate::shell::strong(&ctx, 40.0), p.cream);
            painter.text(pos2(tx + 2.0, hero.top() + 88.0), Align2::LEFT_CENTER, tr(lang, "Your creative oasis"), FontId::proportional(18.0), p.gold);
            painter.text(
                pos2(tx + 2.0, hero.top() + 122.0),
                Align2::LEFT_CENTER,
                format!("{} · {}", greeting(lang), tr(lang, "What will you create today?")),
                FontId::proportional(14.0),
                Color32::from_rgba_unmultiplied(0xFA, 0xF5, 0xE8, 200),
            );
            if width > 640.0 {
                let br = Rect::from_center_size(pos2(hero.right() - 140.0, hero.center().y), vec2(170.0, 44.0));
                let resp = ui.interact(br, Id::unique("krasis_home_open"), Sense::click());
                let h = ctx.animate_bool_with_time(Id::unique("krasis_home_open_h"), resp.hovered(), 0.12);
                let painter = ui.painter();
                painter.rect_filled(br.expand(h * 2.0), 22.0, p.cream);
                icons::paint(painter, Rect::from_center_size(br.left_center() + vec2(28.0, 0.0), vec2(20.0, 20.0)), &Icon::Open, p.teal);
                painter.text(br.center() + vec2(12.0, 0.0), Align2::CENTER_CENTER, tr(lang, "Open a file…"), FontId::proportional(15.0), p.teal);
                if resp.clicked() {
                    clicks.push(Click::Open(None));
                }
            }

            // The apps.
            ui.add_space(30.0);
            section_title(ui, left, tr(lang, "Apps"), p.text);
            let cols = if width >= 980.0 {
                3
            } else if width >= 640.0 {
                2
            } else {
                1
            };
            let gap = 18.0;
            let card_w = (width - gap * (cols as f32 - 1.0)) / cols as f32;
            let card_h = 172.0;
            let rows = SPECS.len().div_ceil(cols);
            let (grid, _) = ui.allocate_exact_size(vec2(ui.available_width(), rows as f32 * (card_h + gap)), Sense::hover());
            for (i, spec) in SPECS.iter().enumerate() {
                let (row, col) = (i / cols, i % cols);
                let rect = Rect::from_min_size(pos2(left + col as f32 * (card_w + gap), grid.top() + row as f32 * (card_h + gap)), vec2(card_w, card_h));
                let resp = ui.interact(rect, Id::unique(("krasis_card", i)), Sense::click());
                let h = ctx.animate_bool_with_time(Id::unique(("krasis_card_h", i)), resp.hovered(), 0.16);
                let lifted = rect.translate(vec2(0.0, -3.0 * h));
                let painter = ui.painter();
                // Shadow grows as the card lifts.
                for k in 1..=4 {
                    let a = ((6.0 + 10.0 * h) / k as f32) as u8;
                    painter.rect_filled(lifted.translate(vec2(0.0, 2.0 + k as f32 * (1.0 + h))).expand(k as f32), 16.0, Color32::from_black_alpha(a));
                }
                painter.rect_filled(lifted, 16.0, if h > 0.0 { p.card_hover } else { p.card });
                let accent = spec.accent();
                painter.rect_stroke(lifted, 16.0, Stroke::new(1.0, lerp(p.line, accent, h * 0.8)), StrokeKind::Inside);
                let ic = Rect::from_min_size(lifted.min + vec2(20.0, 20.0), vec2(46.0, 46.0));
                painter.rect_filled(ic, 13.0, accent.gamma_multiply(if p.dark { 0.28 } else { 0.14 }));
                icons::paint(painter, ic.shrink(9.0), &Icon::Module(spec.id), accent);
                painter.text(lifted.min + vec2(80.0, 30.0), Align2::LEFT_CENTER, spec.title(lang), crate::shell::strong(&ctx, 16.5), p.text);
                let running = app.spec_loaded(spec.id);
                let sub = if running { tr(lang, "Running").to_string() } else { spec.short(lang).to_string() };
                painter.text(lifted.min + vec2(80.0, 53.0), Align2::LEFT_CENTER, sub, FontId::proportional(12.5), if running { accent } else { p.muted });
                let blurb = painter.layout(spec.blurb(lang).to_string(), FontId::proportional(13.0), p.muted, card_w - 40.0);
                painter.galley(lifted.min + vec2(20.0, 78.0), blurb, p.muted);
                // New and Open.
                let by = lifted.bottom() - 40.0;
                let new_rect = Rect::from_min_size(pos2(lifted.left() + 20.0, by), vec2(((card_w - 52.0) * 0.55).max(120.0), 28.0));
                let open_rect = Rect::from_min_size(pos2(new_rect.right() + 10.0, by), vec2(84.0, 28.0));
                if pill_button(ui, new_rect, Id::unique(("krasis_card_new", i)), spec.new_label(lang), accent, Color32::WHITE, true) {
                    clicks.push(Click::New(spec.id));
                } else if pill_button(ui, open_rect, Id::unique(("krasis_card_open", i)), tr(lang, "Open…"), accent, accent, false) {
                    clicks.push(Click::Open(Some(spec.id)));
                } else if resp.clicked() {
                    clicks.push(Click::Activate(spec.id));
                }
            }

            // Recent files.
            ui.add_space(18.0);
            section_title(ui, left, tr(lang, "Recent files"), p.text);
            if app.settings.recent.is_empty() {
                let (r, _) = ui.allocate_exact_size(vec2(ui.available_width(), 36.0), Sense::hover());
                ui.painter().text(
                    pos2(left, r.center().y),
                    Align2::LEFT_CENTER,
                    tr(lang, "No recent files yet. Files you open in any app appear here."),
                    FontId::proportional(13.5),
                    p.muted,
                );
            }
            for (i, recent) in app.settings.recent.iter().take(12).enumerate() {
                let (r, _) = ui.allocate_exact_size(vec2(ui.available_width(), 46.0), Sense::hover());
                let row = Rect::from_min_size(pos2(left, r.top()), vec2(width, 42.0));
                let resp = ui.interact(row, Id::unique(("krasis_recent", i)), Sense::click());
                let h = ctx.animate_bool_with_time(Id::unique(("krasis_recent_h", i)), resp.hovered(), 0.1);
                let painter = ui.painter();
                painter.rect_filled(row, 10.0, lerp(p.bg, p.card_hover, h));
                let spec = &SPECS[recent.module.index()];
                icons::paint(painter, Rect::from_center_size(row.left_center() + vec2(24.0, 0.0), vec2(20.0, 20.0)), &Icon::Module(spec.id), spec.accent());
                painter.text(row.left_center() + vec2(48.0, -8.0), Align2::LEFT_CENTER, file_name(&recent.path), FontId::proportional(14.0), p.text);
                let folder = recent.path.parent().map(|d| d.display().to_string()).unwrap_or_default();
                painter.text(row.left_center() + vec2(48.0, 10.0), Align2::LEFT_CENTER, folder, FontId::proportional(11.5), p.muted);
                painter.text(row.right_center() - vec2(16.0, 0.0), Align2::RIGHT_CENTER, spec.title(lang), FontId::proportional(12.0), p.muted);
                if resp.clicked() {
                    clicks.push(Click::Recent(i));
                }
                resp.context_menu(|ui| {
                    if ui.button(tr(lang, "Open")).clicked() {
                        ui.close();
                        clicks.push(Click::Recent(i));
                    }
                    if ui.button(tr(lang, "Show in folder")).clicked() {
                        ui.close();
                        clicks.push(Click::Reveal(i));
                    }
                    if ui.button(tr(lang, "Remove from list")).clicked() {
                        ui.close();
                        clicks.push(Click::Forget(i));
                    }
                });
            }
            ui.add_space(20.0);
            let (r, _) = ui.allocate_exact_size(vec2(ui.available_width(), 30.0), Sense::hover());
            ui.painter().text(pos2(r.center().x, r.center().y), Align2::CENTER_CENTER, tr(lang, "Drop files to open them"), FontId::proportional(12.0), p.muted);
            ui.add_space(24.0);
        });
    });
    // A file dragged over home.
    if ctx.input(|i| !i.raw.hovered_files.is_empty()) {
        let painter = ctx.layer_painter(egui::LayerId::new(egui::Order::Foreground, Id::unique("krasis_drop")));
        painter.rect_filled(full.shrink(12.0), 18.0, Color32::from_rgba_unmultiplied(0x06, 0x60, 0x5F, 40));
        painter.rect_stroke(full.shrink(12.0), 18.0, Stroke::new(2.0, p.teal), StrokeKind::Inside);
        painter.text(full.center(), Align2::CENTER_CENTER, tr(lang, "Drop files to open them"), FontId::proportional(22.0), p.teal);
    }
    for c in clicks {
        match c {
            Click::Activate(id) => app.activate(&ctx, id),
            Click::New(id) => app.new_in(&ctx, id),
            Click::Open(only) => app.pick_and_open(&ctx, only),
            Click::Recent(i) => {
                if let Some(r) = app.settings.recent.get(i).cloned() {
                    if r.path.exists() {
                        app.open_in(&ctx, r.module, vec![r.path]);
                    } else {
                        app.route(&ctx, vec![r.path]);
                    }
                }
            }
            Click::Forget(i) => {
                if i < app.settings.recent.len() {
                    app.settings.recent.remove(i);
                }
            }
            Click::Reveal(i) => {
                if let Some(dir) = app.settings.recent.get(i).and_then(|r| r.path.parent().map(std::path::Path::to_path_buf)) {
                    let _ = open::that_detached(dir);
                }
            }
        }
    }
}

fn section_title(ui: &mut egui::Ui, left: f32, text: &str, color: Color32) {
    let (r, _) = ui.allocate_exact_size(vec2(ui.available_width(), 34.0), Sense::hover());
    ui.painter().text(pos2(left + 2.0, r.center().y), Align2::LEFT_CENTER, text, crate::shell::strong(ui.ctx(), 18.0), color);
}

fn pill_button(ui: &mut egui::Ui, rect: Rect, id: Id, label: &str, accent: Color32, text: Color32, filled: bool) -> bool {
    let resp = ui.interact(rect, id, Sense::click());
    let h = ui.ctx().animate_bool_with_time(id.with("h"), resp.hovered(), 0.1);
    let painter = ui.painter();
    if filled {
        painter.rect_filled(rect, 14.0, lerp(accent, Color32::BLACK, 0.12 * h));
    } else {
        painter.rect_filled(rect, 14.0, accent.gamma_multiply(0.08 + 0.1 * h));
    }
    painter.text(rect.center(), Align2::CENTER_CENTER, label, FontId::proportional(13.0), text);
    resp.clicked()
}

fn lerp(a: Color32, b: Color32, t: f32) -> Color32 {
    let t = t.clamp(0.0, 1.0);
    let m = |x: u8, y: u8| (x as f32 + (y as f32 - x as f32) * t) as u8;
    Color32::from_rgba_unmultiplied(m(a.r(), b.r()), m(a.g(), b.g()), m(a.b(), b.b()), m(a.a(), b.a()))
}
