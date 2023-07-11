use bevy::prelude::*;

use bevy_egui::egui::ecolor::Hsva;

pub fn rgb_to_egui(color: &Color) -> Hsva {
    let v = color.as_linear_rgba_f32();
    Hsva::from_rgb([v[0], v[1], v[2]])
}
