//! Some useful utilities.

pub mod callback_answer;
pub mod callback_data;
pub mod chat_action;
pub mod command;
pub mod deep_linking;
pub mod filters;
pub mod formatting;
pub mod html;
pub mod i18n;
pub mod keyboard;
pub mod magic_filter;
pub mod markdown;
pub mod media_group;
pub mod render;
pub(crate) mod shutdown_token;
pub mod web_app;
pub mod webhook_security;

pub use teloxide_core::net::client_from_env;
