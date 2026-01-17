pub mod java_junit;
pub mod java_junit4;
pub mod registry;
pub mod rust_native;
pub mod traits;

pub use registry::TemplateRegistry;
pub use traits::{TemplateContext, TemplateGenerator};
