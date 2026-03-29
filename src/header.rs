use egui::{
    Align, Atom, Button, Color32, Frame, Grid, InnerResponse, Layout, Response, Ui,
    epaint::ColorMode, hex_color, widgets,
};

use crate::{
    components::card,
    core::{AMBER_GLOW, STEEL_GRAY},
};

// Daftar 10 warna dasar (Hex ke Color32)
const PALETTE: [Color32; 10] = [
    hex_color!("#1D1D1B"), // Deep Charcoal (Base)
    hex_color!("#CE412B"), // Rust Orange (Primary)
    hex_color!("#A72145"), // Burnt Sienna (Accent)
    hex_color!("#4E4E4E"), // Steel Gray (Secondary)
    hex_color!("#DEA584"), // Amber Glow (Highlight)
    hex_color!("#F5E1C8"), // Almond (Contrast)
    hex_color!("#7B2D26"), // Deep Rust
    hex_color!("#2E2E2E"), // Lighter Charcoal
    hex_color!("#8C3321"), // Auburn
    hex_color!("#5C5C5C"), // Medium Gray
];

pub fn header(ctx: &egui::Context, brush_color: &mut Color32) -> egui::InnerResponse<()> {
    egui::TopBottomPanel::top("top")
        .max_height(75.0)
        .frame(Frame::new().fill(AMBER_GLOW))
        .resizable(false)
        .show(ctx, |ui| {
            // egui::CentralPanel::default().show_inside(ui, |ui| {
            //     ui.label("Nested panel");
            //     ui.heading("Header")
            // });

            // component Grid
            Grid::new("component").show(ui, |ui| {
                ui.label("Paint");
                // di sini tombol warna
                card::card_default(
                    ui,
                    |ui| {
                        ui.label("Pallet Warna");
                        ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                            for hex in PALETTE {
                                buttom_color(ui, hex, brush_color);
                            }
                        });
                    },
                    None,
                );

                // tempat brush atau alat , dan lainya di sini
                card::card_default(
                    ui,
                    |ui| {
                        ui.label("brush");
                    },
                    Some(STEEL_GRAY),
                );
                ui.end_row();
            });
            //         let area = egui::Area::new(egui::Id::new("test area")).fixed_pos(egui::pos2(30.0, 30.0));
            // let mut frame = egui::Frame::default().inner_margin(4.0);
            //         let modal = egui::containers::modal::Modal {
            //             area: area,
            // backdrop_color : egui::Color32::DARK_BLUE,frame : Option<frame>
            //         };
        })
}

// mengembalikan
fn buttom_color(ui: &mut Ui, color: Color32, brush_color: &mut Color32) {
    let btn = widgets::Button::new(Atom::default())
        .fill(color)
        .frame(true)
        .corner_radius(4.0)
        .min_size(egui::vec2(15.0, 9.0));

    if ui.add(btn).clicked() {
        // let hex = color.to_hex();
        // brush_color.clear();
        // brush_color.push_str(&hex);
        // *brush_color = hex; //ini lebih efesient sih
        println!("color {} clicked", color.to_hex());

        *brush_color = color;
    }
}
