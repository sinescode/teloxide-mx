//! Modular router system for organizing handlers.
//!
//! Similar to aiogram's `Router`, this module allows splitting handler logic
//! into separate modules that can be composed together.
//!
//! # Example
//!
//! ```rust,no_run
//! # use teloxide::prelude::*;
//! # use teloxide::dispatching::router::Router;
//!
//! # #[tokio::main]
//! # async fn main() {
//! let mut admin_router = Router::new("admin");
//! admin_router.add_handler(
//!     Update::filter_message()
//!         .filter_command::<AdminCommand>()
//!         .endpoint(handle_admin_command)
//! );
//!
//! let mut user_router = Router::new("user");
//! user_router.add_handler(
//!     Update::filter_message()
//!         .filter_command::<UserCommand>()
//!         .endpoint(handle_user_command)
//! );
//!
//! // Compose routers into a handler tree
//! let schema = Router::compose(vec![admin_router, user_router]);
//! # }
//! ```

use crate::dispatching::{UpdateHandler, UpdateFilterExt};
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
        Self {
            name: name.into(),
            handlers: Vec::new(),
            sub_routers: Vec::new(),
        }
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
        self.handlers
            .push(UpdateFilterExt::filter_message().branch(handler));
    }

    /// Adds a callback query handler branch.
    pub fn add_callback_handler(&mut self, handler: BoxedHandler) {
        self.handlers
            .push(UpdateFilterExt::filter_callback_query().branch(handler));
    }

    /// Adds an inline query handler branch.
    pub fn add_inline_handler(&mut self, handler: BoxedHandler) {
        self.handlers
            .push(UpdateFilterExt::filter_inline_query().branch(handler));
    }

    /// Adds a chosen inline result handler branch.
    pub fn add_chosen_inline_handler(&mut self, handler: BoxedHandler) {
        self.handlers
            .push(UpdateFilterExt::filter_chosen_inline_result().branch(handler));
    }

    /// Adds a shipping query handler branch.
    pub fn add_shipping_handler(&mut self, handler: BoxedHandler) {
        self.handlers
            .push(UpdateFilterExt::filter_shipping_query().branch(handler));
    }

    /// Adds a pre-checkout query handler branch.
    pub fn add_pre_checkout_handler(&mut self, handler: BoxedHandler) {
        self.handlers
            .push(UpdateFilterExt::filter_pre_checkout_query().branch(handler));
    }

    /// Adds a channel post handler branch.
    pub fn add_channel_post_handler(&mut self, handler: BoxedHandler) {
        self.handlers
            .push(UpdateFilterExt::filter_channel_post().branch(handler));
    }

    /// Adds an edited message handler branch.
    pub fn add_edited_message_handler(&mut self, handler: BoxedHandler) {
        self.handlers
            .push(UpdateFilterExt::filter_edited_message().branch(handler));
    }

    /// Adds a message reaction handler branch.
    pub fn add_message_reaction_handler(&mut self, handler: BoxedHandler) {
        self.handlers
            .push(UpdateFilterExt::filter_message_reaction_updated().branch(handler));
    }

    /// Adds a poll handler branch.
    pub fn add_poll_handler(&mut self, handler: BoxedHandler) {
        self.handlers
            .push(UpdateFilterExt::filter_poll().branch(handler));
    }

    /// Adds a poll answer handler branch.
    pub fn add_poll_answer_handler(&mut self, handler: BoxedHandler) {
        self.handlers
            .push(UpdateFilterExt::filter_poll_answer().branch(handler));
    }

    /// Adds a chat member handler branch.
    pub fn add_chat_member_handler(&mut self, handler: BoxedHandler) {
        self.handlers
            .push(UpdateFilterExt::filter_chat_member().branch(handler));
    }

    /// Adds a my chat member handler branch.
    pub fn add_my_chat_member_handler(&mut self, handler: BoxedHandler) {
        self.handlers
            .push(UpdateFilterExt::filter_my_chat_member().branch(handler));
    }

    /// Adds a chat join request handler branch.
    pub fn add_chat_join_request_handler(&mut self, handler: BoxedHandler) {
        self.handlers
            .push(UpdateFilterExt::filter_chat_join_request().branch(handler));
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
    pub fn compose(routers: Vec<Router>) -> BoxedHandler {
        let mut root = dptree::entry();
        for router in routers {
            let handler = router.into_handler();
            root = root.branch(handler);
        }
        root
    }

    /// Converts this router into a dptree handler tree.
    pub fn into_handler(self) -> BoxedHandler {
        let mut root = dptree::entry();

        // Add all direct handlers
        for handler in self.handlers {
            root = root.branch(handler);
        }

        // Add sub-routers
        for sub_router in self.sub_routers {
            let sub_handler = sub_router.into_handler();
            root = root.branch(sub_handler);
        }

        root
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
        let r1 = Router::new("r1");
        let r2 = Router::new("r2");
        let _handler = Router::compose(vec![r1, r2]);
    }
}
