//! Vulcanio's icons, drawn in code: one per app, plus Home, Settings and Open.

use egui::epaint::{CubicBezierShape, PathStroke};
use egui::{Color32, Painter, Pos2, Rect, Stroke, StrokeKind, pos2, vec2};

use crate::modules::ModuleId;

pub enum Icon {
    Module(ModuleId),
    Home,
    Settings,
    Open,
    Info,
    Sidebar,
    Palette,
    Motion,
    Gauge,
}

/// Paint `icon` centred in `rect` (a square) in `color`.
pub fn paint(p: &Painter, rect: Rect, icon: &Icon, color: Color32) {
    // Whole-pixel size and a pixel-centred middle: strokes stay crisp and centred at any display
    // scale (125%, 150%…).
    let ppp = p.pixels_per_point();
    let s = (rect.width().min(rect.height()) * ppp).round() / ppp;
    let c = pos2(p.round_to_pixel_center(rect.center().x), p.round_to_pixel_center(rect.center().y));
    let w = (s * 0.085).max(1.3);
    let st = Stroke::new(w, color);
    let at = |x: f32, y: f32| pos2(c.x + x * s, c.y + y * s);
    match icon {
        Icon::Module(ModuleId::Photo) => {
            let r = Rect::from_min_max(at(-0.38, -0.3), at(0.38, 0.3));
            p.rect_stroke(r, s * 0.08, st, StrokeKind::Middle);
            p.add(egui::Shape::line(vec![at(-0.3, 0.22), at(-0.08, -0.02), at(0.06, 0.12), at(0.16, 0.02), at(0.3, 0.2)], st));
            p.circle_filled(at(0.17, -0.13), s * 0.065, color);
        }
        Icon::Module(ModuleId::Light) => {
            p.circle_stroke(c, s * 0.38, st);
            // Six aperture blades.
            for i in 0..6 {
                let a = i as f32 * std::f32::consts::TAU / 6.0;
                let b = a + std::f32::consts::TAU / 6.0;
                let inner = |t: f32| c + vec2(t.cos(), t.sin()) * s * 0.16;
                let outer = |t: f32| c + vec2((t + 0.55).cos(), (t + 0.55).sin()) * s * 0.38;
                p.line_segment([inner(a), outer(a)], Stroke::new(w * 0.8, color));
                p.line_segment([inner(a), inner(b)], Stroke::new(w * 0.8, color));
            }
        }
        Icon::Module(ModuleId::Vector) => {
            let (a, b) = (at(-0.36, 0.28), at(0.36, -0.24));
            p.add(CubicBezierShape::from_points_stroke([a, at(-0.3, -0.28), at(0.3, 0.32), b], false, Color32::TRANSPARENT, PathStroke::from(st)));
            for q in [a, b] {
                p.rect_filled(Rect::from_center_size(q, vec2(s * 0.14, s * 0.14)), 1.0, color);
            }
            p.line_segment([at(-0.3, -0.28), a], Stroke::new(w * 0.6, color.gamma_multiply(0.7)));
            p.circle_filled(at(-0.3, -0.28), s * 0.05, color);
        }
        Icon::Module(ModuleId::Design) => {
            let page = Rect::from_min_max(at(-0.3, -0.38), at(0.3, 0.38));
            p.rect_stroke(page, s * 0.05, st, StrokeKind::Middle);
            p.rect_filled(Rect::from_min_max(at(-0.2, -0.27), at(0.2, -0.08)), 1.5, color);
            for (i, y) in [0.02f32, 0.12, 0.22].iter().enumerate() {
                let right = if i == 2 { 0.05 } else { 0.2 };
                p.line_segment([at(-0.2, *y), at(right, *y)], Stroke::new(w * 0.8, color));
            }
        }
        Icon::Module(ModuleId::Fig) => {
            let g = s * 0.075;
            let cell = |x: f32, y: f32| Rect::from_min_size(at(x, y), vec2(s * 0.3, s * 0.3));
            p.rect_stroke(cell(-0.36, -0.36), g, st, StrokeKind::Middle);
            p.rect_stroke(cell(0.06, -0.36), g, st, StrokeKind::Middle);
            p.rect_stroke(cell(-0.36, 0.06), g, st, StrokeKind::Middle);
            // A component diamond.
            let d = at(0.21, 0.21);
            let r = s * 0.17;
            p.add(egui::Shape::convex_polygon(vec![d + vec2(0.0, -r), d + vec2(r, 0.0), d + vec2(0.0, r), d + vec2(-r, 0.0)], color, Stroke::NONE));
        }
        Icon::Module(ModuleId::Cad) => {
            p.circle_stroke(c, s * 0.25, st);
            p.line_segment([at(-0.42, 0.0), at(0.42, 0.0)], Stroke::new(w * 0.7, color));
            p.line_segment([at(0.0, -0.42), at(0.0, 0.42)], Stroke::new(w * 0.7, color));
            p.rect_stroke(Rect::from_center_size(c, vec2(s * 0.12, s * 0.12)), 0.0, Stroke::new(w * 0.7, color), StrokeKind::Middle);
        }
        Icon::Home => {
            p.add(egui::Shape::line(vec![at(-0.36, 0.0), at(0.0, -0.34), at(0.36, 0.0)], st));
            p.add(egui::Shape::line(vec![at(-0.26, -0.08), at(-0.26, 0.34), at(0.26, 0.34), at(0.26, -0.08)], st));
            p.rect_filled(Rect::from_min_max(at(-0.07, 0.12), at(0.07, 0.34)), 1.0, color);
        }
        Icon::Settings => {
            let r_out = s * 0.36;
            let r_in = s * 0.26;
            let mut pts: Vec<Pos2> = Vec::new();
            for i in 0..32 {
                let a = i as f32 * std::f32::consts::TAU / 32.0;
                let r = if (i / 2) % 2 == 0 { r_out } else { r_in };
                pts.push(c + vec2(a.cos(), a.sin()) * r);
            }
            pts.push(pts[0]);
            p.add(egui::Shape::line(pts, st));
            p.circle_stroke(c, s * 0.11, st);
        }
        Icon::Open => {
            p.add(egui::Shape::line(vec![at(-0.38, 0.3), at(-0.38, -0.28), at(-0.12, -0.28), at(-0.04, -0.18), at(0.3, -0.18), at(0.3, -0.04)], st));
            p.add(egui::Shape::line(vec![at(-0.38, 0.3), at(-0.24, -0.04), at(0.4, -0.04), at(0.26, 0.3), at(-0.38, 0.3)], st));
        }
        Icon::Sidebar => {
            let r = Rect::from_min_max(at(-0.38, -0.3), at(0.38, 0.3));
            p.rect_stroke(r, s * 0.07, st, StrokeKind::Middle);
            p.rect_filled(Rect::from_min_max(at(-0.38, -0.3), at(-0.12, 0.3)), s * 0.05, color);
        }
        Icon::Palette => {
            // A painter's palette: a rounded body with a thumb hole and three paint dots.
            p.circle_stroke(c, s * 0.36, st);
            p.circle_filled(at(0.14, 0.16), s * 0.07, color);
            for (x, y) in [(-0.16f32, -0.12f32), (0.04, -0.2), (-0.2, 0.1)] {
                p.circle_filled(at(x, y), s * 0.06, color);
            }
        }
        Icon::Motion => {
            // Speed lines and a moving dot.
            for (i, y) in [-0.2f32, 0.0, 0.2].iter().enumerate() {
                let start = [-0.38f32, -0.3, -0.38][i];
                p.line_segment([at(start, *y), at(0.06, *y)], Stroke::new(w, color.gamma_multiply(1.0 - i as f32 * 0.0)));
            }
            p.circle_filled(at(0.24, 0.0), s * 0.13, color);
        }
        Icon::Gauge => {
            // A dial with a needle.
            let pts: Vec<Pos2> = (0..=16)
                .map(|i| {
                    let a = std::f32::consts::PI * (1.0 + i as f32 / 16.0);
                    c + vec2(a.cos(), a.sin()) * s * 0.38 + vec2(0.0, s * 0.12)
                })
                .collect();
            p.add(egui::Shape::line(pts, st));
            let hub = c + vec2(0.0, s * 0.12);
            p.line_segment([hub, at(0.2, -0.12)], st);
            p.circle_filled(hub, s * 0.06, color);
        }
        Icon::Info => {
            p.circle_stroke(c, s * 0.38, st);
            p.circle_filled(at(0.0, -0.16), s * 0.05, color);
            p.line_segment([at(0.0, -0.04), at(0.0, 0.2)], st);
        }
    }
}
