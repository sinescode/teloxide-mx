//! Some useful utilities.

pub mod callback_data;
pub mod command;
pub mod html;
pub mod keyboard;
pub mod markdown;
pub mod render;
pub(crate) mod shutdown_token;

pub use teloxide_core::net::client_from_env;
