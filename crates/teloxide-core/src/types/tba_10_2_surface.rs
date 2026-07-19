//! Unit tests for TBA 9.3–10.2 surface ported from aiogram.
//!
//! Field-name tests serialize real payload structs and assert JSON keys match
//! the aiogram/TBA wire names (not inventing alternate shapes).

#[cfg(test)]
mod tests {
    use crate::payloads::{
        AnswerChatJoinRequestQuery, DeleteAllMessageReactions, DeleteEphemeralMessage,
        DeleteMessageReaction, GetChatGifts, GetUserGifts, GetUserPersonalChatMessages,
        RepostStory, SavePreparedKeyboardButton, SendChatJoinRequestWebApp, SendMessageDraft,
        SetChatMemberTag, SetManagedBotAccessSettings,
    };
    use crate::requests::Payload;
    use crate::types::{
        BotAccessSettings, BotCommand, BotSubscriptionUpdated, BusinessConnectionId, ChatId,
        Community, GuestQueryId, InputRichMessage, KeyboardButton, LivePhoto, MessageEntity,
        MessageEntityKind, MessageId, Recipient, RichMessage, Seconds, StoryId, User, UserId,
        UserRating,
    };

    #[test]
    fn send_message_draft_wire_fields() {
        let mut p = SendMessageDraft::new(ChatId(1), 42);
        p.text = Some("partial…".into());
        assert_eq!(SendMessageDraft::NAME, "SendMessageDraft");
        let json = serde_json::to_value(&p).unwrap();
        assert_eq!(json["chat_id"], 1);
        assert_eq!(json["draft_id"], 42);
        assert_eq!(json["text"], "partial…");
        assert!(json.get("message_thread_id").is_none() || json["message_thread_id"].is_null());
    }

    #[test]
    fn answer_chat_join_request_query_matches_aiogram_fields() {
        // aiogram: chat_join_request_query_id + result (approve|decline|queue)
        let p = AnswerChatJoinRequestQuery::new("cjq-1", "approve");
        let json = serde_json::to_value(&p).unwrap();
        assert!(json.get("query_id").is_none(), "must not use invented query_id");
        assert!(json.get("ok").is_none(), "must not use invented ok");
        assert_eq!(json["chat_join_request_query_id"], "cjq-1");
        assert_eq!(json["result"], "approve");
        assert_eq!(AnswerChatJoinRequestQuery::NAME, "AnswerChatJoinRequestQuery");
    }

    #[test]
    fn send_chat_join_request_web_app_matches_aiogram_fields() {
        // aiogram: chat_join_request_query_id + web_app_url -> bool
        let p = SendChatJoinRequestWebApp::new("cjq-2", "https://example.com/app");
        let json = serde_json::to_value(&p).unwrap();
        assert!(json.get("chat_id").is_none(), "must not use invented chat_id");
        assert!(json.get("web_app").is_none(), "must not use invented web_app object");
        assert_eq!(json["chat_join_request_query_id"], "cjq-2");
        assert_eq!(json["web_app_url"], "https://example.com/app");
        assert_eq!(SendChatJoinRequestWebApp::NAME, "SendChatJoinRequestWebApp");
    }

    #[test]
    fn delete_all_message_reactions_matches_aiogram_fields() {
        // aiogram: chat_id + optional user_id/actor_chat_id (no message_id)
        let mut p = DeleteAllMessageReactions::new(Recipient::Id(ChatId(-100)));
        p.user_id = Some(UserId(5));
        p.actor_chat_id = Some(ChatId(-200));
        let json = serde_json::to_value(&p).unwrap();
        assert!(json.get("message_id").is_none(), "must not require/send message_id");
        assert_eq!(json["chat_id"], -100);
        assert_eq!(json["user_id"], 5);
        assert_eq!(json["actor_chat_id"], -200);
    }

    #[test]
    fn delete_message_reaction_has_actor_chat_id() {
        let mut p = DeleteMessageReaction::new(Recipient::Id(ChatId(1)), MessageId(9));
        p.user_id = Some(UserId(3));
        p.actor_chat_id = Some(ChatId(4));
        let json = serde_json::to_value(&p).unwrap();
        assert_eq!(json["chat_id"], 1);
        // MessageId may serialize via helper as int or nested; accept either
        assert_eq!(json["message_id"], 9);
        assert_eq!(json["user_id"], 3);
        assert_eq!(json["actor_chat_id"], 4);
    }

    #[test]
    fn repost_story_uses_from_story_id() {
        let p = RepostStory::new(
            BusinessConnectionId("bc".into()),
            ChatId(10),
            StoryId(99),
            Seconds::from_seconds(86400),
        );
        let json = serde_json::to_value(&p).unwrap();
        assert!(json.get("story_id").is_none(), "wire name is from_story_id not story_id");
        assert_eq!(json["from_story_id"], 99);
        assert_eq!(json["from_chat_id"], 10);
        assert_eq!(json["business_connection_id"], "bc");
    }

    #[test]
    fn set_managed_bot_access_settings_flat_fields() {
        // aiogram: user_id + is_access_restricted + optional added_user_ids
        let mut p = SetManagedBotAccessSettings::new(UserId(1), true);
        p.added_user_ids = Some(vec![UserId(2), UserId(3)]);
        let json = serde_json::to_value(&p).unwrap();
        assert!(json.get("settings").is_none(), "must not nest settings object");
        assert_eq!(json["user_id"], 1);
        assert_eq!(json["is_access_restricted"], true);
        assert_eq!(json["added_user_ids"], serde_json::json!([2, 3]));
    }

    #[test]
    fn get_user_personal_chat_messages_limit_required() {
        let p = GetUserPersonalChatMessages::new(UserId(7), 10);
        let json = serde_json::to_value(&p).unwrap();
        assert!(json.get("offset").is_none(), "offset not in aiogram");
        assert_eq!(json["user_id"], 7);
        assert_eq!(json["limit"], 10);
    }

    #[test]
    fn save_prepared_keyboard_button_no_invented_allow_fields() {
        let button = KeyboardButton::new("Pick");
        let p = SavePreparedKeyboardButton::new(UserId(1), button);
        let json = serde_json::to_value(&p).unwrap();
        for bad in [
            "allow_user_chats",
            "allow_bot_chats",
            "allow_group_chats",
            "allow_channel_chats",
        ] {
            assert!(json.get(bad).is_none(), "invented field {bad}");
        }
        assert_eq!(json["user_id"], 1);
        assert!(json.get("button").is_some());
    }

    #[test]
    fn bot_access_settings_matches_aiogram() {
        // aiogram: is_access_restricted + optional added_users
        let s: BotAccessSettings = serde_json::from_str(
            r#"{"is_access_restricted":true,"added_users":[{"id":1,"is_bot":false,"first_name":"A"}]}"#,
        )
        .unwrap();
        assert!(s.is_access_restricted);
        assert_eq!(s.added_users.as_ref().unwrap().len(), 1);
        let out = serde_json::to_value(&s).unwrap();
        for bad in [
            "can_join_groups",
            "can_read_all_group_messages",
            "supports_inline_queries",
            "can_connect_to_business",
        ] {
            assert!(out.get(bad).is_none(), "invented field {bad}");
        }
        assert_eq!(out["is_access_restricted"], true);
    }

    #[test]
    fn get_user_gifts_and_chat_gifts_payload_names() {
        let u = GetUserGifts::new(UserId(7));
        assert_eq!(GetUserGifts::NAME, "GetUserGifts");
        let c = GetChatGifts::new(Recipient::Id(ChatId(-100)));
        assert_eq!(GetChatGifts::NAME, "GetChatGifts");
        let uj = serde_json::to_value(&u).unwrap();
        assert_eq!(uj["user_id"], 7);
        let cj = serde_json::to_value(&c).unwrap();
        assert_eq!(cj["chat_id"], -100);
    }

    #[test]
    fn set_chat_member_tag_and_ephemeral_delete() {
        let mut t = SetChatMemberTag::new(Recipient::Id(ChatId(1)), UserId(2));
        t.tag = Some("vip".into());
        assert_eq!(SetChatMemberTag::NAME, "SetChatMemberTag");
        let tj = serde_json::to_value(&t).unwrap();
        assert_eq!(tj["tag"], "vip");

        let d = DeleteEphemeralMessage::new(Recipient::Id(ChatId(1)), UserId(2), 99);
        assert_eq!(DeleteEphemeralMessage::NAME, "DeleteEphemeralMessage");
        let dj = serde_json::to_value(&d).unwrap();
        assert_eq!(dj["ephemeral_message_id"], 99);
        assert_eq!(dj["receiver_user_id"], 2);
    }

    #[test]
    fn guest_query_id_and_input_rich_message() {
        let _ = GuestQueryId::from("gq-1");
        let rich = InputRichMessage {
            content: Some("<b>hi</b>".into()),
            parse_mode: None,
            blocks: None,
            media: None,
        };
        let s = serde_json::to_value(&rich).unwrap();
        assert_eq!(s["content"], "<b>hi</b>");
    }

    #[test]
    fn live_photo_and_rich_message_deser() {
        let lp: LivePhoto = serde_json::from_str(
            r#"{
                "file_id": "vid",
                "file_unique_id": "uniq",
                "width": 100,
                "height": 200,
                "duration": 3
            }"#,
        )
        .unwrap();
        assert_eq!(lp.width, 100);
        assert_eq!(lp.duration, 3);

        let rm: RichMessage = serde_json::from_str(r#"{"blocks":[]}"#).unwrap();
        assert!(rm.blocks.is_empty());
    }

    #[test]
    fn user_rating_community_bot_command_ephemeral() {
        let r: UserRating = serde_json::from_str(
            r#"{"level":1,"rating":10,"current_level_rating":0,"next_level_rating":20}"#,
        )
        .unwrap();
        assert_eq!(r.level, 1);

        let c: Community = serde_json::from_str(r#"{"id":5,"title":"Comm"}"#).unwrap();
        assert_eq!(c.id, 5);

        let cmd =
            BotCommand { command: "start".into(), description: "Start".into(), is_ephemeral: true };
        let j = serde_json::to_value(&cmd).unwrap();
        assert_eq!(j["is_ephemeral"], true);
    }

    #[test]
    fn message_entity_date_time_kind_roundtrip() {
        let e = MessageEntity {
            kind: MessageEntityKind::DateTime {
                unix_time: Some(1_700_000_000),
                date_time_format: Some("%Y".into()),
            },
            offset: 0,
            length: 4,
        };
        let v = serde_json::to_value(&e).unwrap();
        assert_eq!(v["type"], "date_time");
        let back: MessageEntity = serde_json::from_value(v).unwrap();
        match back.kind {
            MessageEntityKind::DateTime { unix_time, .. } => {
                assert_eq!(unix_time, Some(1_700_000_000));
            }
            other => panic!("unexpected kind: {other:?}"),
        }
    }

    #[test]
    fn forward_and_copy_message_effect_id() {
        use crate::payloads::{CopyMessage, ForwardMessage};
        use crate::types::{EffectId, MessageId};
        let mut f = ForwardMessage::new(ChatId(1), ChatId(2), MessageId(3));
        f.message_effect_id = Some(EffectId("fx".into()));
        let j = serde_json::to_value(&f).unwrap();
        assert_eq!(j["message_effect_id"], "fx");

        let mut c = CopyMessage::new(ChatId(1), ChatId(2), MessageId(3));
        c.message_effect_id = Some(EffectId("fx2".into()));
        let j = serde_json::to_value(&c).unwrap();
        assert_eq!(j["message_effect_id"], "fx2");
    }

    #[test]
    fn send_poll_new_tba_fields_and_edit_rich() {
        use crate::payloads::{EditMessageText, SendPoll};
        use crate::types::{MessageId, PollType};
        let mut p = SendPoll::new(ChatId(1), "Q?", vec!["a".into(), "b".into()]);
        p.allows_revoting = Some(true);
        p.shuffle_options = Some(true);
        p.allow_adding_options = Some(true);
        p.hide_results_until_closes = Some(true);
        p.description = Some("desc".into());
        p.correct_option_ids = Some(vec![0, 1]);
        let j = serde_json::to_value(&p).unwrap();
        assert_eq!(j["allows_revoting"], true);
        assert_eq!(j["shuffle_options"], true);
        assert_eq!(j["description"], "desc");
        assert_eq!(j["correct_option_ids"], serde_json::json!([0, 1]));

        let mut e = EditMessageText::new(ChatId(1), MessageId(2), "hi");
        e.rich_message = Some(InputRichMessage {
            content: Some("<b>x</b>".into()),
            parse_mode: None,
            blocks: None,
            media: None,
        });
        let j = serde_json::to_value(&e).unwrap();
        assert!(j.get("rich_message").is_some());
    }

    #[test]
    fn poll_option_service_and_message_fields() {
        use crate::types::{PollOptionAdded, PollOptionDeleted};
        let a: PollOptionAdded = serde_json::from_str(
            r#"{"option_persistent_id":"p1","option_text":"A"}"#
        ).unwrap();
        assert_eq!(a.option_persistent_id, "p1");
        let d: PollOptionDeleted = serde_json::from_str(
            r#"{"option_persistent_id":"p2","option_text":"B"}"#
        ).unwrap();
        assert_eq!(d.option_text, "B");
    }

    #[test]
    fn bot_subscription_updated_deser() {
        let raw = r#"{
            "user": {"id":1,"is_bot":false,"first_name":"A"},
            "is_active": true,
            "date": 1700000000
        }"#;
        let u: BotSubscriptionUpdated = serde_json::from_str(raw).unwrap();
        assert!(u.is_active);
        assert_eq!(u.user.id, UserId(1));
        let _user: User = u.user;
    }
}
