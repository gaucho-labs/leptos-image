#![forbid(unsafe_code)]

mod image;
mod optimizer;
mod provider;
mod routes;

pub use image::*;
#[cfg(feature = "ssr")]
pub use optimizer::ImageOptimizer;
pub use provider::*;
#[cfg(feature = "ssr")]
pub use routes::handlers::*;
