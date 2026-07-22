//! End-to-end tests for Router composition.
//!
//! Tests Router creation, composition, and handler routing.

use teloxide_max::{
    dispatching::{router::Router, UpdateFilterExt},
    types::Update,
};

#[test]
fn test_router_creation() {
    let router = Router::new("test");
    assert_eq!(router.name(), "test");
}

#[test]
fn test_router_merge() {
    let r1 = Router::new("r1");
    let r2 = Router::new("r2");
    let merged = r1.merge(r2);
    assert_eq!(merged.name(), "r1");
}

#[test]
fn test_router_include_router() {
    let mut parent = Router::new("parent");
    let child = Router::new("child");
    parent.include_router(child);
    // No panic means success
}

#[test]
fn test_router_compose_empty() {
    let handler = Router::compose(vec![]);
    // Handler should be created without panic
    let _ = handler;
}

#[test]
fn test_router_compose_single() {
    let r1 = Router::new("r1");
    let handler = Router::compose(vec![r1]);
    let _ = handler;
}

#[test]
fn test_router_compose_multiple() {
    let r1 = Router::new("r1");
    let r2 = Router::new("r2");
    let r3 = Router::new("r3");
    let handler = Router::compose(vec![r1, r2, r3]);
    let _ = handler;
}

#[test]
fn test_router_into_handler() {
    let router = Router::new("test");
    let handler = router.into_handler();
    let _ = handler;
}

#[test]
fn test_router_add_message_handler() {
    let mut router = Router::new("test");
    let handler = Update::filter_message().endpoint(|| async { Ok(()) });
    router.add_message_handler(handler);
    let _handler = router.into_handler();
}

#[test]
fn test_router_add_callback_handler() {
    let mut router = Router::new("test");
    let handler = Update::filter_callback_query().endpoint(|| async { Ok(()) });
    router.add_callback_handler(handler);
    let _handler = router.into_handler();
}

#[test]
fn test_router_add_inline_handler() {
    let mut router = Router::new("test");
    let handler = Update::filter_inline_query().endpoint(|| async { Ok(()) });
    router.add_inline_handler(handler);
    let _handler = router.into_handler();
}

#[test]
fn test_router_debug() {
    let router = Router::new("debug_test");
    let debug_str = format!("{router:?}");
    assert!(debug_str.contains("debug_test"));
}

#[test]
fn test_router_name() {
    let router = Router::new("my_router");
    assert_eq!(router.name(), "my_router");
}

#[test]
fn test_router_compose_with_handlers() {
    let mut r1 = Router::new("r1");
    r1.add_handler(Update::filter_message().endpoint(|| async { Ok(()) }));

    let mut r2 = Router::new("r2");
    r2.add_handler(Update::filter_callback_query().endpoint(|| async { Ok(()) }));

    let handler = Router::compose(vec![r1, r2]);
    let _ = handler;
}

#[test]
fn test_router_nested_composition() {
    let mut child1 = Router::new("child1");
    child1.add_handler(Update::filter_message().endpoint(|| async { Ok(()) }));

    let mut child2 = Router::new("child2");
    child2.add_handler(Update::filter_callback_query().endpoint(|| async { Ok(()) }));

    let mut parent = Router::new("parent");
    parent.include_router(child1);
    parent.include_router(child2);

    let handler = parent.into_handler();
    let _ = handler;
}
