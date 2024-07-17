#[cfg(feature = "wasm")]
mod npm_registry;
#[cfg(feature = "wasm")]
mod package_manager;
#[cfg(feature = "wasm")]
mod proto;

#[cfg(feature = "wasm")]
pub use proto::*;
