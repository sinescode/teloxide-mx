//! Thread-local flags for cross-cutting concerns.
//!
//! This module provides a [`Flags`] struct that implements a thread-local
//! key-value store for passing cross-cutting concerns through handler
//! chains without polluting function signatures.
//!
//! # Example
//!
//! ```rust
//! use teloxide_max::utils::flags::{FlagKey, Flags};
//!
//! // Create a typed flag key using static ID
//! static REQUEST_ID: FlagKey<String> = FlagKey::new(0);
//!
//! // Set a flag
//! Flags::set(REQUEST_ID, "abc-123".to_string());
//!
//! // Get a flag
//! assert_eq!(Flags::get(REQUEST_ID), Some("abc-123".to_string()));
//!
//! // Remove a flag
//! Flags::remove(REQUEST_ID);
//! assert_eq!(Flags::get(REQUEST_ID), None);
//! ```

use std::{
    any::{Any, TypeId},
    cell::RefCell,
    collections::HashMap,
};

/// A typed key for use with [`Flags`].
///
/// Each `FlagKey` carries type information so that `Flags::get` and
/// `Flags::set` are type-safe. Create keys using `FlagKey::new(id)` where
/// `id` is a unique `usize` identifier.
///
/// # Example
///
/// ```rust
/// use teloxide_max::utils::flags::FlagKey;
///
/// static MY_KEY: FlagKey<i32> = FlagKey::new(0);
/// ```
pub struct FlagKey<T: Send + 'static> {
    id: usize,
    _marker: std::marker::PhantomData<T>,
}

impl<T: Send + 'static> Clone for FlagKey<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Send + 'static> Copy for FlagKey<T> {}

impl<T: Send + 'static> FlagKey<T> {
    /// Creates a new `FlagKey` with the given numeric identifier.
    ///
    /// Each key must have a unique `id` within the scope where flags are
    /// used. Using the same `id` for different keys will cause conflicts.
    ///
    /// # Example
    ///
    /// ```rust
    /// use teloxide_max::utils::flags::FlagKey;
    ///
    /// static MY_KEY: FlagKey<i32> = FlagKey::new(42);
    /// ```
    pub const fn new(id: usize) -> Self {
        Self { id, _marker: std::marker::PhantomData }
    }
}

/// Type-erased value stored in flags.
struct ErasedValue {
    _type_id: TypeId,
    value: Box<dyn Any + Send>,
}

/// A thread-local key-value store for cross-cutting concerns.
///
/// `Flags` allows you to store and retrieve typed values in a thread-local
/// manner. This is useful for passing metadata through handler chains
/// without modifying function signatures.
///
/// # Thread Safety
///
/// All operations are thread-local. Values stored in one thread are not
/// visible from other threads.
///
/// # Example
///
/// ```rust
/// use teloxide_max::utils::flags::{FlagKey, Flags};
///
/// static REQUEST_ID: FlagKey<String> = FlagKey::new(0);
/// static USER_ID: FlagKey<u64> = FlagKey::new(1);
///
/// Flags::set(REQUEST_ID, "abc".to_string());
/// Flags::set(USER_ID, 12345u64);
///
/// assert_eq!(Flags::get(REQUEST_ID), Some("abc".to_string()));
/// assert_eq!(Flags::get(USER_ID), Some(12345u64));
/// ```
pub struct Flags;

thread_local! {
    static FLAGS: RefCell<HashMap<(usize, TypeId), ErasedValue>> = RefCell::new(HashMap::new());
}

impl Flags {
    /// Sets a flag value for the given key.
    ///
    /// If a value was previously set for this key, it is replaced.
    ///
    /// # Example
    ///
    /// ```rust
    /// use teloxide_max::utils::flags::{FlagKey, Flags};
    ///
    /// static MY_KEY: FlagKey<i32> = FlagKey::new(0);
    /// Flags::set(MY_KEY, 42);
    /// ```
    pub fn set<T: Send + 'static>(key: FlagKey<T>, value: T) {
        FLAGS.with(|flags| {
            let mut flags = flags.borrow_mut();
            let type_id = TypeId::of::<T>();
            flags.insert(
                (key.id, type_id),
                ErasedValue { _type_id: type_id, value: Box::new(value) },
            );
        });
    }

    /// Gets a flag value for the given key.
    ///
    /// Returns `Some(value)` if a value was set, `None` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use teloxide_max::utils::flags::{FlagKey, Flags};
    ///
    /// static MY_KEY: FlagKey<i32> = FlagKey::new(0);
    /// Flags::set(MY_KEY, 42);
    /// assert_eq!(Flags::get(MY_KEY), Some(42));
    /// ```
    pub fn get<T: Clone + Send + 'static>(key: FlagKey<T>) -> Option<T> {
        FLAGS.with(|flags| {
            let mut flags = flags.borrow_mut();
            let type_id = TypeId::of::<T>();
            flags
                .get_mut(&(key.id, type_id))
                .and_then(|erased| erased.value.downcast_mut::<T>())
                .cloned()
        })
    }

    /// Gets a flag value for the given key, or the default if not set.
    ///
    /// # Example
    ///
    /// ```rust
    /// use teloxide_max::utils::flags::{FlagKey, Flags};
    ///
    /// static MY_KEY: FlagKey<i32> = FlagKey::new(0);
    /// assert_eq!(Flags::get_or_default(MY_KEY), 0);
    ///
    /// Flags::set(MY_KEY, 42);
    /// assert_eq!(Flags::get_or_default(MY_KEY), 42);
    /// ```
    pub fn get_or_default<T: Clone + Send + Default + 'static>(key: FlagKey<T>) -> T {
        Self::get(key).unwrap_or_default()
    }

    /// Removes a flag value for the given key.
    ///
    /// Returns `true` if a value was removed, `false` if no value was set.
    ///
    /// # Example
    ///
    /// ```rust
    /// use teloxide_max::utils::flags::{FlagKey, Flags};
    ///
    /// static MY_KEY: FlagKey<i32> = FlagKey::new(0);
    /// Flags::set(MY_KEY, 42);
    /// assert!(Flags::remove(MY_KEY));
    /// assert!(!Flags::remove(MY_KEY));
    /// ```
    pub fn remove<T: Send + 'static>(key: FlagKey<T>) -> bool {
        FLAGS.with(|flags| {
            let mut flags = flags.borrow_mut();
            let type_id = TypeId::of::<T>();
            flags.remove(&(key.id, type_id)).is_some()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flag_set_and_get() {
        static KEY: FlagKey<i32> = FlagKey::new(100);

        Flags::set(KEY, 42);
        assert_eq!(Flags::get(KEY), Some(42));
    }

    #[test]
    fn flag_not_set_returns_none() {
        static KEY: FlagKey<String> = FlagKey::new(101);

        assert_eq!(Flags::get(KEY), None);
    }

    #[test]
    fn flag_remove() {
        static KEY: FlagKey<i32> = FlagKey::new(102);

        Flags::set(KEY, 42);
        assert!(Flags::remove(KEY));
        assert_eq!(Flags::get(KEY), None);
        assert!(!Flags::remove(KEY));
    }

    #[test]
    fn flag_get_or_default() {
        static KEY: FlagKey<i32> = FlagKey::new(103);

        assert_eq!(Flags::get_or_default(KEY), 0);
        Flags::set(KEY, 42);
        assert_eq!(Flags::get_or_default(KEY), 42);
    }

    #[test]
    fn flag_replace() {
        static KEY: FlagKey<i32> = FlagKey::new(104);

        Flags::set(KEY, 1);
        Flags::set(KEY, 2);
        assert_eq!(Flags::get(KEY), Some(2));
    }

    #[test]
    fn different_keys_independent() {
        static KEY1: FlagKey<i32> = FlagKey::new(105);
        static KEY2: FlagKey<i32> = FlagKey::new(106);

        Flags::set(KEY1, 1);
        Flags::set(KEY2, 2);
        assert_eq!(Flags::get(KEY1), Some(1));
        assert_eq!(Flags::get(KEY2), Some(2));
    }

    #[test]
    fn flag_string_value() {
        static KEY: FlagKey<String> = FlagKey::new(107);

        Flags::set(KEY, "hello".to_string());
        assert_eq!(Flags::get(KEY), Some("hello".to_string()));
    }

    #[test]
    fn different_types_same_id() {
        static KEY_I32: FlagKey<i32> = FlagKey::new(108);
        static KEY_STR: FlagKey<&str> = FlagKey::new(108);

        Flags::set(KEY_I32, 42);
        Flags::set(KEY_STR, "hello");

        assert_eq!(Flags::get(KEY_I32), Some(42));
        assert_eq!(Flags::get(KEY_STR), Some("hello"));
    }
}
