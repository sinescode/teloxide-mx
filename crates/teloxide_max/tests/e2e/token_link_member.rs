//! End-to-end tests for token, link, and chat-member-updated utilities.

use teloxide_max::{
    types::{
        Administrator, Banned, ChatMember, ChatMemberKind, Member, Owner, Restricted, UntilDate,
        User, UserId,
    },
    utils::{
        chat_member_updated::{
            join_transition, leave_transition, promoted_transition, ADMINISTRATOR, CREATOR,
            IS_ADMIN, IS_MEMBER, IS_NOT_MEMBER, JOIN_TRANSITION, KICKED, LEFT, MEMBER,
            RESTRICTED_MEMBER, RESTRICTED_NOT_MEMBER,
        },
        link::{
            create_channel_bot_link, create_telegram_link, create_tg_link, create_user_id_link,
            create_username_link, ChannelBotPermissions,
        },
        token::{extract_bot_id, validate_token, TokenValidationError},
        web_app_signature::{check_webapp_signature, PRODUCTION_PUBLIC_KEY, TEST_PUBLIC_KEY},
    },
};

fn test_user() -> User {
    User {
        id: UserId(42),
        is_bot: false,
        first_name: "Alice".into(),
        last_name: None,
        username: Some("alice".into()),
        language_code: None,
        is_premium: false,
        added_to_attachment_menu: false,
        has_topics_enabled: false,
        allows_users_to_create_topics: false,
        can_join_groups: None,
        can_read_all_group_messages: None,
        supports_guest_queries: None,
        supports_inline_queries: None,
        can_connect_to_business: None,
        has_main_web_app: None,
        can_manage_bots: None,
        supports_join_request_queries: None,
    }
}

fn cm(kind: ChatMemberKind) -> ChatMember {
    ChatMember { user: test_user(), kind }
}

// ─── Token ───────────────────────────────────────────────────────────────────

#[test]
fn token_valid() {
    assert!(validate_token("123456:ABC-DEF").is_ok());
    assert_eq!(extract_bot_id("123456:ABC-DEF").unwrap(), 123456);
}

#[test]
fn token_invalid_cases() {
    assert_eq!(validate_token(""), Err(TokenValidationError::Empty));
    assert_eq!(validate_token("123 456:ABC"), Err(TokenValidationError::ContainsWhitespace));
    assert_eq!(validate_token("notoken"), Err(TokenValidationError::InvalidFormat));
}

// ─── Link ────────────────────────────────────────────────────────────────────

#[test]
fn link_builders() {
    assert_eq!(create_tg_link("resolve", &[("domain", "durov")]), "tg://resolve?domain=durov");
    assert_eq!(create_telegram_link(&["durov"], &[]), "https://t.me/durov");
    assert_eq!(create_username_link("@mybot"), "https://t.me/mybot");
    assert_eq!(create_user_id_link(1), "tg://user?id=1");

    let link = create_channel_bot_link(
        "my_bot",
        Some("payload"),
        ChannelBotPermissions { manage_chat: true, delete_messages: true, ..Default::default() },
    );
    assert!(link.starts_with("https://t.me/my_bot?"));
    assert!(link.contains("startgroup=payload"));
    assert!(link.contains("admin="));
}

// ─── Chat member transitions ─────────────────────────────────────────────────

#[test]
fn member_transitions() {
    let left = cm(ChatMemberKind::Left);
    let member = cm(ChatMemberKind::Member(Member { until_date: None, tag: None }));
    let admin = cm(ChatMemberKind::Administrator(Administrator {
        custom_title: None,
        is_anonymous: false,
        can_be_edited: true,
        can_manage_chat: true,
        can_change_info: true,
        can_post_messages: false,
        can_edit_messages: false,
        can_delete_messages: true,
        can_post_stories: false,
        can_edit_stories: false,
        can_delete_stories: false,
        can_manage_video_chats: true,
        can_invite_users: true,
        can_restrict_members: true,
        can_pin_messages: true,
        can_manage_topics: false,
        can_manage_direct_messages: false,
        can_manage_tags: false,
        can_promote_members: false,
    }));
    let owner = cm(ChatMemberKind::Owner(Owner { custom_title: None, is_anonymous: false }));
    let banned = cm(ChatMemberKind::Banned(Banned { until_date: UntilDate::Forever }));
    let restricted_in = cm(ChatMemberKind::Restricted(Restricted {
        tag: None,
        can_edit_tag: None,
        until_date: UntilDate::Forever,
        is_member: true,
        can_send_messages: true,
        can_send_audios: true,
        can_send_documents: true,
        can_send_photos: true,
        can_send_videos: true,
        can_send_video_notes: true,
        can_send_voice_notes: true,
        can_send_other_messages: true,
        can_add_web_page_previews: true,
        can_change_info: false,
        can_invite_users: false,
        can_pin_messages: false,
        can_manage_topics: false,
        can_send_polls: true,
        can_react_to_messages: false,
    }));
    let restricted_out = cm(ChatMemberKind::Restricted(Restricted {
        tag: None,
        can_edit_tag: None,
        until_date: UntilDate::Forever,
        is_member: false,
        can_send_messages: false,
        can_send_audios: false,
        can_send_documents: false,
        can_send_photos: false,
        can_send_videos: false,
        can_send_video_notes: false,
        can_send_voice_notes: false,
        can_send_other_messages: false,
        can_add_web_page_previews: false,
        can_change_info: false,
        can_invite_users: false,
        can_pin_messages: false,
        can_manage_topics: false,
        can_send_polls: false,
        can_react_to_messages: false,
    }));

    assert!(LEFT.check(&left));
    assert!(MEMBER.check(&member));
    assert!(ADMINISTRATOR.check(&admin));
    assert!(CREATOR.check(&owner));
    assert!(KICKED.check(&banned));
    assert!(RESTRICTED_MEMBER.check(&restricted_in));
    assert!(RESTRICTED_NOT_MEMBER.check(&restricted_out));

    assert!(join_transition().check(&left, &member));
    assert!(join_transition().check(&restricted_out, &restricted_in));
    assert!(leave_transition().check(&member, &left));
    assert!(promoted_transition().check(&member, &admin));

    assert!(JOIN_TRANSITION.check(&left, &member));
    assert!(IS_MEMBER.check(&member));
    assert!(IS_MEMBER.check(&restricted_in));
    assert!(IS_NOT_MEMBER.check(&left));
    assert!(IS_NOT_MEMBER.check(&restricted_out));
    assert!(IS_ADMIN.check(&admin));
    assert!(IS_ADMIN.check(&owner));
}

// ─── WebApp Ed25519 signature surface ────────────────────────────────────────

#[test]
fn webapp_signature_keys_and_rejects() {
    assert_eq!(PRODUCTION_PUBLIC_KEY.len(), 32);
    assert_eq!(TEST_PUBLIC_KEY.len(), 32);
    assert!(!check_webapp_signature(1, "", PRODUCTION_PUBLIC_KEY));
    assert!(!check_webapp_signature(1, "auth_date=1&hash=x", PRODUCTION_PUBLIC_KEY));
}
