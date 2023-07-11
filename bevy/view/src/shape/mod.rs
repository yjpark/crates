pub mod view;
pub mod dev;

pub mod prelude {
    #[doc(hidden)]
    pub use edger_bevy_shape::prelude::*;

    #[doc(hidden)]
    pub use super::view::color_background::ColorBackground;

    #[doc(hidden)]
    pub use super::dev::theme::ViewShapeDevTheme;

    #[doc(hidden)]
    pub use super::dev::plugin::ViewShapeDevPlugin;
}