//! End-to-end tests for MediaGroup builder.

use teloxide_max::{
    types::{MessageEntity, MessageEntityKind, ParseMode},
    utils::media_group::MediaGroupBuilder,
};

#[test]
fn test_media_group_builder_new() {
    let builder = MediaGroupBuilder::new();
    assert!(builder.is_empty());
}

#[test]
fn test_media_group_builder_with_caption() {
    let media = MediaGroupBuilder::new().caption("Test caption").photo("file_id_1").build();
    assert_eq!(media.len(), 1);
}

#[test]
fn test_media_group_builder_photo() {
    let media = MediaGroupBuilder::new().photo("file_id_or_url").build();
    assert_eq!(media.len(), 1);
}

#[test]
fn test_media_group_builder_video() {
    let media = MediaGroupBuilder::new().video("file_id_or_url").build();
    assert_eq!(media.len(), 1);
}

#[test]
fn test_media_group_builder_audio() {
    let media = MediaGroupBuilder::new().audio("file_id_or_url").build();
    assert_eq!(media.len(), 1);
}

#[test]
fn test_media_group_builder_document() {
    let media = MediaGroupBuilder::new().document("file_id_or_url").build();
    assert_eq!(media.len(), 1);
}

#[test]
fn test_media_group_builder_mixed() {
    let media = MediaGroupBuilder::new()
        .photo("photo_id")
        .video("video_id")
        .audio("audio_id")
        .document("doc_id")
        .build();
    assert_eq!(media.len(), 4);
}

#[test]
fn test_media_group_builder_max_items() {
    let mut builder = MediaGroupBuilder::new();
    for i in 0..10 {
        builder = builder.photo(format!("photo_{i}"));
    }
    let media = builder.build();
    assert_eq!(media.len(), 10);
}

#[test]
fn test_media_group_builder_debug() {
    let builder = MediaGroupBuilder::new().caption("x");
    let debug_str = format!("{builder:?}");
    assert!(!debug_str.is_empty());
}

#[test]
fn test_media_group_builder_clone() {
    let builder1 = MediaGroupBuilder::new().caption("test");
    let builder2 = builder1.clone();
    assert_eq!(builder2.len(), 0);
}

#[test]
fn test_media_group_builder_empty() {
    let media = MediaGroupBuilder::new().build();
    assert!(media.is_empty());
}

#[test]
fn test_media_group_builder_builder_pattern() {
    let media =
        MediaGroupBuilder::new().caption("My album").photo("p1").photo("p2").video("v1").build();
    assert_eq!(media.len(), 3);
}

#[test]
fn test_media_group_builder_with_parse_mode() {
    let media = MediaGroupBuilder::new()
        .caption("<b>Bold caption</b>")
        .parse_mode(ParseMode::Html)
        .photo("p1")
        .build();
    assert_eq!(media.len(), 1);
}

#[test]
fn test_media_group_builder_caption_entities() {
    let media = MediaGroupBuilder::new()
        .caption("Hello World")
        .caption_entities(vec![MessageEntity {
            kind: MessageEntityKind::Bold,
            offset: 0,
            length: 5,
        }])
        .photo("p1")
        .build();
    assert_eq!(media.len(), 1);
}
