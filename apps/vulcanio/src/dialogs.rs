//! Vulcanio's Open With dialog (Settings and About live in `settings_ui`).

use egui::{Id, pos2, vec2};

use crate::i18n::{tr, trf};
use crate::icons::{self, Icon};
use crate::modules::{ModuleId, SPECS};
use crate::shell::{Vulcanio, file_name};

pub fn show(app: &mut Vulcanio, ctx: &egui::Context) {
    open_with(app, ctx);
    // About is a page of the Settings window.
    if app.show_about {
        app.show_about = false;
        app.show_settings = true;
        app.settings_page = crate::settings_ui::Page::About;
    }
    if app.show_settings {
        crate::settings_ui::show(app, ctx);
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
