//! Some useful utilities.

pub mod auth_widget;
pub mod callback_answer;
pub mod callback_data;
pub mod chat_action;
pub mod chat_member_updated;
pub mod command;
pub mod command_start;
pub mod content_type;
pub mod deep_linking;
pub mod filters;
pub mod flags;
pub mod formatting;
pub mod html;
pub mod i18n;
pub mod keyboard;
pub mod lazy_i18n;
pub mod link;
pub mod magic_data;
pub mod magic_filter;
pub mod markdown;
pub mod media_group;
pub mod render;
pub(crate) mod shutdown_token;
pub mod token;
pub mod web_app;
pub mod web_app_signature;
pub mod webhook_security;

pub use teloxide_max_core::net::client_from_env;
