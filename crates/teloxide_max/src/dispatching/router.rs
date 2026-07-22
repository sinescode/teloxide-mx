//! Modular router system for organizing handlers.
//!
//! Similar to aiogram's `Router`, this module allows splitting handler logic
//! into separate modules that can be composed together.
//!
//! # Example
//!
//! ```rust,no_run
//! # use teloxide_max::prelude::*;
//! # use teloxide_max::dispatching::router::Router;
//!
//! # #[tokio::main]
//! # async fn main() {
//! let mut admin_router = Router::new("admin");
//! admin_router.add_handler(
//!     Update::filter_message().filter_command::<AdminCommand>().endpoint(handle_admin_command),
//! );
//!
//! let mut user_router = Router::new("user");
//! user_router.add_handler(
//!     Update::filter_message().filter_command::<UserCommand>().endpoint(handle_user_command),
//! );
//!
//! // Compose routers into a handler tree
//! let schema = Router::compose(vec![admin_router, user_router]);
//! # }
//! ```

use crate::{
    dispatching::{UpdateFilterExt, UpdateHandler},
    types::Update,
};
use std::fmt;

type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
type BoxedHandler = UpdateHandler<HandlerResult>;

/// A modular router that organizes handlers by event type.
///
/// Routers can be nested and merged to build complex handler trees
/// while keeping code organized.
pub struct Router {
    name: String,
    handlers: Vec<BoxedHandler>,
    sub_routers: Vec<Router>,
}

impl Router {
    /// Creates a new router with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), handlers: Vec::new(), sub_routers: Vec::new() }
    }

    /// Returns the router's name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Adds a pre-built handler branch to this router.
    pub fn add_handler(&mut self, handler: BoxedHandler) {
        self.handlers.push(handler);
    }

    /// Adds a message handler branch.
    pub fn add_message_handler(&mut self, handler: BoxedHandler) {
        self.handlers.push(Update::filter_message().branch(handler));
    }

    /// Adds a callback query handler branch.
    pub fn add_callback_handler(&mut self, handler: BoxedHandler) {
        self.handlers.push(Update::filter_callback_query().branch(handler));
    }

    /// Adds an inline query handler branch.
    pub fn add_inline_handler(&mut self, handler: BoxedHandler) {
        self.handlers.push(Update::filter_inline_query().branch(handler));
    }

    /// Adds a chosen inline result handler branch.
    pub fn add_chosen_inline_handler(&mut self, handler: BoxedHandler) {
        self.handlers.push(Update::filter_chosen_inline_result().branch(handler));
    }

    /// Adds a shipping query handler branch.
    pub fn add_shipping_handler(&mut self, handler: BoxedHandler) {
        self.handlers.push(Update::filter_shipping_query().branch(handler));
    }

    /// Adds a pre-checkout query handler branch.
    pub fn add_pre_checkout_handler(&mut self, handler: BoxedHandler) {
        self.handlers.push(Update::filter_pre_checkout_query().branch(handler));
    }

    /// Adds a channel post handler branch.
    pub fn add_channel_post_handler(&mut self, handler: BoxedHandler) {
        self.handlers.push(Update::filter_channel_post().branch(handler));
    }

    /// Adds an edited message handler branch.
    pub fn add_edited_message_handler(&mut self, handler: BoxedHandler) {
        self.handlers.push(Update::filter_edited_message().branch(handler));
    }

    /// Adds a message reaction handler branch.
    pub fn add_message_reaction_handler(&mut self, handler: BoxedHandler) {
        self.handlers.push(Update::filter_message_reaction_updated().branch(handler));
    }

    /// Adds a poll handler branch.
    pub fn add_poll_handler(&mut self, handler: BoxedHandler) {
        self.handlers.push(Update::filter_poll().branch(handler));
    }

    /// Adds a poll answer handler branch.
    pub fn add_poll_answer_handler(&mut self, handler: BoxedHandler) {
        self.handlers.push(Update::filter_poll_answer().branch(handler));
    }

    /// Adds a chat member handler branch.
    pub fn add_chat_member_handler(&mut self, handler: BoxedHandler) {
        self.handlers.push(Update::filter_chat_member().branch(handler));
    }

    /// Adds a my chat member handler branch.
    pub fn add_my_chat_member_handler(&mut self, handler: BoxedHandler) {
        self.handlers.push(Update::filter_my_chat_member().branch(handler));
    }

    /// Adds a chat join request handler branch.
    pub fn add_chat_join_request_handler(&mut self, handler: BoxedHandler) {
        self.handlers.push(Update::filter_chat_join_request().branch(handler));
    }

    /// Includes a sub-router.
    pub fn include_router(&mut self, router: Router) {
        self.sub_routers.push(router);
    }

    /// Merges another router into this one, combining all handlers.
    pub fn merge(mut self, other: Router) -> Self {
        self.include_router(other);
        self
    }

    /// Composes multiple routers into a single handler tree.
    ///
    /// Empty routers are skipped. If every router is empty, returns
    /// [`dptree::entry`].
    pub fn compose(routers: Vec<Router>) -> BoxedHandler {
        let mut combined: Option<BoxedHandler> = None;
        for router in routers {
            if router.handlers.is_empty() && router.sub_routers.is_empty() {
                continue;
            }
            let handler = router.into_handler();
            combined = Some(match combined {
                None => handler,
                Some(root) => root.branch(handler),
            });
        }
        combined.unwrap_or_else(dptree::entry)
    }

    /// Converts this router into a dptree handler tree.
    ///
    /// An empty router becomes [`dptree::entry`]. Sub-routers that are
    /// themselves empty are skipped so we never branch an entry onto another
    /// entry (which panics in dptree).
    pub fn into_handler(self) -> BoxedHandler {
        let mut parts: Vec<BoxedHandler> = self.handlers;

        for sub_router in self.sub_routers {
            if sub_router.handlers.is_empty() && sub_router.sub_routers.is_empty() {
                continue;
            }
            parts.push(sub_router.into_handler());
        }

        match parts.len() {
            0 => dptree::entry(),
            1 => parts.pop().unwrap(),
            _ => {
                let first = parts.remove(0);
                parts.into_iter().fold(first, |acc, h| acc.branch(h))
            }
        }
    }
}

impl fmt::Debug for Router {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Router")
            .field("name", &self.name)
            .field("handlers", &self.handlers.len())
            .field("sub_routers", &self.sub_routers.len())
            .finish()
    }
}

/// Extension trait for composing routers with the dispatcher.
pub trait RouterExt {
    /// Includes a router's handlers in the dispatcher.
    fn include_router(self, router: Router) -> Self;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn router_creation() {
        let router = Router::new("test");
        assert_eq!(router.name(), "test");
        assert!(router.handlers.is_empty());
    }

    #[test]
    fn router_merge() {
        let r1 = Router::new("r1");
        let r2 = Router::new("r2");
        let merged = r1.merge(r2);
        assert_eq!(merged.name(), "r1");
        assert_eq!(merged.sub_routers.len(), 1);
    }

    #[test]
    fn router_compose() {
        let mut r1 = Router::new("r1");
        r1.add_handler(Update::filter_message().endpoint(|| async { Ok(()) }));
        let mut r2 = Router::new("r2");
        r2.add_handler(Update::filter_message().endpoint(|| async { Ok(()) }));
        let _handler = Router::compose(vec![r1, r2]);
    }
}
