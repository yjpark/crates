use bevy::prelude::*;
use bevy_egui::egui::*;
use bevy_egui::egui::Label;
use bevy_egui::egui::ecolor::Hsva;

use crate::prelude::easy_mark_viewer;
use crate::prelude::EasyMarkStyle;

pub fn color_to_hsva(color: &Color) -> Hsva {
    let v = color.as_linear_rgba_f32();
    Hsva::from_rgb([v[0], v[1], v[2]])
}

pub fn label_from_style(text: &str, style: &EasyMarkStyle) -> Label {
    Label::new(easy_mark_viewer::rich_text_from_style(text, &style))
}

pub fn link_from_style(text: &str, style: &EasyMarkStyle, ui: &Ui) -> Label {
    let color = ui.visuals().hyperlink_color;
    Label::new(easy_mark_viewer::rich_text_from_style(text, &style).color(color))
        .sense(Sense::click())
}
