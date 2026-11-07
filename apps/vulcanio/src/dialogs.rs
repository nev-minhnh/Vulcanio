//! Vulcanio's dialogs: Open With, Settings and About.

use egui::{Color32, Id, RichText, pos2, vec2};
use vulcanio_host::Lang;

use crate::i18n::{tr, trf};
use crate::icons::{self, Icon};
use crate::modules::{self, ModuleId, SPECS};
use crate::settings::ThemeChoice;
use crate::shell::{Vulcanio, file_name};

pub fn show(app: &mut Vulcanio, ctx: &egui::Context) {
    open_with(app, ctx);
    if app.show_settings {
        settings(app, ctx);
    }
    if app.show_about {
        about(app, ctx);
    }
}

fn open_with(app: &mut Vulcanio, ctx: &egui::Context) {
    let Some(ow) = &mut app.open_with else { return };
    let lang = app.lang;
    let mut chosen: Option<ModuleId> = None;
    let mut cancel = false;
    let title = if ow.files.len() == 1 {
        trf(lang, "Open {file} with", &[("file", &file_name(&ow.files[0]))])
    } else {
        trf(lang, "Open {file} with", &[("file", &trf(lang, "{n} files", &[("n", &ow.files.len().to_string())]))])
    };
    let modal = egui::Modal::new(Id::unique("vulcanio_open_with")).show(ctx, |ui| {
        ui.set_width(440.0);
        if title_bar(ui, &title, tr(lang, "Cancel")) {
            cancel = true;
        }
        ui.add_space(6.0);
        for (i, id) in ow.candidates.iter().enumerate() {
            let spec = &SPECS[id.index()];
            let (rect, resp) = ui.allocate_exact_size(vec2(ui.available_width(), 52.0), egui::Sense::click());
            let h = ui.ctx().animate_bool_with_time(Id::unique(("vulcanio_ow", i)), resp.hovered(), 0.1);
            let painter = ui.painter();
            painter.rect_filled(rect, 10.0, spec.accent().gamma_multiply(0.06 + 0.12 * h));
            icons::paint(painter, egui::Rect::from_center_size(rect.left_center() + vec2(26.0, 0.0), vec2(24.0, 24.0)), &Icon::Module(*id), spec.accent());
            let text_color = ui.visuals().text_color();
            painter.text(rect.left_center() + vec2(52.0, -8.0), egui::Align2::LEFT_CENTER, spec.title(lang), egui::FontId::proportional(15.0), text_color);
            let sub = if i == 0 { tr(lang, "Recommended").to_string() } else { String::new() };
            painter.text(rect.left_center() + vec2(52.0, 10.0), egui::Align2::LEFT_CENTER, sub, egui::FontId::proportional(12.0), ui.visuals().weak_text_color());
            if resp.clicked() {
                chosen = Some(*id);
            }
            ui.add_space(6.0);
        }
        ui.add_space(4.0);
        ui.checkbox(&mut ow.remember, trf(lang, "Always use this app for .{ext} files", &[("ext", &ow.ext)]));
        ui.add_space(8.0);
        if footer_button(ui, tr(lang, "Cancel")) {
            cancel = true;
        }
    });
    if modal.should_close() {
        cancel = true;
    }
    if let Some(id) = chosen {
        if let Some(ow) = app.open_with.take() {
            if ow.remember {
                app.settings.open_with.insert(ow.ext.clone(), id);
            }
            app.open_in(ctx, id, ow.files);
        }
    } else if cancel {
        app.open_with = None;
    }
}

fn settings(app: &mut Vulcanio, ctx: &egui::Context) {
    let lang = app.lang;
    let mut close = false;
    let accent = app.palette.accent;
    let modal = egui::Modal::new(Id::unique("vulcanio_settings")).show(ctx, |ui| {
        ui.set_width(540.0);
        if title_bar(ui, tr(lang, "Settings"), tr(lang, "Close")) {
            close = true;
        }
        ui.add_space(4.0);
        let max_h = (ctx.content_rect().height() - 200.0).max(160.0);
        egui::ScrollArea::vertical().max_height(max_h).auto_shrink([false, true]).show(ui, |ui| {
            heading(ui, accent, tr(lang, "Language"));
            ui.horizontal(|ui| {
                ui.radio_value(&mut app.settings.lang, None, tr(lang, "Same as Windows"));
                for l in Lang::ALL {
                    ui.radio_value(&mut app.settings.lang, Some(l), l.name());
                }
            });
            heading(ui, accent, tr(lang, "Appearance"));
            ui.horizontal(|ui| {
                ui.label(tr(lang, "Theme"));
                let before = app.settings.theme;
                ui.radio_value(&mut app.settings.theme, ThemeChoice::System, tr(lang, "System"));
                ui.radio_value(&mut app.settings.theme, ThemeChoice::Light, tr(lang, "Light"));
                ui.radio_value(&mut app.settings.theme, ThemeChoice::Dark, tr(lang, "Dark"));
                // Vulcanio and every app follow the change on the next frame.
                let _ = before;
            });
            heading(ui, accent, tr(lang, "App bar"));
            ui.checkbox(&mut app.settings.rail_autohide, tr(lang, "Hide while an app is open; show when the pointer reaches the left edge"));
            heading(ui, accent, tr(lang, "Memory"));
            ui.checkbox(&mut app.settings.free_idle_apps, tr(lang, "Close apps with nothing open after a minute in the background"));
            heading(ui, accent, tr(lang, "Motion and smoothness"));
            ui.checkbox(&mut app.settings.smooth_scrolling, tr(lang, "Smooth scrolling and zooming"));
            ui.add_enabled_ui(app.settings.smooth_scrolling, |ui| {
                ui.horizontal(|ui| {
                    ui.label(tr(lang, "Ease time"));
                    ui.add(egui::Slider::new(&mut app.settings.ease_ms, 60..=320).suffix(" ms"));
                });
            });
            ui.checkbox(&mut app.settings.animations, tr(lang, "Animated transitions"));
            ui.horizontal(|ui| {
                ui.label(tr(lang, "Presentation"));
            });
            ui.radio_value(&mut app.settings.low_latency, false, tr(lang, "Smooth (VSync, like Microsoft Edge)"));
            ui.radio_value(&mut app.settings.low_latency, true, tr(lang, "Lowest latency (for pen drawing)"));
            ui.label(RichText::new(tr(lang, "Takes effect the next time Vulcanio starts.")).weak().size(12.0));
            heading(ui, accent, tr(lang, "Startup"));
            ui.checkbox(&mut app.settings.reopen_last, tr(lang, "Reopen the last app on start"));
            heading(ui, accent, tr(lang, "Files"));
            if ui.button(tr(lang, "Make Vulcanio the app for these file types")).clicked() {
                app.assoc_note = Some(match crate::associations::register() {
                    Ok(()) => tr(lang, "Registered for the file types Vulcanio opens.").to_string(),
                    Err(e) => e,
                });
            }
            if let Some(note) = &app.assoc_note {
                ui.label(RichText::new(note).weak().size(12.0));
            }
            ui.add_space(4.0);
            ui.label(tr(lang, "Remembered choices"));
            if app.settings.open_with.is_empty() {
                ui.label(RichText::new(tr(lang, "None yet.")).weak());
            }
            let mut forget = None;
            for (ext, id) in &app.settings.open_with {
                ui.horizontal(|ui| {
                    ui.label(format!(".{ext}  →  {}", SPECS[id.index()].title(lang)));
                    if ui.small_button(tr(lang, "Forget")).clicked() {
                        forget = Some(ext.clone());
                    }
                });
            }
            if let Some(ext) = forget {
                app.settings.open_with.remove(&ext);
            }
        });
        ui.add_space(10.0);
        if footer_button(ui, tr(lang, "Close")) {
            close = true;
        }
    });
    if close || modal.should_close() {
        app.show_settings = false;
    }
}

fn heading(ui: &mut egui::Ui, accent: Color32, text: &str) {
    ui.add_space(10.0);
    ui.label(RichText::new(text).size(15.0).strong().color(accent));
    ui.add_space(2.0);
}

/// A dialog's title with a close (×) button at the right. True when the button is clicked.
fn title_bar(ui: &mut egui::Ui, title: &str, close_tip: &str) -> bool {
    let width = ui.available_width();
    let (row, _) = ui.allocate_exact_size(vec2(width, 34.0), egui::Sense::hover());
    let text = ui.visuals().strong_text_color();
    ui.painter().text(row.left_center(), egui::Align2::LEFT_CENTER, title, crate::shell::strong(ui.ctx(), 20.0), text);
    let x = egui::Rect::from_center_size(pos2(row.right() - 16.0, row.center().y), vec2(32.0, 32.0));
    let resp = ui.interact(x, ui.scope_id().with("vulcanio_dialog_x"), egui::Sense::click()).on_hover_text(close_tip);
    let h = ui.ctx().animate_bool_with_time(resp.id.with("h"), resp.hovered(), 0.1);
    if h > 0.0 {
        ui.painter().rect_filled(x, 8.0, ui.visuals().widgets.hovered.bg_fill.gamma_multiply(h));
    }
    let c = x.center();
    let d = 5.5;
    let stroke = egui::Stroke::new(1.6, ui.visuals().text_color());
    ui.painter().line_segment([c + vec2(-d, -d), c + vec2(d, d)], stroke);
    ui.painter().line_segment([c + vec2(-d, d), c + vec2(d, -d)], stroke);
    resp.clicked()
}

/// The primary button at a dialog's bottom right. True when clicked.
fn footer_button(ui: &mut egui::Ui, label: &str) -> bool {
    let width = ui.available_width();
    let (row, _) = ui.allocate_exact_size(vec2(width, 34.0), egui::Sense::hover());
    let galley = ui.painter().layout_no_wrap(label.to_string(), egui::FontId::proportional(14.0), egui::Color32::WHITE);
    let w = galley.size().x + 36.0;
    let r = egui::Rect::from_min_size(pos2(row.right() - w, row.top()), vec2(w, 34.0));
    let resp = ui.interact(r, ui.scope_id().with("vulcanio_dialog_ok"), egui::Sense::click());
    let h = ui.ctx().animate_bool_with_time(resp.id.with("h"), resp.hovered(), 0.1);
    let fill = ui.visuals().selection.bg_fill;
    ui.painter().rect_filled(r, 10.0, fill.linear_multiply(1.0 - 0.15 * h).gamma_multiply(1.0));
    ui.painter().galley(r.center() - galley.size() / 2.0, galley, egui::Color32::WHITE);
    resp.clicked()
}

fn about(app: &mut Vulcanio, ctx: &egui::Context) {
    let lang = app.lang;
    let mut close = false;
    let p = app.palette;
    let modal = egui::Modal::new(Id::unique("vulcanio_about")).show(ctx, |ui| {
        ui.set_width(480.0);
        if title_bar(ui, tr(lang, "About Vulcanio"), tr(lang, "Close")) {
            close = true;
        }
        ui.add_space(6.0);
        ui.horizontal(|ui| {
            if let Some(logo) = &app.logo {
                ui.add(egui::Image::new((logo.id(), vec2(84.0, 84.0))));
            }
            ui.add_space(6.0);
            ui.vertical(|ui| {
                ui.add_space(8.0);
                ui.label(RichText::new("Vulcanio").font(crate::shell::strong(ui.ctx(), 28.0)));
                ui.label(RichText::new(tr(lang, "The all-in-one studio")).size(15.0).color(p.spark));
                ui.label(RichText::new(format!("{} {}", tr(lang, "Version"), env!("CARGO_PKG_VERSION"))).weak());
            });
        });
        ui.add_space(12.0);
        ui.label(tr(lang, "Vulcanio brings six open-source creative apps together in one window."));
        ui.add_space(6.0);
        for s in &SPECS {
            ui.horizontal(|ui| {
                let (r, _) = ui.allocate_exact_size(vec2(18.0, 18.0), egui::Sense::hover());
                icons::paint(ui.painter(), r, &Icon::Module(s.id), s.accent());
                ui.label(s.title(lang));
            });
        }
        ui.add_space(6.0);
        ui.label(RichText::new(tr(lang, "Open-source components under MIT OR Apache-2.0; see NOTICE.md.")).weak().size(12.0));
        ui.add_space(10.0);
        if footer_button(ui, tr(lang, "Close")) {
            close = true;
        }
    });
    if close || modal.should_close() {
        app.show_about = false;
    }
    let _ = modules::all_extensions;
}
