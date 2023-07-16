pub use bevy;

// https://github.com/bevyengine/bevy/issues/3659
pub mod bevy_prelude {
    pub use bevy::prelude::*;
    pub use bevy::ecs as bevy_ecs;
    pub use bevy::reflect as bevy_reflect;
}

pub use edger_bevy_util;

#[cfg(feature = "shape")]
pub use edger_bevy_shape;

#[cfg(feature = "shape")]
pub use edger_bevy_shape::bevy_prototype_lyon;

#[cfg(feature = "view")]
pub use edger_bevy_view;

#[cfg(feature = "egui")]
pub use edger_bevy_egui;

#[cfg(feature = "egui")]
pub use edger_bevy_egui::bevy_egui;

#[cfg(feature = "egui")]
pub use edger_bevy_egui::egui;

pub mod prelude {
    #[doc(hidden)]
    pub use edger_bevy_util::prelude::*;

    #[cfg(feature = "shape")]
    #[doc(hidden)]
    pub use edger_bevy_shape::prelude::*;

    #[cfg(feature = "view")]
    #[doc(hidden)]
    pub use edger_bevy_view::prelude::*;

    #[cfg(feature = "egui")]
    #[doc(hidden)]
    pub use edger_bevy_egui::prelude::*;
}
