//! Message content type classification (aiogram `ContentType` parity).
//!
//! Provides a single enum that classifies a [`Message`] by its primary content,
//! matching aiogram's `ContentType` filter usage.
//!
//! # Example
//!
//! ```rust
//! use teloxide_max::{types::Message, utils::content_type::ContentType};
//!
//! fn handle(msg: &Message) {
//!     match ContentType::of(msg) {
//!         ContentType::Text => { /* ... */ }
//!         ContentType::Photo => { /* ... */ }
//!         ContentType::Sticker => { /* ... */ }
//!         other => {
//!             let _ = other;
//!         }
//!     }
//! }
//! ```
//!
//! # Migration from aiogram
//!
//! ```python
//! # aiogram
//! from aiogram.enums import ContentType
//! @router.message(F.content_type == ContentType.TEXT)
//! async def on_text(message: Message): ...
//! ```
//!
//! ```rust
//! // teloxide_max
//! use teloxide_max::utils::content_type::ContentType;
//! dptree::filter(|msg: Message| ContentType::of(&msg) == ContentType::Text)
//! ```

use teloxide_max_core::types::{MediaKind, Message, MessageKind};

/// Classification of a message's primary content (aiogram `ContentType`
/// parity).
///
/// Derived from [`MessageKind`] / [`MediaKind`]. Use [`ContentType::of`] to
/// classify a message, then compare with `==` in filters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ContentType {
    /// Plain text message.
    Text,
    /// Animation (GIF / H.264 without sound).
    Animation,
    /// Audio file.
    Audio,
    /// Generic document / file.
    Document,
    /// Live photo.
    LivePhoto,
    /// Paid media.
    PaidMedia,
    /// Photo.
    Photo,
    /// Sticker.
    Sticker,
    /// Forwarded story.
    Story,
    /// Video.
    Video,
    /// Video note (round video).
    VideoNote,
    /// Voice message.
    Voice,
    /// Checklist.
    Checklist,
    /// Contact card.
    Contact,
    /// Dice.
    Dice,
    /// Game.
    Game,
    /// Poll.
    Poll,
    /// Venue.
    Venue,
    /// Location.
    Location,
    /// Rich message (TBA).
    RichMessage,
    /// Chat migration.
    Migration,
    /// New chat members service message.
    NewChatMembers,
    /// Left chat member service message.
    LeftChatMember,
    /// Chat owner left.
    ChatOwnerLeft,
    /// Chat owner changed.
    ChatOwnerChanged,
    /// New chat title.
    NewChatTitle,
    /// New chat photo.
    NewChatPhoto,
    /// Delete chat photo.
    DeleteChatPhoto,
    /// Group chat created.
    GroupChatCreated,
    /// Supergroup chat created.
    SupergroupChatCreated,
    /// Channel chat created.
    ChannelChatCreated,
    /// Auto-delete timer changed.
    MessageAutoDeleteTimerChanged,
    /// Migrate to chat id (via media migration).
    MigrateToChatId,
    /// Migrate from chat id (via media migration).
    MigrateFromChatId,
    /// Pinned message.
    PinnedMessage,
    /// Invoice.
    Invoice,
    /// Successful payment.
    SuccessfulPayment,
    /// Refunded payment.
    RefundedPayment,
    /// Users shared.
    UsersShared,
    /// Chat shared.
    ChatShared,
    /// Gift.
    Gift,
    /// Unique gift.
    UniqueGift,
    /// Gift upgrade sent.
    GiftUpgradeSent,
    /// Connected website.
    ConnectedWebsite,
    /// Write access allowed.
    WriteAccessAllowed,
    /// Passport data.
    PassportData,
    /// Proximity alert.
    ProximityAlertTriggered,
    /// Boost added.
    BoostAdded,
    /// Chat background set.
    ChatBackgroundSet,
    /// Checklist tasks done.
    ChecklistTasksDone,
    /// Checklist tasks added.
    ChecklistTasksAdded,
    /// Direct message price changed.
    DirectMessagePriceChanged,
    /// Forum topic created.
    ForumTopicCreated,
    /// Forum topic edited.
    ForumTopicEdited,
    /// Forum topic closed.
    ForumTopicClosed,
    /// Forum topic reopened.
    ForumTopicReopened,
    /// General forum topic hidden.
    GeneralForumTopicHidden,
    /// General forum topic unhidden.
    GeneralForumTopicUnhidden,
    /// Giveaway created.
    GiveawayCreated,
    /// Giveaway.
    Giveaway,
    /// Giveaway winners.
    GiveawayWinners,
    /// Giveaway completed.
    GiveawayCompleted,
    /// Managed bot created (service).
    ManagedBotCreated,
    /// Managed bot updated (service).
    ManagedBotUpdated,
    /// Paid message price changed.
    PaidMessagePriceChanged,
    /// Poll option added.
    PollOptionAdded,
    /// Poll option deleted.
    PollOptionDeleted,
    /// Suggested post approved.
    SuggestedPostApproved,
    /// Suggested post approval failed.
    SuggestedPostApprovalFailed,
    /// Suggested post declined.
    SuggestedPostDeclined,
    /// Suggested post paid.
    SuggestedPostPaid,
    /// Suggested post refunded.
    SuggestedPostRefunded,
    /// Video chat scheduled.
    VideoChatScheduled,
    /// Video chat started.
    VideoChatStarted,
    /// Video chat ended.
    VideoChatEnded,
    /// Video chat participants invited.
    VideoChatParticipantsInvited,
    /// Web app data.
    WebAppData,
    /// Community chat added.
    CommunityChatAdded,
    /// Community chat removed.
    CommunityChatRemoved,
    /// Unknown / unrecognized content.
    Unknown,
    /// Wildcard: matches any content (for filter composition).
    Any,
}

impl ContentType {
    /// Classify a message by its primary content.
    pub fn of(msg: &Message) -> Self {
        match &msg.kind {
            MessageKind::Common(common) => match &common.media_kind {
                MediaKind::Text(_) => Self::Text,
                MediaKind::Animation(_) => Self::Animation,
                MediaKind::Audio(_) => Self::Audio,
                MediaKind::Document(_) => Self::Document,
                MediaKind::LivePhoto(_) => Self::LivePhoto,
                MediaKind::PaidMedia(_) => Self::PaidMedia,
                MediaKind::Photo(_) => Self::Photo,
                MediaKind::Sticker(_) => Self::Sticker,
                MediaKind::Story(_) => Self::Story,
                MediaKind::Video(_) => Self::Video,
                MediaKind::VideoNote(_) => Self::VideoNote,
                MediaKind::Voice(_) => Self::Voice,
                MediaKind::Checklist(_) => Self::Checklist,
                MediaKind::Contact(_) => Self::Contact,
                MediaKind::Game(_) => Self::Game,
                MediaKind::Poll(_) => Self::Poll,
                MediaKind::Venue(_) => Self::Venue,
                MediaKind::Location(_) => Self::Location,
                MediaKind::RichMessage(_) => Self::RichMessage,
                MediaKind::Migration(m) => match m {
                    teloxide_max_core::types::ChatMigration::To { .. } => Self::MigrateToChatId,
                    teloxide_max_core::types::ChatMigration::From { .. } => Self::MigrateFromChatId,
                },
            },
            MessageKind::NewChatMembers(_) => Self::NewChatMembers,
            MessageKind::LeftChatMember(_) => Self::LeftChatMember,
            MessageKind::NewChatTitle(_) => Self::NewChatTitle,
            MessageKind::NewChatPhoto(_) => Self::NewChatPhoto,
            MessageKind::DeleteChatPhoto(_) => Self::DeleteChatPhoto,
            MessageKind::GroupChatCreated(_) => Self::GroupChatCreated,
            MessageKind::SupergroupChatCreated(_) => Self::SupergroupChatCreated,
            MessageKind::ChannelChatCreated(_) => Self::ChannelChatCreated,
            MessageKind::MessageAutoDeleteTimerChanged(_) => Self::MessageAutoDeleteTimerChanged,
            MessageKind::Pinned(_) => Self::PinnedMessage,
            MessageKind::ChatShared(_) => Self::ChatShared,
            MessageKind::UsersShared(_) => Self::UsersShared,
            MessageKind::Invoice(_) => Self::Invoice,
            MessageKind::SuccessfulPayment(_) => Self::SuccessfulPayment,
            MessageKind::RefundedPayment(_) => Self::RefundedPayment,
            MessageKind::ConnectedWebsite(_) => Self::ConnectedWebsite,
            MessageKind::WriteAccessAllowed(_) => Self::WriteAccessAllowed,
            MessageKind::PassportData(_) => Self::PassportData,
            MessageKind::Dice(_) => Self::Dice,
            MessageKind::ProximityAlertTriggered(_) => Self::ProximityAlertTriggered,
            MessageKind::ChatBoostAdded(_) => Self::BoostAdded,
            MessageKind::ChatBackground(_) => Self::ChatBackgroundSet,
            MessageKind::ChecklistTasksDone(_) => Self::ChecklistTasksDone,
            MessageKind::ChecklistTasksAdded(_) => Self::ChecklistTasksAdded,
            MessageKind::DirectMessagePriceChanged(_) => Self::DirectMessagePriceChanged,
            MessageKind::ForumTopicCreated(_) => Self::ForumTopicCreated,
            MessageKind::ForumTopicEdited(_) => Self::ForumTopicEdited,
            MessageKind::ForumTopicClosed(_) => Self::ForumTopicClosed,
            MessageKind::ForumTopicReopened(_) => Self::ForumTopicReopened,
            MessageKind::GeneralForumTopicHidden(_) => Self::GeneralForumTopicHidden,
            MessageKind::GeneralForumTopicUnhidden(_) => Self::GeneralForumTopicUnhidden,
            MessageKind::Giveaway(_) => Self::Giveaway,
            MessageKind::GiveawayCompleted(_) => Self::GiveawayCompleted,
            MessageKind::GiveawayCreated(_) => Self::GiveawayCreated,
            MessageKind::GiveawayWinners(_) => Self::GiveawayWinners,
            MessageKind::PaidMessagePriceChanged(_) => Self::PaidMessagePriceChanged,
            MessageKind::SuggestedPostApproved(_) => Self::SuggestedPostApproved,
            MessageKind::SuggestedPostApprovalFailed(_) => Self::SuggestedPostApprovalFailed,
            MessageKind::SuggestedPostDeclined(_) => Self::SuggestedPostDeclined,
            MessageKind::SuggestedPostPaid(_) => Self::SuggestedPostPaid,
            MessageKind::SuggestedPostRefunded(_) => Self::SuggestedPostRefunded,
            MessageKind::GiftInfo(_) => Self::Gift,
            MessageKind::PollOptionAdded(_) => Self::PollOptionAdded,
            MessageKind::PollOptionDeleted(_) => Self::PollOptionDeleted,
            MessageKind::ManagedBotUpdated(_) => Self::ManagedBotUpdated,
            MessageKind::ManagedBotCreated(_) => Self::ManagedBotCreated,
            MessageKind::CommunityChatRemoved(_) => Self::CommunityChatRemoved,
            MessageKind::CommunityChatAdded(_) => Self::CommunityChatAdded,
            MessageKind::GiftUpgradeSent(_) => Self::GiftUpgradeSent,
            MessageKind::ChatOwnerChanged(_) => Self::ChatOwnerChanged,
            MessageKind::ChatOwnerLeft(_) => Self::ChatOwnerLeft,
            MessageKind::UniqueGiftInfo(_) => Self::UniqueGift,
            MessageKind::VideoChatScheduled(_) => Self::VideoChatScheduled,
            MessageKind::VideoChatStarted(_) => Self::VideoChatStarted,
            MessageKind::VideoChatEnded(_) => Self::VideoChatEnded,
            MessageKind::VideoChatParticipantsInvited(_) => Self::VideoChatParticipantsInvited,
            MessageKind::WebAppData(_) => Self::WebAppData,
            MessageKind::Empty {} => Self::Unknown,
        }
    }

    /// Returns `true` if this type matches `other`, treating
    /// [`ContentType::Any`] as a wildcard.
    pub fn matches(self, other: ContentType) -> bool {
        self == ContentType::Any || other == ContentType::Any || self == other
    }

    /// Predicate for dptree filters: `ContentType::Text.predicate()`.
    pub fn predicate(self) -> impl Fn(&Message) -> bool + Send + Sync + 'static {
        move |msg: &Message| ContentType::of(msg).matches(self)
    }

    /// Snake-case name matching aiogram enum values (e.g. `"text"`, `"photo"`).
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Animation => "animation",
            Self::Audio => "audio",
            Self::Document => "document",
            Self::LivePhoto => "live_photo",
            Self::PaidMedia => "paid_media",
            Self::Photo => "photo",
            Self::Sticker => "sticker",
            Self::Story => "story",
            Self::Video => "video",
            Self::VideoNote => "video_note",
            Self::Voice => "voice",
            Self::Checklist => "checklist",
            Self::Contact => "contact",
            Self::Dice => "dice",
            Self::Game => "game",
            Self::Poll => "poll",
            Self::Venue => "venue",
            Self::Location => "location",
            Self::RichMessage => "rich_message",
            Self::Migration => "migration",
            Self::NewChatMembers => "new_chat_members",
            Self::LeftChatMember => "left_chat_member",
            Self::ChatOwnerLeft => "chat_owner_left",
            Self::ChatOwnerChanged => "chat_owner_changed",
            Self::NewChatTitle => "new_chat_title",
            Self::NewChatPhoto => "new_chat_photo",
            Self::DeleteChatPhoto => "delete_chat_photo",
            Self::GroupChatCreated => "group_chat_created",
            Self::SupergroupChatCreated => "supergroup_chat_created",
            Self::ChannelChatCreated => "channel_chat_created",
            Self::MessageAutoDeleteTimerChanged => "message_auto_delete_timer_changed",
            Self::MigrateToChatId => "migrate_to_chat_id",
            Self::MigrateFromChatId => "migrate_from_chat_id",
            Self::PinnedMessage => "pinned_message",
            Self::Invoice => "invoice",
            Self::SuccessfulPayment => "successful_payment",
            Self::RefundedPayment => "refunded_payment",
            Self::UsersShared => "users_shared",
            Self::ChatShared => "chat_shared",
            Self::Gift => "gift",
            Self::UniqueGift => "unique_gift",
            Self::GiftUpgradeSent => "gift_upgrade_sent",
            Self::ConnectedWebsite => "connected_website",
            Self::WriteAccessAllowed => "write_access_allowed",
            Self::PassportData => "passport_data",
            Self::ProximityAlertTriggered => "proximity_alert_triggered",
            Self::BoostAdded => "boost_added",
            Self::ChatBackgroundSet => "chat_background_set",
            Self::ChecklistTasksDone => "checklist_tasks_done",
            Self::ChecklistTasksAdded => "checklist_tasks_added",
            Self::DirectMessagePriceChanged => "direct_message_price_changed",
            Self::ForumTopicCreated => "forum_topic_created",
            Self::ForumTopicEdited => "forum_topic_edited",
            Self::ForumTopicClosed => "forum_topic_closed",
            Self::ForumTopicReopened => "forum_topic_reopened",
            Self::GeneralForumTopicHidden => "general_forum_topic_hidden",
            Self::GeneralForumTopicUnhidden => "general_forum_topic_unhidden",
            Self::GiveawayCreated => "giveaway_created",
            Self::Giveaway => "giveaway",
            Self::GiveawayWinners => "giveaway_winners",
            Self::GiveawayCompleted => "giveaway_completed",
            Self::ManagedBotCreated => "managed_bot_created",
            Self::ManagedBotUpdated => "managed_bot_updated",
            Self::PaidMessagePriceChanged => "paid_message_price_changed",
            Self::PollOptionAdded => "poll_option_added",
            Self::PollOptionDeleted => "poll_option_deleted",
            Self::SuggestedPostApproved => "suggested_post_approved",
            Self::SuggestedPostApprovalFailed => "suggested_post_approval_failed",
            Self::SuggestedPostDeclined => "suggested_post_declined",
            Self::SuggestedPostPaid => "suggested_post_paid",
            Self::SuggestedPostRefunded => "suggested_post_refunded",
            Self::VideoChatScheduled => "video_chat_scheduled",
            Self::VideoChatStarted => "video_chat_started",
            Self::VideoChatEnded => "video_chat_ended",
            Self::VideoChatParticipantsInvited => "video_chat_participants_invited",
            Self::WebAppData => "web_app_data",
            Self::CommunityChatAdded => "community_chat_added",
            Self::CommunityChatRemoved => "community_chat_removed",
            Self::Unknown => "unknown",
            Self::Any => "any",
        }
    }
}

/// Extension trait: `msg.content_type()`.
pub trait MessageContentTypeExt {
    /// Returns the [`ContentType`] of this message.
    fn content_type(&self) -> ContentType;
}

impl MessageContentTypeExt for Message {
    fn content_type(&self) -> ContentType {
        ContentType::of(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn text_message(text: &str) -> Message {
        let json = serde_json::json!({
            "message_id": 1,
            "from": {
                "id": 1,
                "is_bot": false,
                "first_name": "Test",
            },
            "chat": {
                "id": 1,
                "type": "private",
                "first_name": "Test",
            },
            "date": 1_569_518_829_i64,
            "text": text,
        });
        serde_json::from_value(json).expect("failed to deserialize test Message")
    }

    fn photo_message() -> Message {
        let json = serde_json::json!({
            "message_id": 2,
            "from": { "id": 1, "is_bot": false, "first_name": "Test" },
            "chat": { "id": 1, "type": "private", "first_name": "Test" },
            "date": 1_569_518_829_i64,
            "photo": [
                {
                    "file_id": "AgADBAAD",
                    "file_unique_id": "AQADxyz",
                    "width": 90,
                    "height": 90,
                }
            ],
        });
        serde_json::from_value(json).expect("failed to deserialize photo Message")
    }

    #[test]
    fn classifies_text() {
        let msg = text_message("hello");
        assert_eq!(ContentType::of(&msg), ContentType::Text);
        assert_eq!(msg.content_type(), ContentType::Text);
        assert_eq!(ContentType::Text.as_str(), "text");
        assert!(ContentType::Text.matches(ContentType::Any));
        assert!(ContentType::of(&msg).matches(ContentType::Text));
    }

    #[test]
    fn classifies_photo() {
        let msg = photo_message();
        assert_eq!(ContentType::of(&msg), ContentType::Photo);
        assert_eq!(ContentType::Photo.as_str(), "photo");
    }

    #[test]
    fn predicate_works() {
        let msg = text_message("hi");
        assert!(ContentType::Text.predicate()(&msg));
        assert!(!ContentType::Photo.predicate()(&msg));
    }
}
