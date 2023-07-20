pub use figment;

pub mod loader;
pub mod env;

pub mod prelude {
    #[doc(hidden)]
    pub use crate::loader::{
        ConfigLoader, ConfigLoaderBuilder, ConfigLoaderBuilderError
    };

    #[doc(hidden)]
    pub use crate::env::EnvInjector;
}