pub use bevy;

#[cfg(feature = "shape")]
pub use edger_bevy_shape

#[cfg(feature = "view")]
pub use edger_bevy_view

#[cfg(feature = "egui")]
pub use edger_bevy_egui

pub mod prelude {
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
