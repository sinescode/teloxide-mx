//! FSM (Finite State Machine) strategies — control how dialogue state is keyed.
//!
//! Similar to aiogram's `FSMStrategy`, these strategies determine the storage
//! key for dialogue state, enabling different scoping behaviors:
//!
//! - [`ChatStrategy`] — state per chat (default teloxide_max behavior)
//! - [`UserInChatStrategy`] — state per user per chat (aiogram default)
//! - [`GlobalUserStrategy`] — state per user across all chats
//! - [`UserInTopicStrategy`] — state per user per chat per topic
//! - [`ChatTopicStrategy`] — state per chat per topic
//!
//! # Example
//!
//! ```rust,no_run
//! # use teloxide_max::prelude::*;
//! # use teloxide_max::dispatching::dialogue::{InMemStorage, Dialogue};
//! # use teloxide_max::dispatching::dialogue::strategy::{DialogueKey, UserInChatStrategy, StrategyStorage};
//! # type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
//! #[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
//! enum State {
//!     #[default]
//!     Start,
//!     ReceiveName,
//! }
//!
//! # async fn example() {
//! let bot = Bot::from_env();
//!
//! // Use UserInChat strategy (each user has own state per chat)
//! let storage = StrategyStorage::<State, UserInChatStrategy>::new();
//!
//! let handler = Update::filter_message()
//!     .branch(Message::filter_text().endpoint(handle_text));
//!
//! Dispatcher::builder(bot, handler)
//!     .dependencies(dptree::deps![storage])
//!     .build()
//!     .dispatch()
//!     .await;
//! # }
//! # async fn handle_text(bot: Bot, msg: Message, dialogue: Dialogue<State, StrategyStorage<State, UserInChatStrategy>>) -> HandlerResult { Ok(()) }
//! ```

use std::{collections::HashMap, hash::Hash, marker::PhantomData, sync::Arc};

use futures::future::BoxFuture;
use teloxide_max_core::types::{ChatId, MessageId, ThreadId, UserId};
use tokio::sync::Mutex;

use super::Storage;

/// A composite key for dialogue storage.
///
/// Different strategies extract different combinations of these fields
/// to form the actual storage key.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct DialogueKey {
    /// The chat ID.
    pub chat_id: ChatId,
    /// The user ID (optional, for per-user strategies).
    pub user_id: Option<UserId>,
    /// The thread/topic ID (optional, for topic-aware strategies).
    pub thread_id: Option<ThreadId>,
}

impl DialogueKey {
    /// Creates a key from just a chat ID.
    pub fn from_chat(chat_id: ChatId) -> Self {
        Self { chat_id, user_id: None, thread_id: None }
    }

    /// Creates a key from chat + user IDs.
    pub fn from_user_in_chat(chat_id: ChatId, user_id: UserId) -> Self {
        Self { chat_id, user_id: Some(user_id), thread_id: None }
    }

    /// Creates a key from a global user ID.
    pub fn from_global_user(user_id: UserId) -> Self {
        Self {
            chat_id: ChatId(0), // placeholder
            user_id: Some(user_id),
            thread_id: None,
        }
    }

    /// Creates a key from chat + user + thread IDs.
    pub fn from_user_in_topic(chat_id: ChatId, user_id: UserId, thread_id: ThreadId) -> Self {
        Self { chat_id, user_id: Some(user_id), thread_id: Some(thread_id) }
    }

    /// Creates a key from chat + thread IDs.
    pub fn from_chat_topic(chat_id: ChatId, thread_id: ThreadId) -> Self {
        Self { chat_id, user_id: None, thread_id: Some(thread_id) }
    }
}

/// A trait for extracting dialogue keys from updates.
///
/// Different strategies implement this to determine how state is scoped.
pub trait DialogueStrategy: Send + Sync + 'static {
    /// Extracts a dialogue key from an update context.
    fn extract_key(
        chat_id: ChatId,
        user_id: Option<UserId>,
        thread_id: Option<ThreadId>,
    ) -> DialogueKey;
}

/// State is stored per chat (default teloxide_max behavior).
///
/// All users in a chat share the same dialogue state.
pub struct ChatStrategy;

impl DialogueStrategy for ChatStrategy {
    fn extract_key(
        chat_id: ChatId,
        _user_id: Option<UserId>,
        _thread_id: Option<ThreadId>,
    ) -> DialogueKey {
        DialogueKey::from_chat(chat_id)
    }
}

/// State is stored per user per chat (aiogram's default `USER_IN_CHAT`).
///
/// Each user in each chat has their own independent dialogue state.
pub struct UserInChatStrategy;

impl DialogueStrategy for UserInChatStrategy {
    fn extract_key(
        chat_id: ChatId,
        user_id: Option<UserId>,
        _thread_id: Option<ThreadId>,
    ) -> DialogueKey {
        let uid = user_id.unwrap_or(UserId(0));
        DialogueKey::from_user_in_chat(chat_id, uid)
    }
}

/// State is stored per user globally across all chats (`GLOBAL_USER`).
///
/// A user has the same dialogue state regardless of which chat they're in.
pub struct GlobalUserStrategy;

impl DialogueStrategy for GlobalUserStrategy {
    fn extract_key(
        _chat_id: ChatId,
        user_id: Option<UserId>,
        _thread_id: Option<ThreadId>,
    ) -> DialogueKey {
        let uid = user_id.unwrap_or(UserId(0));
        DialogueKey::from_global_user(uid)
    }
}

/// State is stored per user per chat per topic (`USER_IN_TOPIC`).
///
/// Each user in each forum topic has their own independent state.
pub struct UserInTopicStrategy;

impl DialogueStrategy for UserInTopicStrategy {
    fn extract_key(
        chat_id: ChatId,
        user_id: Option<UserId>,
        thread_id: Option<ThreadId>,
    ) -> DialogueKey {
        let uid = user_id.unwrap_or(UserId(0));
        let tid = thread_id.unwrap_or(ThreadId(MessageId(0)));
        DialogueKey::from_user_in_topic(chat_id, uid, tid)
    }
}

/// State is stored per chat per topic (`CHAT_TOPIC`).
///
/// All users in a forum topic share the same state.
pub struct ChatTopicStrategy;

impl DialogueStrategy for ChatTopicStrategy {
    fn extract_key(
        chat_id: ChatId,
        _user_id: Option<UserId>,
        thread_id: Option<ThreadId>,
    ) -> DialogueKey {
        let tid = thread_id.unwrap_or(ThreadId(MessageId(0)));
        DialogueKey::from_chat_topic(chat_id, tid)
    }
}

/// An in-memory dialogue storage with configurable keying strategy.
///
/// This is like [`InMemStorage`](super::InMemStorage) but uses [`DialogueKey`]
/// instead of plain `ChatId`, allowing different scoping strategies.
pub struct StrategyStorage<D, S: DialogueStrategy = ChatStrategy> {
    map: Mutex<HashMap<DialogueKey, D>>,
    _strategy: PhantomData<S>,
}

impl<D, S: DialogueStrategy> StrategyStorage<D, S> {
    /// Creates a new empty storage.
    pub fn new() -> Arc<Self> {
        Arc::new(Self { map: Mutex::new(HashMap::new()), _strategy: PhantomData })
    }
}

impl<D, S: DialogueStrategy> Default for StrategyStorage<D, S> {
    fn default() -> Self {
        Self { map: Mutex::new(HashMap::new()), _strategy: PhantomData }
    }
}

impl<D, S> Storage<D> for StrategyStorage<D, S>
where
    D: Clone + Send + 'static,
    S: DialogueStrategy,
{
    type Error = StrategyStorageError;

    fn remove_dialogue(
        self: Arc<Self>,
        chat_id: ChatId,
    ) -> BoxFuture<'static, Result<(), Self::Error>>
    where
        D: Send + 'static,
    {
        Box::pin(async move {
            let key = S::extract_key(chat_id, None, None);
            self.map.lock().await.remove(&key).ok_or(StrategyStorageError::DialogueNotFound)?;
            Ok(())
        })
    }

    fn update_dialogue(
        self: Arc<Self>,
        chat_id: ChatId,
        dialogue: D,
    ) -> BoxFuture<'static, Result<(), Self::Error>>
    where
        D: Send + 'static,
    {
        Box::pin(async move {
            let key = S::extract_key(chat_id, None, None);
            self.map.lock().await.insert(key, dialogue);
            Ok(())
        })
    }

    fn get_dialogue(
        self: Arc<Self>,
        chat_id: ChatId,
    ) -> BoxFuture<'static, Result<Option<D>, Self::Error>> {
        Box::pin(async move {
            let key = S::extract_key(chat_id, None, None);
            Ok(self.map.lock().await.get(&key).cloned())
        })
    }

    fn get_dialogue_with_context(
        self: Arc<Self>,
        chat_id: ChatId,
        user_id: Option<UserId>,
        thread_id: Option<ThreadId>,
    ) -> BoxFuture<'static, Result<Option<D>, Self::Error>> {
        Box::pin(async move {
            let key = S::extract_key(chat_id, user_id, thread_id);
            Ok(self.map.lock().await.get(&key).cloned())
        })
    }
}

/// Error type for [`StrategyStorage`].
#[derive(Debug, thiserror::Error)]
pub enum StrategyStorageError {
    #[error("dialogue not found")]
    DialogueNotFound,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dialogue_key_chat() {
        let key = DialogueKey::from_chat(ChatId(123));
        assert_eq!(key.chat_id, ChatId(123));
        assert!(key.user_id.is_none());
    }

    #[test]
    fn dialogue_key_user_in_chat() {
        let key = DialogueKey::from_user_in_chat(ChatId(123), UserId(456));
        assert_eq!(key.chat_id, ChatId(123));
        assert_eq!(key.user_id, Some(UserId(456)));
    }

    #[test]
    fn strategy_chat() {
        let key = ChatStrategy::extract_key(ChatId(1), Some(UserId(2)), None);
        assert_eq!(key, DialogueKey::from_chat(ChatId(1)));
    }

    #[test]
    fn strategy_user_in_chat() {
        let key = UserInChatStrategy::extract_key(ChatId(1), Some(UserId(2)), None);
        assert_eq!(key, DialogueKey::from_user_in_chat(ChatId(1), UserId(2)));
    }

    #[test]
    fn strategy_global_user() {
        let key = GlobalUserStrategy::extract_key(ChatId(1), Some(UserId(2)), None);
        assert_eq!(key.user_id, Some(UserId(2)));
    }

    #[tokio::test]
    async fn strategy_storage_roundtrip() {
        let storage = StrategyStorage::<String, UserInChatStrategy>::new();
        let key = DialogueKey::from_user_in_chat(ChatId(1), UserId(2));

        // Manually insert
        storage.map.lock().await.insert(key.clone(), "hello".to_string());

        // Read back
        let result = storage.map.lock().await.get(&key).cloned();
        assert_eq!(result.as_deref(), Some("hello"));
    }
}
