//! Event isolation for concurrent FSM updates (aiogram `BaseEventIsolation`
//! parity).
//!
//! When two updates for the same dialogue key arrive at once, handlers can race
//! and corrupt dialogue state. Isolation serializes handling per key.
//!
//! # Examples
//!
//! ```
//! use teloxide_max::{
//!     dispatching::{
//!         dialogue::strategy::DialogueKey,
//!         event_isolation::{EventIsolation, SimpleEventIsolation},
//!     },
//!     types::ChatId,
//! };
//!
//! # #[tokio::main(flavor = "current_thread")]
//! # async fn main() {
//! let isolation = SimpleEventIsolation::new();
//! let key = DialogueKey::from_chat(ChatId(1));
//! let _guard = isolation.lock(&key).await;
//! // exclusive critical section for this dialogue key
//! drop(_guard);
//! isolation.close().await;
//! # }
//! ```
//!
//! Teloxide also isolates concurrent work via
//! [`DispatcherBuilder::distribution_function`](crate::dispatching::DispatcherBuilder::distribution_function)
//! (one worker queue per key). Prefer that for pipeline-level isolation; use
//! this module when you need an explicit, aiogram-style lock around storage
//! access.

use std::{collections::HashMap, hash::Hash, sync::Arc};

use tokio::sync::{Mutex, OwnedMutexGuard};

use crate::dispatching::dialogue::strategy::DialogueKey;

/// Locks concurrent access for a storage / dialogue key.
///
/// Matches aiogram's `BaseEventIsolation` abstract lock API.
#[async_trait::async_trait]
pub trait EventIsolation: Send + Sync + 'static {
    /// Acquire an exclusive lock for `key`.
    ///
    /// Drop the returned guard to release the lock.
    async fn lock(&self, key: &DialogueKey) -> IsolationGuard;

    /// Release any resources held by this isolation backend.
    async fn close(&self);
}

/// RAII guard returned by [`EventIsolation::lock`].
///
/// Holding the guard keeps the key locked. Dropping it releases the lock.
pub struct IsolationGuard {
    _inner: IsolationGuardInner,
}

enum IsolationGuardInner {
    /// Real mutex guard from [`SimpleEventIsolation`].
    Mutex(OwnedMutexGuard<()>),
    /// No-op guard from [`DisabledEventIsolation`].
    Disabled,
}

/// No isolation — every lock acquires immediately (aiogram
/// `DisabledEventIsolation`).
#[derive(Debug, Default, Clone, Copy)]
pub struct DisabledEventIsolation;

impl DisabledEventIsolation {
    /// Creates a disabled isolation backend.
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl EventIsolation for DisabledEventIsolation {
    async fn lock(&self, _key: &DialogueKey) -> IsolationGuard {
        IsolationGuard { _inner: IsolationGuardInner::Disabled }
    }

    async fn close(&self) {}
}

/// In-process mutex map isolation (aiogram `SimpleEventIsolation`).
///
/// Uses one [`tokio::sync::Mutex`] per [`DialogueKey`]. Idle locks are retained
/// for the lifetime of the isolation object (cheap; avoid unbounded key churn
/// in long-lived processes with millions of unique keys).
#[derive(Debug, Default)]
pub struct SimpleEventIsolation {
    locks: Mutex<HashMap<DialogueKey, Arc<Mutex<()>>>>,
}

impl SimpleEventIsolation {
    /// Creates an empty isolation map.
    pub fn new() -> Self {
        Self::default()
    }

    async fn mutex_for(&self, key: &DialogueKey) -> Arc<Mutex<()>> {
        let mut map = self.locks.lock().await;
        Arc::clone(map.entry(key.clone()).or_insert_with(|| Arc::new(Mutex::new(()))))
    }
}

#[async_trait::async_trait]
impl EventIsolation for SimpleEventIsolation {
    async fn lock(&self, key: &DialogueKey) -> IsolationGuard {
        let mtx = self.mutex_for(key).await;
        let guard = mtx.lock_owned().await;
        IsolationGuard { _inner: IsolationGuardInner::Mutex(guard) }
    }

    async fn close(&self) {
        self.locks.lock().await.clear();
    }
}

/// Generic key isolation for custom hashable keys (advanced use).
#[derive(Debug, Default)]
pub struct KeyedEventIsolation<K: Eq + Hash + Clone + Send + Sync + 'static> {
    locks: Mutex<HashMap<K, Arc<Mutex<()>>>>,
}

impl<K: Eq + Hash + Clone + Send + Sync + 'static> KeyedEventIsolation<K> {
    /// Creates an empty keyed isolation map.
    pub fn new() -> Self {
        Self { locks: Mutex::new(HashMap::new()) }
    }

    /// Acquire an exclusive lock for an arbitrary key.
    pub async fn lock_key(&self, key: &K) -> OwnedMutexGuard<()> {
        let mtx = {
            let mut map = self.locks.lock().await;
            Arc::clone(map.entry(key.clone()).or_insert_with(|| Arc::new(Mutex::new(()))))
        };
        mtx.lock_owned().await
    }

    /// Drop all idle locks.
    pub async fn close(&self) {
        self.locks.lock().await.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::ChatId;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[tokio::test]
    async fn simple_serializes_same_key() {
        let isolation = Arc::new(SimpleEventIsolation::new());
        let key = DialogueKey::from_chat(ChatId(42));
        let counter = Arc::new(AtomicUsize::new(0));
        let max_seen = Arc::new(AtomicUsize::new(0));

        let mut handles = Vec::new();
        for _ in 0..8 {
            let isolation = Arc::clone(&isolation);
            let counter = Arc::clone(&counter);
            let max_seen = Arc::clone(&max_seen);
            let key = key.clone();
            handles.push(tokio::spawn(async move {
                let _g = isolation.lock(&key).await;
                let cur = counter.fetch_add(1, Ordering::SeqCst) + 1;
                max_seen.fetch_max(cur, Ordering::SeqCst);
                tokio::task::yield_now().await;
                counter.fetch_sub(1, Ordering::SeqCst);
            }));
        }
        for h in handles {
            h.await.unwrap();
        }
        assert_eq!(max_seen.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn disabled_allows_overlap() {
        let isolation = DisabledEventIsolation::new();
        let key = DialogueKey::from_chat(ChatId(1));
        let _a = isolation.lock(&key).await;
        let _b = isolation.lock(&key).await; // must not deadlock
    }
}
