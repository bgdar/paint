use eframe::glow::Context;
use egui::{InnerResponse, SidePanel};

// sidebar rencanya menggunakan panel di sebelah kiri
pub fn sidebar(ctx: &egui::Context) -> InnerResponse<()> {
    egui::SidePanel::right("sidebar").show(ctx, |ui| {})
}
