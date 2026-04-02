use egui::{
    Color32, Event, Id, Key, KeyboardShortcut, Modifiers, MouseWheelUnit, Order, PointerButton,
    Pos2, Stroke,
};

use crate::core::{MAX_RADIUS, brush, update_brush_draw};

// body utama yang di bagian tegah tempat paint atau menggambar di lakukan

const SAVE: KeyboardShortcut = KeyboardShortcut::new(Modifiers::CTRL, Key::S);
const UNDO: KeyboardShortcut = KeyboardShortcut::new(Modifiers::CTRL, Key::Z);
const REDO: KeyboardShortcut = KeyboardShortcut::new(Modifiers::CTRL, Key::R);

pub fn body(
    brush_content: &mut Vec<brush>,
    curr_color: Color32,
    curr_radius: &mut f32,
    last_brush_index: &mut Vec<usize>,
    stack_data_redo: &mut Vec<Vec<brush>>,
    ctx: &egui::Context,
) -> egui::InnerResponse<()> {
    egui::CentralPanel::default().show(ctx, |ui| {
        let panel_rect = ui.min_rect(); // Rect dari panel ini

        ctx.input(|i| {
            let pointer = &i.pointer;
            let events = &i.events;
            // let modifier = &i.modifiers; // key Event (  KeyboardShortcut )

            let scroll_mouse = pointer.hover_pos();
            // let scroll_delta = i.smooth_scroll_delta;

            // SECTION Mouse
            // jika di panel ini dan jika mause kiri di click dan jika mouse bergeral
            if let Some(pos) = scroll_mouse {
                if panel_rect.contains(pos) {
                    // click 1x
                    if pointer.button_pressed(PointerButton::Primary) {
                        // simpan posis akhir mouse untuk cnt + Z
                        if let Some(last_index) = brush_content.len().checked_sub(1) {
                            // checked_sub = len -1
                            last_brush_index.push(last_index);
                        }
                        stack_data_redo.clear(); //bersihkan data yang di simpan  jika user lanjut
                        //menggambar atau tidak kembali ke titik awal setelah REDO
                    }
                    if pointer.button_down(PointerButton::Primary) && pointer.is_moving() {
                        // draw_koordiant.push(pos);
                        // brush_now.kordiant = pos;

                        if !update_brush_draw(pos, curr_color, brush_content) {
                            // simpan langsung Struct brush baru berdasarkan data
                            brush_content.push(brush::new(pos, curr_color, curr_radius.clone()));
                        }
                        // } else if pointer.button_released { }
                    }
                }
            }

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
                            *curr_radius = curr_radius.clamp(2.0, MAX_RADIUS)
                        }
                        _ => {}
                    }
                }
            }
        });
        ctx.input_mut(|i| {
            // SECTION KEY
            if i.consume_shortcut(&UNDO) {
                // redo
                // truncate  = memotong dari suatu index sampai last index
                println!("cnt  + z execu");
                if !last_brush_index.is_empty() {
                    if let Some(last_brush_index) = last_brush_index.pop() {
                        // brush_content.truncate(last_brush_index);
                        let get_data_redo = brush_content.split_off(last_brush_index); // ambil data sampan akhir 
                        // last_data_redo.append(&mut stack_data_redo); // simpan langsung setiap data akhirnya
                        println!("get data redo yg di dapat  : {}", get_data_redo.len());
                        stack_data_redo.push(get_data_redo);
                    }
                }
            } else if i.consume_shortcut(&REDO) {
                // update kembali data yang sudah di simpan ke akhir element brush_content

                println!("cnt  + r execu");
                println!("Pnjangg redo data : {}", stack_data_redo.len());
                if let Some(mut last_stack_data) = stack_data_redo.pop() {
                    println!("last data di tambhakan");
                    brush_content.append(&mut last_stack_data);

                    // set kembali panjang datanya
                    let last_index = brush_content.len() - 1; // sudah append seluruh batch
                    last_brush_index.push(last_index);
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
