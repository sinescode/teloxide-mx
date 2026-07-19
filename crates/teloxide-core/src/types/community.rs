use serde::{Deserialize, Serialize};

/// Represents a community (a group of chats).
///
/// [The official docs](https://core.telegram.org/bots/api#community).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct Community {
    /// Unique identifier of the community
    pub id: i64,
    /// Title of the community
    pub title: Option<String>,
    /// Username of the community
    pub username: Option<String>,
}
