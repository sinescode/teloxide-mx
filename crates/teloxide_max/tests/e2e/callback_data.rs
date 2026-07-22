//! End-to-end tests for CallbackData.

use teloxide_max::utils::callback_data::CallbackData;
use teloxide_max_macros::CallbackData;

#[derive(Debug, Clone, PartialEq, Eq, CallbackData)]
#[callback_data(prefix = "action")]
enum TestAction {
    #[callback_data(suffix = "select")]
    Select { id: u32 },
    #[callback_data(suffix = "delete")]
    Delete { id: u32 },
    #[callback_data(suffix = "confirm")]
    Confirm,
}

#[test]
fn test_callback_data_serialize() {
    let action = TestAction::Select { id: 42 };
    let packed = action.serialize().unwrap();
    assert!(packed.starts_with("action:"));
    assert!(packed.contains("select"));
    assert!(packed.contains("42"));
}

#[test]
fn test_callback_data_deserialize() {
    let packed = "action:select:42";
    let unpacked = TestAction::deserialize(packed).unwrap();
    assert_eq!(unpacked, TestAction::Select { id: 42 });
}

#[test]
fn test_callback_data_roundtrip() {
    let original = TestAction::Select { id: 42 };
    let packed = original.serialize().unwrap();
    let unpacked = TestAction::deserialize(&packed).unwrap();
    assert_eq!(original, unpacked);
}

#[test]
fn test_callback_data_invalid_prefix() {
    let packed = "wrong_prefix:select:42";
    assert!(TestAction::deserialize(packed).is_err());
}

#[test]
fn test_callback_data_invalid_action() {
    let packed = "action:unknown:42";
    assert!(TestAction::deserialize(packed).is_err());
}

#[test]
fn test_callback_data_confirm() {
    let action = TestAction::Confirm;
    let packed = action.serialize().unwrap();
    let unpacked = TestAction::deserialize(&packed).unwrap();
    assert_eq!(unpacked, TestAction::Confirm);
}

#[test]
fn test_callback_data_empty() {
    assert!(TestAction::deserialize("").is_err());
}

#[test]
fn test_callback_data_complex_id() {
    let action = TestAction::Select { id: u32::MAX };
    let packed = action.serialize().unwrap();
    let unpacked = TestAction::deserialize(&packed).unwrap();
    assert_eq!(unpacked, action);
}

#[test]
fn test_into_callback_string() {
    let s = TestAction::Confirm.into_callback_string().unwrap();
    assert_eq!(s, "action:confirm");
}
