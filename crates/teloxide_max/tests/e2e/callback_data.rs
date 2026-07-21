//! End-to-end tests for CallbackData.
//!
//! Tests the typed callback data system.

use teloxide_max::utils::callback_data::{CallbackData, CallbackDataExt};
use teloxide_max::types::CallbackQuery;

#[derive(Debug, Clone, CallbackData)]
#[callback_data(prefix = "action", sep = ":")]
enum TestAction {
    #[callback_data(action = "select")]
    Select { id: u32 },
    #[callback_data(action = "delete")]
    Delete { id: u32 },
    #[callback_data(action = "confirm")]
    Confirm,
}

#[test]
fn test_callback_data_pack() {
    let action = TestAction::Select { id: 42 };
    let packed = action.pack();
    assert!(packed.starts_with("action:"));
    assert!(packed.contains("select"));
}

#[test]
fn test_callback_data_unpack() {
    let packed = "action:select:42";
    let unpacked = TestAction::unpack(packed);
    assert!(unpacked.is_some());
}

#[test]
fn test_callback_data_roundtrip() {
    let original = TestAction::Select { id: 42 };
    let packed = original.pack();
    let unpacked = TestAction::unpack(&packed);
    assert!(unpacked.is_some());
}

#[test]
fn test_callback_data_invalid_prefix() {
    let packed = "wrong_prefix:select:42";
    let unpacked = TestAction::unpack(packed);
    assert!(unpacked.is_none());
}

#[test]
fn test_callback_data_invalid_action() {
    let packed = "action:unknown:42";
    let unpacked = TestAction::unpack(packed);
    assert!(unpacked.is_none());
}

#[test]
fn test_callback_data_confirm() {
    let action = TestAction::Confirm;
    let packed = action.pack();
    let unpacked = TestAction::unpack(&packed);
    assert!(unpacked.is_some());
}

#[test]
fn test_callback_query_filter() {
    let query = CallbackQuery {
        id: "test_id".into(),
        from: Default::default(),
        chat_instance: "test".into(),
        data: Some("action:select:42".into()),
        ..Default::default()
    };

    // Test that we can extract callback data
    assert_eq!(query.data.as_deref(), Some("action:select:42"));
}

#[test]
fn test_callback_data_empty() {
    let packed = "";
    let unpacked = TestAction::unpack(packed);
    assert!(unpacked.is_none());
}

#[test]
fn test_callback_data_complex_id() {
    let action = TestAction::Select { id: u32::MAX };
    let packed = action.pack();
    let unpacked = TestAction::unpack(&packed);
    assert!(unpacked.is_some());
}
