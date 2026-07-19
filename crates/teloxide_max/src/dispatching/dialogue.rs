//! Support for user dialogues.
//!
//! The main type is (surprise!) [`Dialogue`]. Under the hood, it is just a
//! wrapper over [`Storage`] and a chat ID. All it does is provides convenient
//! method for manipulating the dialogue state. [`Storage`] is where all
//! dialogue states are stored; it can be either [`InMemStorage`], which is a
//! simple hash map from [`std::collections`], or an advanced database wrapper
//! such as [`SqliteStorage`]. In the latter case, your dialogues are
//! _persistent_, meaning that you can safely restart your bot and all ongoing
//! dialogues will remain in the database -- this is a preferred method for
//! production bots.
//!
//! [`examples/dialogue.rs`] clearly demonstrates the typical usage of
//! dialogues. Your dialogue state can be represented as an enumeration:
//!
//! ```no_run
//! #[derive(Clone, Default)]
//! pub enum State {
//!     #[default]
//!     Start,
//!     ReceiveFullName,
//!     ReceiveAge {
//!         full_name: String,
//!     },
//!     ReceiveLocation {
//!         full_name: String,
//!         age: u8,
//!     },
//! }
//! ```
//!
//! Each state is associated with its respective handler: e.g., when a dialogue
//! state is `ReceiveAge`, `receive_age` is invoked:
//!
//! ```no_run
//! # use teloxide_max::{dispatching::dialogue::InMemStorage, prelude::*};
//! # type MyDialogue = Dialogue<State, InMemStorage<State>>;
//! # type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
//! # #[derive(Clone, Debug)] enum State { ReceiveLocation { full_name: String, age: u8 } }
//! async fn receive_age(
//!     bot: Bot,
//!     dialogue: MyDialogue,
//!     full_name: String, // Available from `State::ReceiveAge`.
//!     msg: Message,
//! ) -> HandlerResult {
//!     match msg.text().map(|text| text.parse::<u8>()) {
//!         Some(Ok(age)) => {
//!             bot.send_message(msg.chat.id, "What's your location?").await?;
//!             dialogue.update(State::ReceiveLocation { full_name, age }).await?;
//!         }
//!         _ => {
//!             bot.send_message(msg.chat.id, "Send me a number.").await?;
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! Variant's fields are passed to state handlers as single arguments like
//! `full_name: String` or tuples in case of two or more variant parameters (see
//! below). Using [`Dialogue::update`], you can update the dialogue with a new
//! state, in our case -- `State::ReceiveLocation { full_name, age }`. To exit
//! the dialogue, just call [`Dialogue::exit`] and it will be removed from the
//! underlying storage:
//!
//! ```no_run
//! # use teloxide_max::{dispatching::dialogue::InMemStorage, prelude::*};
//! # type MyDialogue = Dialogue<State, InMemStorage<State>>;
//! # type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
//! # #[derive(Clone, Debug)] enum State {}
//! async fn receive_location(
//!     bot: Bot,
//!     dialogue: MyDialogue,
//!     (full_name, age): (String, u8), // Available from `State::ReceiveLocation`.
//!     msg: Message,
//! ) -> HandlerResult {
//!     match msg.text() {
//!         Some(location) => {
//!             let message =
//!                 format!("Full name: {}\nAge: {}\nLocation: {}", full_name, age, location);
//!             bot.send_message(msg.chat.id, message).await?;
//!             dialogue.exit().await?;
//!         }
//!         None => {
//!             bot.send_message(msg.chat.id, "Send me a text message.").await?;
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! [`examples/dialogue.rs`]: https://github.com/sinescode/teloxide_max/blob/master/crates/teloxide_max/examples/dialogue.rs

#[cfg(feature = "redis-storage")]
pub use self::{RedisStorage, RedisStorageError};

#[cfg(any(feature = "sqlite-storage-nativetls", feature = "sqlite-storage-rustls"))]
pub use self::{SqliteStorage, SqliteStorageError};

#[cfg(any(feature = "postgres-storage-nativetls", feature = "postgres-storage-rustls"))]
pub use self::{PostgresStorage, PostgresStorageError};

pub use get_chat_id::GetChatId;
pub use storage::*;

pub mod scene;
pub mod strategy;
pub use strategy::{
    ChatStrategy, ChatTopicStrategy, DialogueKey, DialogueStrategy, GlobalUserStrategy,
    StrategyStorage, UserInChatStrategy, UserInTopicStrategy,
};

pub use scene::{route, Scene, SceneContext, SceneId, SceneManager, SceneRecord};

use dptree::Handler;
use teloxide_max_core::types::ChatId;

use std::{fmt::Debug, marker::PhantomData, sync::Arc};

use super::DpHandlerDescription;

mod get_chat_id;
mod storage;

const TELOXIDE_DIALOGUE_BEHAVIOUR: &str = "TELOXIDE_DIALOGUE_BEHAVIOUR";

/// A handle for controlling dialogue state.
#[derive(Debug)]
pub struct Dialogue<D, S>
where
    S: ?Sized,
{
    storage: Arc<S>,
    chat_id: ChatId,
    _phantom: PhantomData<D>,
}

// `#[derive]` requires generics to implement `Clone`, but `S` is wrapped around
// `Arc`, and `D` is wrapped around PhantomData.
impl<D, S> Clone for Dialogue<D, S>
where
    S: ?Sized,
{
    fn clone(&self) -> Self {
        Dialogue { storage: self.storage.clone(), chat_id: self.chat_id, _phantom: PhantomData }
    }
}

impl<D, S> Dialogue<D, S>
where
    D: Send + 'static,
    S: Storage<D> + ?Sized,
{
    /// Constructs a new dialogue with `storage` (where dialogues are stored)
    /// and `chat_id` of a current dialogue.
    #[must_use]
    pub fn new(storage: Arc<S>, chat_id: ChatId) -> Self {
        Self { storage, chat_id, _phantom: PhantomData }
    }

    /// Returns a chat ID associated with this dialogue.
    #[must_use]
    pub fn chat_id(&self) -> ChatId {
        self.chat_id
    }

    /// Retrieves the current state of the dialogue or `None` if there is no
    /// dialogue.
    pub async fn get(&self) -> Result<Option<D>, S::Error> {
        self.storage.clone().get_dialogue(self.chat_id).await
    }

    /// Like [`Dialogue::get`] but returns a default value if there is no
    /// dialogue.
    pub async fn get_or_default(&self) -> Result<D, S::Error>
    where
        D: Default,
    {
        match self.get().await? {
            Some(d) => Ok(d),
            None => {
                self.storage.clone().update_dialogue(self.chat_id, D::default()).await?;
                Ok(D::default())
            }
        }
    }

    /// Updates the dialogue state.
    ///
    /// The dialogue type `D` must implement `From<State>` to allow implicit
    /// conversion from `State` to `D`.
    pub async fn update<State>(&self, state: State) -> Result<(), S::Error>
    where
        D: From<State>,
    {
        let new_dialogue = state.into();
        self.storage.clone().update_dialogue(self.chat_id, new_dialogue).await?;
        Ok(())
    }

    /// Updates the dialogue with a default value.
    pub async fn reset(&self) -> Result<(), S::Error>
    where
        D: Default,
    {
        self.update(D::default()).await
    }

    /// Removes the dialogue from the storage provided to [`Dialogue::new`].
    pub async fn exit(&self) -> Result<(), S::Error> {
        self.storage.clone().remove_dialogue(self.chat_id).await
    }

    /// Returns a reference to the underlying storage.
    pub fn storage(&self) -> &Arc<S> {
        &self.storage
    }
}

/// Enters a dialogue context.
///
/// If `TELOXIDE_DIALOGUE_BEHAVIOUR` environmental variable exists and is equal
/// to "default", this function will not panic if it can't get the dialogue (if,
/// for example, the state enum was updated). Setting the value to "panic" will
/// return the initial behaviour.
///
/// A call to this function is the same as `dptree::entry().enter_dialogue()`.
///
/// See [`HandlerExt::enter_dialogue`].
///
/// ## Dependency requirements
///
///  - `Arc<S>`
///  - `Upd`
///
/// [`HandlerExt::enter_dialogue`]: super::HandlerExt::enter_dialogue
#[must_use]
pub fn enter<Upd, S, D, Output>() -> Handler<'static, Output, DpHandlerDescription>
where
    S: Storage<D> + ?Sized + Send + Sync + 'static,
    <S as Storage<D>>::Error: Debug + Send,
    D: Default + Clone + Send + Sync + 'static,
    Upd: GetChatId + Clone + Send + Sync + 'static,
    Output: Send + Sync + 'static,
{
    dptree::filter_map(|storage: Arc<S>, upd: Upd| {
        let chat_id = upd.chat_id()?;
        Some(Dialogue::new(storage, chat_id))
    })
    .filter_map_async(|dialogue: Dialogue<D, S>| async move {
        match dialogue.get_or_default().await {
            Ok(dialogue) => Some(dialogue),
            Err(err) => match std::env::var(TELOXIDE_DIALOGUE_BEHAVIOUR).as_deref() {
                Ok("default") => {
                    let default = D::default();
                    dialogue.update(default.clone()).await.ok()?;
                    Some(default)
                }
                Ok("panic") | Err(_) => {
                    log::error!("dialogue.get_or_default() failed: {err:?}");
                    None
                }
                Ok(_) => {
                    panic!(
                        "`TELOXIDE_DIALOGUE_BEHAVIOUR` env variable should be one of: \
                         default/panic"
                    )
                }
            },
        }
    })
}

/// A separate data storage for dialogues, independent of state.
///
/// This allows storing key-value data alongside the dialogue state,
/// similar to aiogram's `state.update_data()` / `state.get_data()`.
///
/// # Example
///
/// ```rust,no_run
/// # use teloxide_max::prelude::*;
/// # use teloxide_max::dispatching::dialogue::{DialogueData, InMemStorage, Dialogue};
/// # type MyDialogue = Dialogue<State, InMemStorage<State>>;
/// # type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
/// # #[derive(Clone, Default)]
/// # enum State { #[default] Start, WaitForAge }
///
/// async fn receive_name(
///     bot: Bot,
///     dialogue: MyDialogue,
///     mut data: DialogueData,
///     msg: Message,
/// ) -> HandlerResult {
///     let name = msg.text().unwrap_or("").to_string();
///     data.insert("name".to_string(), serde_json::Value::String(name.clone()));
///     dialogue.update(State::WaitForAge).await?;
///     bot.send_message(msg.chat.id, format!("Hello {name}! How old are you?")).await?;
///     Ok(())
/// }
///
/// async fn receive_age(
///     bot: Bot,
///     dialogue: MyDialogue,
///     mut data: DialogueData,
///     msg: Message,
/// ) -> HandlerResult {
///     let age = msg.text().unwrap_or("").to_string();
///     let name =
///         data.get("name").and_then(|v| v.as_str().map(|s| s.to_string())).unwrap_or_default();
///     data.clear();
///     dialogue.exit().await?;
///     bot.send_message(msg.chat.id, format!("Name: {name}, Age: {age}")).await?;
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone, Default)]
pub struct DialogueData {
    data: std::collections::HashMap<String, serde_json::Value>,
}

impl DialogueData {
    /// Creates a new empty dialogue data.
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts a key-value pair into the data.
    pub fn insert(&mut self, key: String, value: serde_json::Value) {
        self.data.insert(key, value);
    }

    /// Gets a value by key.
    pub fn get(&self, key: &str) -> Option<&serde_json::Value> {
        self.data.get(key)
    }

    /// Gets a mutable reference to a value by key.
    pub fn get_mut(&mut self, key: &str) -> Option<&mut serde_json::Value> {
        self.data.get_mut(key)
    }

    /// Removes a key-value pair from the data.
    pub fn remove(&mut self, key: &str) -> Option<serde_json::Value> {
        self.data.remove(key)
    }

    /// Returns true if the data contains the given key.
    pub fn contains_key(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    /// Returns the number of key-value pairs.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns true if the data is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Clears all key-value pairs.
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Returns an iterator over the key-value pairs.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &serde_json::Value)> {
        self.data.iter().map(|(k, v)| (k.as_str(), v))
    }

    /// Extends the data with another DialogueData.
    pub fn extend(&mut self, other: DialogueData) {
        self.data.extend(other.data);
    }

    /// Sets a string value.
    pub fn set_string(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.insert(key.into(), serde_json::Value::String(value.into()));
    }

    /// Gets a string value.
    pub fn get_string(&self, key: &str) -> Option<String> {
        self.get(key).and_then(|v| v.as_str().map(|s| s.to_string()))
    }

    /// Sets a numeric value.
    pub fn set_number(&mut self, key: impl Into<String>, value: f64) {
        if let Some(n) = serde_json::Number::from_f64(value) {
            self.insert(key.into(), serde_json::Value::Number(n));
        }
    }

    /// Gets a numeric value as f64.
    pub fn get_number(&self, key: &str) -> Option<f64> {
        self.get(key).and_then(|v| v.as_f64())
    }

    /// Sets a boolean value.
    pub fn set_bool(&mut self, key: impl Into<String>, value: bool) {
        self.insert(key.into(), serde_json::Value::Bool(value));
    }

    /// Gets a boolean value.
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.get(key).and_then(|v| v.as_bool())
    }

    /// Serializes the data to JSON.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self.data)
    }

    /// Deserializes data from JSON.
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        let data = serde_json::from_str(json)?;
        Ok(Self { data })
    }
}

impl std::ops::Index<&str> for DialogueData {
    type Output = serde_json::Value;

    fn index(&self, index: &str) -> &Self::Output {
        self.data.get(index).unwrap_or(&serde_json::Value::Null)
    }
}

impl std::ops::IndexMut<&str> for DialogueData {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        self.data.entry(index.to_string()).or_insert(serde_json::Value::Null)
    }
}
