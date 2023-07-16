pub use bevy_egui;
pub use bevy_egui::egui;

pub mod easy_link;
pub mod helper;

mod easy_mark;
pub use crate::easy_mark::easy_mark_parser;
pub use crate::easy_mark::easy_mark_viewer;

pub mod prelude {
    #[doc(hidden)]
    pub use bevy_egui::EguiContexts;

    #[doc(hidden)]
    pub use crate::easy_mark_parser;
    #[doc(hidden)]
    pub use crate::easy_mark_viewer;
    #[doc(hidden)]
    pub use crate::easy_mark_viewer::easy_mark;
    #[doc(hidden)]
    pub use crate::easy_mark::easy_mark_parser::Style as EasyMarkStyle;

    #[doc(hidden)]
    pub use crate::easy_link::{EasyLink, EasyLinkEvent};

    #[doc(hidden)]
    pub use crate::helper::{color_to_hsva, label_from_style, link_from_style};
}