use egui::{Color32, Pos2};

pub const DEEP_CHARCOAL: Color32 = Color32::from_rgb(29, 29, 27);
pub const RUST_ORANGE: Color32 = Color32::from_rgb(206, 65, 43);
pub const BURNT_SIENNA: Color32 = Color32::from_rgb(167, 33, 69);
pub const ALMOND_OFF_WHITE: Color32 = Color32::from_rgb(245, 225, 200);
pub const STEEL_GRAY: Color32 = Color32::from_rgb(78, 78, 78);
pub const AMBER_GLOW: Color32 = Color32::from_rgb(222, 165, 132);

pub const MAX_RADIUS: f32 = 30.0;

// data untuk brush nya
// !! : data ini akan di simpan setiap circle nya , jadi setiap data struct memiliki kehiudupan yg
// berbeda
#[derive(Debug, Clone, Copy)]
pub struct brush {
    pub kordiant: Pos2, // simpan koorniat (x  y)
    pub color: Color32,
    pub radius: Option<f32>,
}

impl brush {
    pub fn new(pos: Pos2, color: Color32, radius: f32) -> Self {
        Self {
            kordiant: pos,
            color,
            radius: Some(radius),
        }
    }
}

// fn untuk cek jika koordiant bertemu pada titik yang sama , maka ganti ke baru
// dgn harapan agar tidak banyak frame yang bertumpuk
// brush_all : daftar semua brush yang di simpan
// return : true jika koordinat bertabrakan atau sama
pub fn update_brush_draw(
    kordinat_now: Pos2,
    color_now: Color32,
    brush_all: &mut Vec<brush>,
) -> bool {
    // , polem  jika terlalu banyak , update lagi dengan algo sercing
    if let Some(br) = brush_all.iter_mut().find(|br| br.kordiant == kordinat_now) {
        // update ,color , dan any untuk koordiant ini
        br.color = color_now;
        return true;
    }

    false
}
