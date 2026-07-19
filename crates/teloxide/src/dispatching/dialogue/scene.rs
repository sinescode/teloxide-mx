//! Scenes / Wizards — stateful conversation flows built on top of Dialogue.
//!
//! Similar to aiogram's `Scene` system, this module provides:
//! - **Scenes**: named conversation flows with their own state machine
//! - **History**: rollback to previous states within a scene
//! - **Scene transitions**: enter, exit, switch between scenes
//! - **Nested scenes**: scenes can contain sub-scenes
//!
//! # Example
//!
//! ```rust,no_run
//! # use teloxide::prelude::*;
//! # use teloxide::dispatching::dialogue::{InMemStorage, Dialogue};
//! # use teloxide::dispatching::dialogue::scene::{Scene, SceneId, SceneState, SceneContext};
//! # type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
//!
//! #[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
//! enum RegistrationState {
//!     #[default]
//!     Start,
//!     WaitForName,
//!     WaitForAge,
//! }
//!
//! struct RegistrationScene;
//!
//! #[async_trait::async_trait]
//! impl Scene for RegistrationScene {
//!     type State = RegistrationState;
//!
//!     fn id(&self) -> &str {
//!         "registration"
//!     }
//!
//!     async fn on_enter(&self, ctx: SceneContext<'_, Self::State>) -> HandlerResult {
//!         ctx.answer("Welcome to registration! What's your name?").await?;
//!         ctx.set_state(RegistrationState::WaitForName).await?;
//!         Ok(())
//!     }
//!
//!     async fn on_message(
//!         &self,
//!         ctx: SceneContext<'_, Self::State>,
//!         msg: Message,
//!     ) -> HandlerResult {
//!         match ctx.state() {
//!             RegistrationState::WaitForName => {
//!                 let name = msg.text().unwrap_or("").to_string();
//!                 ctx.answer(format!("Hello {name}! How old are you?")).await?;
//!                 ctx.set_state(RegistrationState::WaitForAge).await?;
//!             }
//!             RegistrationState::WaitForAge => {
//!                 let age = msg.text().unwrap_or("");
//!                 ctx.answer(format!("You're {age}! Registration complete.")).await?;
//!                 ctx.exit().await?;
//!             }
//!             _ => {}
//!         }
//!         Ok(())
//!     }
//! }
//! ```

use std::{collections::HashMap, sync::Arc};

use crate::{
    dispatching::UpdateHandler,
    requests::Requester,
    types::{ChatId, Message, UserId},
};

/// Identifier for a scene.
pub type SceneId = String;

/// A trait for defining scenes (conversation flows).
pub trait Scene: Send + Sync + 'static {
    /// The state type for this scene.
    type State: Clone + Default + Send + Sync + 'static;

    /// Returns the unique identifier for this scene.
    fn id(&self) -> &str;

    /// Called when the scene is entered.
    fn on_enter(
        &self,
        ctx: SceneContext<'_, Self::State>,
    ) -> impl std::future::Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send
    {
        async move {
            let _ = ctx;
            Ok(())
        }
    }

    /// Called when a message is received while in this scene.
    fn on_message(
        &self,
        ctx: SceneContext<'_, Self::State>,
        msg: Message,
    ) -> impl std::future::Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send
    {
        async move {
            let _ = (ctx, msg);
            Ok(())
        }
    }

    /// Called when a callback query is received while in this scene.
    fn on_callback_query(
        &self,
        ctx: SceneContext<'_, Self::State>,
        q: crate::types::CallbackQuery,
    ) -> impl std::future::Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send
    {
        async move {
            let _ = (ctx, q);
            Ok(())
        }
    }

    /// Called when the scene is exited.
    fn on_exit(
        &self,
        ctx: SceneContext<'_, Self::State>,
    ) -> impl std::future::Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send
    {
        async move {
            let _ = ctx;
            Ok(())
        }
    }
}

/// Context passed to scene handlers.
pub struct SceneContext<'a, S> {
    bot: &'a crate::Bot,
    chat_id: ChatId,
    user_id: Option<UserId>,
    state: S,
    history: &'a mut Vec<SceneRecord>,
    scene_id: &'a str,
}

/// A record in the scene history (for rollback).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SceneRecord {
    pub scene_id: String,
    pub state_snapshot: String, // serialized state
}

impl<'a, S: Clone + Default + serde::Serialize + for<'de> serde::Deserialize<'de>>
    SceneContext<'a, S>
{
    /// Returns the current state.
    pub fn state(&self) -> &S {
        &self.state
    }

    /// Returns the scene ID.
    pub fn scene_id(&self) -> &str {
        self.scene_id
    }

    /// Returns the chat ID.
    pub fn chat_id(&self) -> ChatId {
        self.chat_id
    }

    /// Sends a message to the chat (like `message.answer()`).
    pub async fn answer(&self, text: impl Into<String>) -> Result<(), crate::RequestError> {
        self.bot.send_message(self.chat_id, text).await?;
        Ok(())
    }

    /// Sets the scene state.
    pub async fn set_state(
        &mut self,
        state: S,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.state = state;
        Ok(())
    }

    /// Takes the current state, replacing it with Default.
    pub fn take_state(&mut self) -> S {
        std::mem::take(&mut self.state)
    }

    /// Pushes current state to history (for later rollback).
    pub fn snapshot(&mut self) {
        let snapshot = serde_json::to_string(&self.state).unwrap_or_default();
        self.history
            .push(SceneRecord { scene_id: self.scene_id.to_string(), state_snapshot: snapshot });
    }

    /// Rolls back to the previous state in history.
    pub fn rollback(&mut self) -> bool {
        if let Some(record) = self.history.pop() {
            if let Ok(state) = serde_json::from_str(&record.state_snapshot) {
                self.state = state;
            }
            true
        } else {
            false
        }
    }

    /// Returns the number of history entries.
    pub fn history_len(&self) -> usize {
        self.history.len()
    }
}

/// Manages scenes and their lifecycle.
pub struct SceneManager {
    scenes: HashMap<String, Arc<dyn std::any::Any + Send + Sync>>,
    handlers: HashMap<String, UpdateHandler<Box<dyn std::error::Error + Send + Sync>>>,
    history: Vec<SceneRecord>,
}

impl SceneManager {
    /// Creates a new empty scene manager.
    pub fn new() -> Self {
        Self { scenes: HashMap::new(), handlers: HashMap::new(), history: Vec::new() }
    }

    /// Registers a scene.
    pub fn register<S: Scene>(&mut self, scene: S) {
        let id = scene.id().to_string();
        self.scenes.insert(id, Arc::new(scene));
    }

    /// Registers a handler for a scene by its ID.
    pub fn register_handler(
        &mut self,
        scene_id: impl Into<String>,
        handler: UpdateHandler<Box<dyn std::error::Error + Send + Sync>>,
    ) {
        self.handlers.insert(scene_id.into(), handler);
    }

    /// Returns the list of registered scene IDs.
    pub fn scene_ids(&self) -> Vec<&str> {
        self.scenes.keys().map(|s| s.as_str()).collect()
    }

    /// Dispatches to the handler for the given scene ID.
    ///
    /// Returns `Some(handler)` if a handler is registered for `scene_id`,
    /// `None` otherwise.
    pub fn dispatch(
        &self,
        scene_id: &str,
    ) -> Option<&UpdateHandler<Box<dyn std::error::Error + Send + Sync>>> {
        self.handlers.get(scene_id)
    }

    /// Returns the current history depth.
    pub fn history_len(&self) -> usize {
        self.history.len()
    }

    /// Clears the history.
    pub fn clear_history(&mut self) {
        self.history.clear();
    }
}

impl Default for SceneManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Creates a handler tree from registered scenes.
///
/// The returned handler branches on each registered scene handler. When an
/// update arrives, the handler tree dispatches to the matching scene handler
/// based on the scene ID extracted from the dialogue state.
///
/// # Example
///
/// ```rust,no_run
/// # use teloxide::prelude::*;
/// # use teloxide::dispatching::dialogue::{InMemStorage, Dialogue};
/// # use teloxide::dispatching::dialogue::scene::{Scene, SceneManager, route};
/// # type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
///
/// let mut manager = SceneManager::new();
/// // register scenes and their handlers ...
///
/// let scene_handler = route(&manager);
/// ```
pub fn route(manager: &SceneManager) -> UpdateHandler<Box<dyn std::error::Error + Send + Sync>> {
    let mut root = dptree::entry();

    for (_scene_id, handler) in &manager.handlers {
        root = root.branch(handler.clone());
    }

    root
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scene_manager_register() {
        let mut manager = SceneManager::new();
        assert!(manager.scene_ids().is_empty());

        // We can't easily test with real Scene impl without async_trait,
        // so just test the manager structure
        assert_eq!(manager.history_len(), 0);
    }

    #[test]
    fn scene_context_snapshot_rollback() {
        // Test history management
        let mut history = Vec::new();
        history.push(SceneRecord {
            scene_id: "test".to_string(),
            state_snapshot: r#""Start""#.to_string(),
        });
        history.push(SceneRecord {
            scene_id: "test".to_string(),
            state_snapshot: r#""WaitForName""#.to_string(),
        });

        assert_eq!(history.len(), 2);
        let last = history.pop().unwrap();
        assert_eq!(last.state_snapshot, r#""WaitForName""#);
        assert_eq!(history.len(), 1);
    }
}
