pub use askama;

pub mod writer;

pub mod prelude {
    #[doc(hidden)]
    pub use crate::writer::{
        TemplateWriter, TemplateWriterBuilder, TemplateWriterBuilderError
    };

}