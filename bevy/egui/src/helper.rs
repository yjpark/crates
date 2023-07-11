use bevy_egui::egui::*;

use crate::prelude::easy_mark_viewer;
use crate::prelude::EasyMarkStyle;

pub fn label_from_style(text: &str, style: &EasyMarkStyle) -> Label {
    Label::new(easy_mark_viewer::rich_text_from_style(text, &style))
}

pub fn link_from_style(text: &str, style: &EasyMarkStyle, ui: &Ui) -> Label {
    let color = ui.visuals().hyperlink_color;
    Label::new(easy_mark_viewer::rich_text_from_style(text, &style).color(color))
        .sense(Sense::click())
}
