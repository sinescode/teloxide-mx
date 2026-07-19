use derive_more::From;
use serde::{Deserialize, Serialize};

/// Unique identifier for a guest query.
#[derive(Clone, Debug, derive_more::Display)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize, From)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(transparent)]
#[from(&'static str, String)]
pub struct GuestQueryId(pub String);
