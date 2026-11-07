//! Krasis' dialogs: Open With, Settings and About.

use egui::{Color32, Id, RichText, vec2};
use krasis_host::Lang;

use crate::i18n::{tr, trf};
use crate::icons::{self, Icon};
use crate::modules::{self, ModuleId, SPECS};
use crate::settings::ThemeChoice;
use crate::shell::{Krasis, file_name};

pub fn show(app: &mut Krasis, ctx: &egui::Context) {
    open_with(app, ctx);
    if app.show_settings {
        settings(app, ctx);
    }
    if app.show_about {
        about(app, ctx);
    }
}

fn open_with(app: &mut Krasis, ctx: &egui::Context) {
    let Some(ow) = &mut app.open_with else { return };
    let lang = app.lang;
    let mut chosen: Option<ModuleId> = None;
    let mut cancel = false;
    let title = if ow.files.len() == 1 {
        trf(lang, "Open {file} with", &[("file", &file_name(&ow.files[0]))])
    } else {
        trf(lang, "Open {file} with", &[("file", &trf(lang, "{n} files", &[("n", &ow.files.len().to_string())]))])
    };
    let modal = egui::Modal::new(Id::unique("krasis_open_with")).show(ctx, |ui| {
        ui.set_width(440.0);
        ui.add_space(4.0);
        ui.label(RichText::new(title).size(18.0).strong());
        ui.add_space(10.0);
        for (i, id) in ow.candidates.iter().enumerate() {
            let spec = &SPECS[id.index()];
            let (rect, resp) = ui.allocate_exact_size(vec2(ui.available_width(), 52.0), egui::Sense::click());
            let h = ui.ctx().animate_bool_with_time(Id::unique(("krasis_ow", i)), resp.hovered(), 0.1);
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
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button(tr(lang, "Cancel")).clicked() {
                cancel = true;
            }
        });
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

fn settings(app: &mut Krasis, ctx: &egui::Context) {
    let lang = app.lang;
    let mut close = false;
    let modal = egui::Modal::new(Id::unique("krasis_settings")).show(ctx, |ui| {
        ui.set_width(520.0);
        ui.label(RichText::new(tr(lang, "Settings")).size(20.0).strong());
        ui.add_space(8.0);
        egui::ScrollArea::vertical().max_height(ctx.content_rect().height() - 180.0).show(ui, |ui| {
            heading(ui, tr(lang, "Language"));
            ui.horizontal(|ui| {
                ui.radio_value(&mut app.settings.lang, None, tr(lang, "Same as Windows"));
                for l in Lang::ALL {
                    ui.radio_value(&mut app.settings.lang, Some(l), l.name());
                }
            });
            heading(ui, tr(lang, "Appearance"));
            ui.horizontal(|ui| {
                ui.label(tr(lang, "Theme"));
                let before = app.settings.theme;
                ui.radio_value(&mut app.settings.theme, ThemeChoice::System, tr(lang, "System"));
                ui.radio_value(&mut app.settings.theme, ThemeChoice::Light, tr(lang, "Light"));
                ui.radio_value(&mut app.settings.theme, ThemeChoice::Dark, tr(lang, "Dark"));
                if before != app.settings.theme {
                    let pref = match app.settings.theme {
                        ThemeChoice::System => egui::ThemePreference::System,
                        ThemeChoice::Light => egui::ThemePreference::Light,
                        ThemeChoice::Dark => egui::ThemePreference::Dark,
                    };
                    if app.view == crate::shell::View::Home {
                        ui.ctx().set_theme(pref);
                    }
                }
            });
            heading(ui, tr(lang, "App bar"));
            ui.checkbox(&mut app.settings.rail_autohide, tr(lang, "Hide while an app is open; show when the pointer reaches the left edge"));
            heading(ui, tr(lang, "Memory"));
            ui.checkbox(&mut app.settings.free_idle_apps, tr(lang, "Close apps with nothing open after a minute in the background"));
            heading(ui, tr(lang, "Motion and smoothness"));
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
            ui.label(RichText::new(tr(lang, "Takes effect the next time Krasis starts.")).weak().size(12.0));
            heading(ui, tr(lang, "Startup"));
            ui.checkbox(&mut app.settings.reopen_last, tr(lang, "Reopen the last app on start"));
            heading(ui, tr(lang, "Files"));
            if ui.button(tr(lang, "Make Krasis the app for these file types")).clicked() {
                app.assoc_note = Some(match crate::associations::register() {
                    Ok(()) => tr(lang, "Registered for the file types Krasis opens.").to_string(),
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
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button(tr(lang, "Close")).clicked() {
                close = true;
            }
        });
    });
    if close || modal.should_close() {
        app.show_settings = false;
    }
}

fn heading(ui: &mut egui::Ui, text: &str) {
    ui.add_space(10.0);
    ui.label(RichText::new(text).size(15.0).strong().color(Color32::from_rgb(0x0A, 0x86, 0x84)));
    ui.add_space(2.0);
}

fn about(app: &mut Krasis, ctx: &egui::Context) {
    let lang = app.lang;
    let mut close = false;
    let modal = egui::Modal::new(Id::unique("krasis_about")).show(ctx, |ui| {
        ui.set_width(460.0);
        ui.horizontal(|ui| {
            if let Some(logo) = &app.logo {
                ui.add(egui::Image::new((logo.id(), vec2(72.0, 72.0))));
            }
            ui.vertical(|ui| {
                ui.add_space(8.0);
                ui.label(RichText::new("Krasis").size(26.0).strong());
                ui.label(RichText::new(tr(lang, "Your creative oasis")).size(15.0).color(Color32::from_rgb(0xC9, 0x9A, 0x3A)));
                ui.label(RichText::new(format!("{} {}", tr(lang, "Version"), env!("CARGO_PKG_VERSION"))).weak());
            });
        });
        ui.add_space(10.0);
        ui.label(tr(lang, "Krasis brings six open-source creative apps together in one window."));
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
        ui.add_space(8.0);
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button(tr(lang, "Close")).clicked() {
                close = true;
            }
        });
    });
    if close || modal.should_close() {
        app.show_about = false;
    }
    let _ = modules::all_extensions;
}
