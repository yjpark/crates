pub mod id;
pub mod tree;

pub mod hashmap;

#[cfg(feature = "indexmap")]
pub mod indexmap;

#[cfg(feature = "dashmap")]
pub mod dashmap;

mod macros;
mod impls;

#[cfg(feature = "serde")]
pub mod serde;

pub mod prelude {
    #[doc(hidden)]
    pub use super::id::Identifiable;

    #[doc(hidden)]
    pub use super::tree::Tree;

    #[doc(hidden)]
    pub use super::hashmap::*;

    #[cfg(feature = "indexmap")]
    pub use super::indexmap::*;

    #[cfg(feature = "dashmap")]
    pub use super::dashmap::*;
}
