//! MagicData-style contextual filter (aiogram `MagicData` parity).
//!
//! Filters events using a predicate over dependency-injected context, not only
//! the event object. In aiogram this inspects `kwargs` from middleware; in
//! teloxide_max the equivalent is a dptree filter that receives injected deps.
//!
//! # Example
//!
//! ```rust,no_run
//! use teloxide_max::{
//!     prelude::*,
//!     utils::magic_data::{and_f, invert_f, magic_data, or_f},
//! };
//!
//! #[derive(Clone)]
//! struct IsAdmin(bool);
//!
//! let handler = Update::filter_message()
//!     // Filter on DI context (admin flag), like MagicData(F["is_admin"] == True)
//!     .branch(dptree::filter(|admin: IsAdmin| magic_data(admin.0)).endpoint(
//!         |bot: Bot, msg: Message| async move {
//!             bot.send_message(msg.chat.id, "Admin only").await?;
//!             Ok(())
//!         },
//!     ))
//!     // Compose predicates
//!     .branch(
//!         dptree::filter(|msg: Message| {
//!             and_f(msg.text().is_some(), msg.from.as_ref().is_some_and(|u| !u.is_bot))
//!         })
//!         .endpoint(|bot: Bot, msg: Message| async move {
//!             bot.send_message(msg.chat.id, "ok").await?;
//!             Ok(())
//!         }),
//!     );
//! # let _ = handler;
//! ```
//!
//! # Migration from aiogram
//!
//! ```python
//! # aiogram
//! from aiogram.filters import MagicData
//! from magic_filter import F
//! @router.message(MagicData(F["is_admin"].as_(True)))
//! async def admin_handler(message: Message): ...
//! ```
//!
//! ```rust
//! // teloxide_max — inject IsAdmin via middleware/deps, then:
//! dptree::filter(|admin: IsAdmin| admin.0)
//! ```

/// Pass-through for a boolean context check (documents MagicData intent).
///
/// Prefer writing the predicate directly; this helper exists for readability
/// and migration docs.
#[inline]
pub fn magic_data(condition: bool) -> bool {
    condition
}

/// Logical AND of two filter results (aiogram `and_f` parity).
#[inline]
pub fn and_f(a: bool, b: bool) -> bool {
    a && b
}

/// Logical OR of two filter results (aiogram `or_f` parity).
#[inline]
pub fn or_f(a: bool, b: bool) -> bool {
    a || b
}

/// Logical NOT of a filter result (aiogram `invert_f` parity).
#[inline]
pub fn invert_f(a: bool) -> bool {
    !a
}

/// Combines many boolean predicates with AND (all must pass).
#[inline]
pub fn and_all(parts: impl IntoIterator<Item = bool>) -> bool {
    parts.into_iter().all(|x| x)
}

/// Combines many boolean predicates with OR (any may pass).
#[inline]
pub fn or_any(parts: impl IntoIterator<Item = bool>) -> bool {
    parts.into_iter().any(|x| x)
}

/// Builds a dptree-compatible message filter from a predicate.
///
/// ```rust
/// use teloxide_max::{types::Message, utils::magic_data::filter_msg};
///
/// let f = filter_msg(|msg: &Message| msg.text().is_some());
/// ```
pub fn filter_msg<F>(pred: F) -> impl Fn(Message) -> bool + Send + Sync + 'static
where
    F: Fn(&Message) -> bool + Send + Sync + 'static,
{
    move |msg: Message| pred(&msg)
}

use teloxide_max_core::types::Message;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logic_helpers() {
        assert!(magic_data(true));
        assert!(!magic_data(false));
        assert!(and_f(true, true));
        assert!(!and_f(true, false));
        assert!(or_f(false, true));
        assert!(!or_f(false, false));
        assert!(invert_f(false));
        assert!(!invert_f(true));
        assert!(and_all([true, true, true]));
        assert!(!and_all([true, false]));
        assert!(or_any([false, true]));
        assert!(!or_any([false, false]));
    }
}
