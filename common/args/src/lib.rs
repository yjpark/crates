pub use clap;

pub mod path;
pub mod verbose;

pub mod prelude {
    #[doc(hidden)]
    pub use crate::path::{PathArg, path_arg};

    #[doc(hidden)]
    pub use crate::verbose::VerboseArg;
}