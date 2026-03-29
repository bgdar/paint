use egui::{Atom, Color32, InnerResponse, Margin, Shadow, Stroke, Ui, Widget};


// masih ada bebrapa perbaikan lagi ini

// card default dengan border
// content : adalah cousur function , untuk menambah ui secara bebad
pub fn card_default(
    ui: &mut Ui,
    content: impl FnOnce(&mut Ui), // clouser
    color: Option<Color32>,
) -> InnerResponse<()> {
    egui::containers::Frame::new()
        
        .fill(color.unwrap_or(Color32::TRANSPARENT))
        .stroke(Stroke::new(1.2, Color32::GRAY))
        .corner_radius(15.0)
        // .shadow(Shadow::default())
        .inner_margin(Margin::same(10))
        .show(ui, |ui| 

            // ui.set_max_size()
            content(ui))
}

// card_default(ui, None, |ui| {
//     ui.label("Ini isi card"); // bebas isi apa saja
//     ui.button("Klik"); // bisa banyak widget
// });
