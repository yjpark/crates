pub mod misc;
pub mod asset;
pub mod bundle;
pub mod plugin;

pub mod prelude {
    #[doc(hidden)]
    pub use crate::misc::math;
    #[doc(hidden)]
    pub use crate::misc::entity;
    #[doc(hidden)]
    pub use crate::misc::asset;
    #[doc(hidden)]
    pub use crate::misc::text;

    #[doc(hidden)]
    pub use crate::asset::markdown_asset::MarkDownAsset;

    #[doc(hidden)]
    pub use crate::bundle::single_bundle::{SingleBundle, SingleData};

    #[doc(hidden)]
    pub use crate::plugin::UtilsPlugin;
}