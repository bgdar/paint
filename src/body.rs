use egui::{Color32, Event, Id, Key, MouseWheelUnit, Order, PointerButton, Pos2, Stroke};

use crate::core::{MAX_RADIUS, brush, update_brush_draw};

// body utama yang di bagian tegah tempat paint atau menggambar di lakukan

pub fn body(
    brush_content: &mut Vec<brush>,
    curr_color: Color32,
    curr_radius: &mut f32,
    ctx: &egui::Context,
) -> egui::InnerResponse<()> {
    egui::CentralPanel::default().show(ctx, |ui| {
        let panel_rect = ui.min_rect(); // Rect dari panel ini

        ctx.input(|i| {
            let pointer = &i.pointer;
            let events = &i.events;

            let scroll_mouse = pointer.hover_pos();
            // let scroll_delta = i.smooth_scroll_delta;

            let mut brush_now = brush::new();
            brush_now.color = curr_color;
            brush_now.radius = Some(curr_radius.clone());

            // jika di panel ini dan jika mause kiri di click dan jika mouse bergeral
            if let Some(pos) = scroll_mouse {
                if panel_rect.contains(pos)
                    && pointer.button_down(PointerButton::Primary)
                    && pointer.is_moving()
                {
                    // draw_koordiant.push(pos);
                    brush_now.kordiant = pos;

                    if !update_brush_draw(pos, curr_color, brush_content) {
                        brush_content.push(brush_now); // pindahkan owner brush data baru ke sini
                    }
                }
            }

            // SECTION KEY
            // if i.key_pressed(Key::) {
            // todo!();
            // }

            // SECTION EVENT
            // saat roda mouse di geser ke atas atau ke bawah
            for event in events {
                if let Event::MouseWheel {
                    unit,
                    delta,
                    modifiers,
                } = event
                {
                    match unit {
                        MouseWheelUnit::Line | MouseWheelUnit::Point => {
                            // brush_now.radius = delta.y;
                            // Cek apakah scroll ke atas (delta.y > 0)

                            *curr_radius += delta.y * 0.8;
                            // biar gak ada di bawah 0
                            *curr_radius = curr_radius.clamp(1.0, MAX_RADIUS)
                        }
                        _ => {}
                    }
                }
            }
        });

        // gambar circle berdasarkan data brush yang di siman
        for br in brush_content {
            if let Some(radius) = br.radius {
                line_art(&ctx, br.kordiant, &br.color, radius);
            }
        }
    })
}

// DRAWER

/// Linkaram
fn line_art(ctx: &egui::Context, pos: Pos2, color: &Color32, radius: f32) {
    let painter = ctx.layer_painter(egui::LayerId::new(Order::Foreground, Id::new("circle")));
    painter.circle(pos, radius, *color, Stroke::NONE);
    // painter
}

// circle untuk di tampilkan di layar
// fn circle(ctx: &egui::Context ,pos: Pos2 ,color: &Color32) {
//
// }
