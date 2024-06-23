#![warn(missing_docs)]

//! Crate containing shared structs between plugin and ide.

mod serverbound_message;
mod pluginbound_message;
mod common;
#[cfg(target_family = "wasm")]
mod plugin_main;

pub use serverbound_message::*;
pub use pluginbound_message::*;
pub use common::*;