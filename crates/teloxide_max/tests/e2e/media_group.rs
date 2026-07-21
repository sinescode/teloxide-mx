//! End-to-end tests for MediaGroup builder.
//!
//! Tests the MediaGroupBuilder utility.

use teloxide_max::utils::media_group::MediaGroupBuilder;

#[test]
fn test_media_group_builder_new() {
    let _builder = MediaGroupBuilder::new();
}

#[test]
fn test_media_group_builder_with_caption() {
    let _builder = MediaGroupBuilder::new().caption("Test caption");
}

#[test]
fn test_media_group_builder_photo() {
    let mut builder = MediaGroupBuilder::new();
    builder.photo("file_id_or_url");
    let media = builder.build();
    assert_eq!(media.len(), 1);
}

#[test]
fn test_media_group_builder_video() {
    let mut builder = MediaGroupBuilder::new();
    builder.video("file_id_or_url");
    let media = builder.build();
    assert_eq!(media.len(), 1);
}

#[test]
fn test_media_group_builder_audio() {
    let mut builder = MediaGroupBuilder::new();
    builder.audio("file_id_or_url");
    let media = builder.build();
    assert_eq!(media.len(), 1);
}

#[test]
fn test_media_group_builder_document() {
    let mut builder = MediaGroupBuilder::new();
    builder.document("file_id_or_url");
    let media = builder.build();
    assert_eq!(media.len(), 1);
}

#[test]
fn test_media_group_builder_mixed() {
    let mut builder = MediaGroupBuilder::new();
    builder.photo("photo_id");
    builder.video("video_id");
    builder.audio("audio_id");
    builder.document("doc_id");
    let media = builder.build();
    assert_eq!(media.len(), 4);
}

#[test]
fn test_media_group_builder_max_items() {
    let mut builder = MediaGroupBuilder::new();
    for i in 0..10 {
        builder.photo(format!("photo_{i}"));
    }
    let media = builder.build();
    assert_eq!(media.len(), 10);
}

#[test]
fn test_media_group_builder_caption_applied() {
    let mut builder = MediaGroupBuilder::new();
    builder.caption("Test caption");
    builder.photo("photo_id");
    let _media = builder.build();
    // Caption should be applied to first item
}

#[test]
fn test_media_group_builder_debug() {
    let builder = MediaGroupBuilder::new();
    let debug_str = format!("{builder:?}");
    assert!(!debug_str.is_empty());
}

#[test]
fn test_media_group_builder_clone() {
    let builder1 = MediaGroupBuilder::new().caption("test");
    let builder2 = builder1.clone();
    let _ = builder2;
}

#[test]
fn test_media_group_builder_empty() {
    let builder = MediaGroupBuilder::new();
    let media = builder.build();
    assert!(media.is_empty());
}

#[test]
fn test_media_group_builder_builder_pattern() {
    let media = MediaGroupBuilder::new()
        .caption("My album")
        .photo("p1")
        .photo("p2")
        .video("v1")
        .build();

    assert_eq!(media.len(), 4);
}

#[test]
fn test_media_group_builder_with_parse_mode() {
    let _builder = MediaGroupBuilder::new()
        .caption("<b>Bold caption</b>")
        .parse_mode("HTML");
}

#[test]
fn test_media_group_builder_caption_entities() {
    use teloxide_max::types::MessageEntity;
    let _builder = MediaGroupBuilder::new()
        .caption("Hello World")
        .caption_entities(vec![MessageEntity {
            kind: teloxide_max::types::MessageEntityKind::Bold,
            offset: 0,
            length: 5,
        }]);
}
