// tujuannya menyembunyikan console window | saat realease di windows
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod components;

mod body;
mod core;
mod event;
mod header;
mod sidebar;

use eframe::{App, egui};
use egui::Color32;

use crate::core::brush;

const WINDOWSIZE: [f32; 2] = [520.0, 350.0];

#[derive(Debug)]
struct PaintApp {
    // poblem , jika color berubah , maka setiap draw_koordiant kan berubah
    // draw_koordiant: Vec<Pos2>,
    // brush_color: Color32,

    // solusi : brus punya data sendiri
    brushs: Vec<brush>,
    curr_color: Color32,
    last_brush_index: Vec<usize>, // dftar index mouse saat mulai menggambar
    stack_data_redo: Vec<Vec<brush>>, // ini data dari undo yang di simpan di sini sementara
    curr_radius: f32,
}

impl App for PaintApp {
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        println!("aplikasi keluar ");
    }
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // header di maan tools seperti pecil , save ,  redo , dll di ada di atasnya
        header::header(&ctx, &mut self.curr_color);

        // body tempat menggambar di lakukan
        body::body(
            &mut self.brushs,
            self.curr_color,
            &mut self.curr_radius,
            &mut self.last_brush_index,
            &mut self.stack_data_redo,
            &ctx,
        );
    }
    // fn save(&mut self, _storage: &mut dyn eframe::Storage) {
    //
    // }
}

impl Default for PaintApp {
    fn default() -> Self {
        Self {
            brushs: Vec::new(),
            curr_color: Color32::default(),
            curr_radius: 3.2, // default ketebalan line
            last_brush_index: Vec::new(),
            stack_data_redo: Vec::new(),
        }
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(WINDOWSIZE)
            .with_title_shown(true)
            .with_drag_and_drop(true)
            .with_title("Paint"),

        // field lain default aja dulu
        ..Default::default()
    };

    // run
    eframe::run_native(
        "Paint",
        options,
        Box::new(|cc| {
            //loader untuk gambar
            // egui_extra

            Ok(Box::<PaintApp>::default())
        }),
    )
}
