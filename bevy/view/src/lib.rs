pub use edger_bevy_util;

#[cfg(feature = "shape")]
pub use edger_bevy_shape;

pub mod bundle;
pub mod layout;

#[cfg(feature = "shape")]
pub mod shape;

pub mod prelude {
    #[doc(hidden)]
    pub use edger_bevy_util::prelude::*;

    #[doc(hidden)]
    pub use crate::bundle::view_bundle::ViewBundle;
    #[doc(hidden)]
    pub use crate::layout::anchor::{LayoutAnchor, LayoutHAnchor, LayoutVAnchor};
    #[doc(hidden)]
    pub use crate::layout::data::{LayoutConstraint, LayoutData, LayoutSize};
    #[doc(hidden)]
    pub use crate::layout::dock::{DockPanel, DockSide, DockView};
    #[doc(hidden)]
    pub use crate::layout::grid::{GridCell, GridCellData, GridCellSize, GridData, GridView};
    #[doc(hidden)]
    pub use crate::layout::vbox::{VBoxCell, VBoxCellData, VBoxView};
    #[doc(hidden)]
    pub use crate::layout::view::{
        DoLayoutEvent, LayoutChangedQuery, LayoutChangedWithChildrenQuery, LayoutEnv, LayoutQuery,
        View, ViewAddedQuery, ViewEntity, ViewQuery, ViewRootAddedQuery, ViewRootQuery,
    };

    #[cfg(feature = "shape")]
    #[doc(hidden)]
    pub use crate::shape::prelude::*;
}
