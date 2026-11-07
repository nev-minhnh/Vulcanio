//! The home screen: the logo and slogan, the six apps, and recent files.

use egui::{Align2, Color32, FontId, Id, Rect, Sense, Stroke, StrokeKind, pos2, vec2};

use crate::i18n::tr;
use crate::icons::{self, Icon};
use crate::modules::{ModuleId, SPECS};
use crate::shell::{Vulcanio, file_name};

enum Click {
    Activate(ModuleId),
    New(ModuleId),
    Open(Option<ModuleId>),
    Recent(usize),
    Forget(usize),
    Reveal(usize),
}

fn greeting(lang: vulcanio_host::Lang) -> &'static str {
    use chrono::Timelike;
    let h = chrono::Local::now().hour();
    match h {
        5..=11 => tr(lang, "Good morning"),
        12..=17 => tr(lang, "Good afternoon"),
        _ => tr(lang, "Good evening"),
    }
}

pub fn show(app: &mut Vulcanio, ui: &mut egui::Ui) {
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

            // The hero: the logo on its own charcoal, name, slogan, greeting and Open.
            let (hero, _) = ui.allocate_exact_size(vec2(ui.available_width(), 184.0), Sense::hover());
            let hero = Rect::from_min_size(pos2(left, hero.top()), vec2(width, 184.0));
            let painter = ui.painter();
            painter.rect_filled(hero, 24.0, p.rail);
            // The logo's glow, soft behind the banner's right side.
            let glow_painter = painter.with_clip_rect(hero.shrink(1.0));
            for (i, (dx, dy, r)) in [(-150.0f32, -30.0f32, 150.0f32), (-40.0, 50.0, 120.0), (-260.0, 70.0, 110.0)].iter().enumerate() {
                let c = crate::shell::GLOW[[1, 4, 0][i]];
                for k in 0..10 {
                    let rr = r * (1.0 - k as f32 * 0.09);
                    glow_painter.circle_filled(pos2(hero.right() + dx, hero.center().y + dy), rr, Color32::from_rgba_unmultiplied(c.r(), c.g(), c.b(), 3));
                }
            }
            painter.rect_stroke(hero, 24.0, Stroke::new(1.0, Color32::from_white_alpha(14)), StrokeKind::Inside);
            if let Some(logo) = app.mark.as_ref().or(app.logo.as_ref()) {
                let r = Rect::from_center_size(hero.left_center() + vec2(92.0, 0.0), vec2(124.0, 124.0));
                painter.image(logo.id(), r, Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)), Color32::WHITE);
            }
            let tx = hero.left() + 184.0;
            painter.text(pos2(tx, hero.top() + 54.0), Align2::LEFT_CENTER, "Vulcanio", crate::shell::strong(&ctx, 42.0), p.on_rail);
            // The slogan in the logo's warm glow.
            let slogan = tr(lang, "The all-in-one studio");
            let font = FontId::proportional(19.0);
            let mut x = tx + 2.0;
            let chars: Vec<char> = slogan.chars().collect();
            let n = chars.len().max(2) as f32 - 1.0;
            for (i, ch) in chars.iter().enumerate() {
                let t = i as f32 / n;
                let c = lerp(crate::shell::GLOW[0], crate::shell::GLOW[2], t);
                let g = painter.layout_no_wrap(ch.to_string(), font.clone(), c);
                let w = g.size().x;
                painter.galley(pos2(x, hero.top() + 93.0 - g.size().y / 2.0), g, c);
                x += w;
            }
            painter.text(
                pos2(tx + 2.0, hero.top() + 130.0),
                Align2::LEFT_CENTER,
                format!("{} · {}", greeting(lang), tr(lang, "What will you create today?")),
                FontId::proportional(14.0),
                p.on_rail.gamma_multiply(0.75),
            );
            if width > 700.0 {
                let br = Rect::from_center_size(pos2(hero.right() - 130.0, hero.center().y), vec2(176.0, 46.0));
                let resp = ui.interact(br, Id::unique("vulcanio_home_open"), Sense::click());
                let h = ctx.animate_bool_with_time(Id::unique("vulcanio_home_open_h"), resp.hovered(), 0.12);
                let painter = ui.painter();
                painter.rect_filled(br.expand(h * 2.0), 23.0, p.on_rail);
                let ink = Color32::from_rgb(0x2A, 0x1F, 0x3D);
                icons::paint(painter, Rect::from_center_size(br.left_center() + vec2(30.0, 0.0), vec2(20.0, 20.0)), &Icon::Open, ink);
                painter.text(br.center() + vec2(12.0, 0.0), Align2::CENTER_CENTER, tr(lang, "Open a file…"), FontId::proportional(15.0), ink);
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
                let resp = ui.interact(rect, Id::unique(("vulcanio_card", i)), Sense::click());
                let h = ctx.animate_bool_with_time(Id::unique(("vulcanio_card_h", i)), resp.hovered(), 0.16);
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
                if pill_button(ui, new_rect, Id::unique(("vulcanio_card_new", i)), spec.new_label(lang), accent, Color32::WHITE, true) {
                    clicks.push(Click::New(spec.id));
                } else if pill_button(ui, open_rect, Id::unique(("vulcanio_card_open", i)), tr(lang, "Open…"), accent, accent, false) {
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
                let resp = ui.interact(row, Id::unique(("vulcanio_recent", i)), Sense::click());
                let h = ctx.animate_bool_with_time(Id::unique(("vulcanio_recent_h", i)), resp.hovered(), 0.1);
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
        let painter = ctx.layer_painter(egui::LayerId::new(egui::Order::Foreground, Id::unique("vulcanio_drop")));
        painter.rect_filled(full.shrink(12.0), 18.0, p.accent.gamma_multiply(0.14));
        painter.rect_stroke(full.shrink(12.0), 18.0, Stroke::new(2.0, p.accent), StrokeKind::Inside);
        painter.text(full.center(), Align2::CENTER_CENTER, tr(lang, "Drop files to open them"), FontId::proportional(22.0), p.accent);
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
