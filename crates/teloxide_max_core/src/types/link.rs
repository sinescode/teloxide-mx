use serde::{Deserialize, Serialize};

/// Represents a link object used in various API surfaces.
///
/// [The official docs](https://core.telegram.org/bots/api#link).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct Link {
    /// URL of the link
    pub url: String,
    /// Optional label
    pub label: Option<String>,
}
