pub mod tracing;

pub use edger_args;

#[cfg(feature = "config")]
pub use edger_config;

#[cfg(feature = "template")]
pub use edger_template;

pub mod prelude {
    #[doc(hidden)]
    pub use edger_args::prelude::*;

    #[cfg(feature = "config")]
    #[doc(hidden)]
    pub use edger_config::prelude::*;

    #[cfg(feature = "template")]
    #[doc(hidden)]
    pub use edger_template::prelude::*;

    #[doc(hidden)]
    pub use crate::tracing::init as init_tracing;
}
