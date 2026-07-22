//! Chat member status transition filters (aiogram `ChatMemberUpdatedFilter`
//! parity).
//!
//! Provides expressive markers for filtering `ChatMemberUpdated` events by
//! old/new status — including join, leave, and promotion transitions.
//!
//! # Example
//!
//! ```rust,no_run
//! use teloxide_max::{
//!     prelude::*,
//!     types::ChatMemberUpdated,
//!     utils::chat_member_updated::{
//!         ChatMemberUpdatedFilter, JOIN_TRANSITION, LEAVE_TRANSITION, PROMOTED_TRANSITION,
//!     },
//! };
//!
//! let handler = Update::filter_chat_member()
//!     .branch(dptree::filter(|u: ChatMemberUpdated| JOIN_TRANSITION.check_update(&u)).endpoint(
//!         |bot: Bot, u: ChatMemberUpdated| async move {
//!             bot.send_message(u.chat.id, "Welcome!").await?;
//!             Ok(())
//!         },
//!     ))
//!     .branch(dptree::filter(|u: ChatMemberUpdated| LEAVE_TRANSITION.check_update(&u)).endpoint(
//!         |bot: Bot, u: ChatMemberUpdated| async move {
//!             bot.send_message(u.chat.id, "Goodbye!").await?;
//!             Ok(())
//!         },
//!     ))
//!     .branch(
//!         dptree::filter(|u: ChatMemberUpdated| {
//!             ChatMemberUpdatedFilter::new(PROMOTED_TRANSITION.clone()).check(&u)
//!         })
//!         .endpoint(|bot: Bot, u: ChatMemberUpdated| async move {
//!             bot.send_message(u.chat.id, "Congrats on the promotion!").await?;
//!             Ok(())
//!         }),
//!     );
//! # let _ = handler;
//! ```
//!
//! # Migration from aiogram
//!
//! ```python
//! # aiogram
//! from aiogram.filters import IS_MEMBER, JOIN_TRANSITION, ChatMemberUpdatedFilter
//! @router.chat_member(ChatMemberUpdatedFilter(JOIN_TRANSITION))
//! async def on_join(event: ChatMemberUpdated): ...
//! ```
//!
//! ```rust
//! // teloxide_max
//! use teloxide_max::utils::chat_member_updated::{ChatMemberUpdatedFilter, JOIN_TRANSITION};
//! dptree::filter(|u: ChatMemberUpdated| JOIN_TRANSITION.check_update(&u))
//! ```

use std::sync::LazyLock;

use teloxide_max_core::types::{ChatMember, ChatMemberKind, ChatMemberStatus, ChatMemberUpdated};

/// A single member status marker (optionally constrained by `is_member`).
///
/// Supports composition via [`MemberStatusMarker::or`] and transitions via
/// [`MemberStatusMarker::then`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MemberStatusMarker {
    /// Wire status name: `creator`, `administrator`, `member`, `restricted`,
    /// `left`, `kicked`.
    pub name: &'static str,
    /// When set, also require `ChatMemberKind::Restricted.is_member` to match.
    pub is_member: Option<bool>,
}

impl MemberStatusMarker {
    pub const fn new(name: &'static str) -> Self {
        Self { name, is_member: None }
    }

    /// Require `is_member == true` (for restricted members still present).
    pub const fn with_member(self) -> Self {
        Self { name: self.name, is_member: Some(true) }
    }

    /// Require `is_member == false` (for restricted members not present).
    pub const fn without_member(self) -> Self {
        Self { name: self.name, is_member: Some(false) }
    }

    /// Check whether a chat member matches this marker.
    pub fn check(&self, member: &ChatMember) -> bool {
        let status_ok = matches!(
            (self.name, member.kind.status()),
            ("creator", ChatMemberStatus::Owner)
                | ("administrator", ChatMemberStatus::Administrator)
                | ("member", ChatMemberStatus::Member)
                | ("restricted", ChatMemberStatus::Restricted)
                | ("left", ChatMemberStatus::Left)
                | ("kicked", ChatMemberStatus::Banned)
        );
        if !status_ok {
            return false;
        }
        if let Some(required) = self.is_member {
            match &member.kind {
                ChatMemberKind::Restricted(r) => r.is_member == required,
                // Other kinds do not expose `is_member`; only fail if a constraint was set.
                _ => true,
            }
        } else {
            true
        }
    }

    /// Union of this marker with another.
    pub fn or(self, other: MemberStatusMarker) -> MemberStatusGroup {
        MemberStatusGroup::from_markers(&[self, other])
    }

    /// Transition: this status → `other`.
    pub fn then(self, other: MemberStatusMarker) -> MemberStatusTransition {
        MemberStatusTransition {
            old: MemberStatusGroup::from_markers(&[self]),
            new: MemberStatusGroup::from_markers(&[other]),
        }
    }
}

/// A set of status markers (logical OR).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemberStatusGroup {
    statuses: Vec<MemberStatusMarker>,
}

impl MemberStatusGroup {
    pub fn from_markers(markers: &[MemberStatusMarker]) -> Self {
        assert!(!markers.is_empty(), "Member status group must not be empty");
        Self { statuses: markers.to_vec() }
    }

    pub fn or(mut self, other: MemberStatusMarker) -> Self {
        self.statuses.push(other);
        self
    }

    pub fn or_group(mut self, other: MemberStatusGroup) -> Self {
        self.statuses.extend(other.statuses);
        self
    }

    pub fn check(&self, member: &ChatMember) -> bool {
        self.statuses.iter().any(|s| s.check(member))
    }

    /// Transition from this group to another.
    pub fn then(self, other: MemberStatusGroup) -> MemberStatusTransition {
        MemberStatusTransition { old: self, new: other }
    }

    pub fn then_marker(self, other: MemberStatusMarker) -> MemberStatusTransition {
        self.then(MemberStatusGroup::from_markers(&[other]))
    }
}

/// A transition from one status set (`old`) to another (`new`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemberStatusTransition {
    pub old: MemberStatusGroup,
    pub new: MemberStatusGroup,
}

impl MemberStatusTransition {
    pub fn check(&self, old: &ChatMember, new: &ChatMember) -> bool {
        self.old.check(old) && self.new.check(new)
    }

    /// Invert the transition direction (`A >> B` becomes `B >> A`).
    pub fn invert(self) -> Self {
        Self { old: self.new, new: self.old }
    }

    /// Check against a full `ChatMemberUpdated` event.
    pub fn check_update(&self, update: &ChatMemberUpdated) -> bool {
        self.check(&update.old_chat_member, &update.new_chat_member)
    }
}

/// Filter matching aiogram's `ChatMemberUpdatedFilter`.
#[derive(Debug, Clone)]
pub enum ChatMemberUpdatedFilter {
    /// Match only the new member's status.
    Status(MemberStatusGroup),
    /// Match an old → new transition.
    Transition(MemberStatusTransition),
}

impl ChatMemberUpdatedFilter {
    pub fn new_status(group: MemberStatusGroup) -> Self {
        Self::Status(group)
    }

    pub fn new(transition: MemberStatusTransition) -> Self {
        Self::Transition(transition)
    }

    pub fn from_marker(marker: MemberStatusMarker) -> Self {
        Self::Status(MemberStatusGroup::from_markers(&[marker]))
    }

    pub fn check(&self, update: &ChatMemberUpdated) -> bool {
        match self {
            Self::Status(g) => g.check(&update.new_chat_member),
            Self::Transition(t) => t.check_update(update),
        }
    }
}

// ─── Predefined markers (aiogram names) ─────────────────────────────────────

/// Owner / creator.
pub const CREATOR: MemberStatusMarker = MemberStatusMarker::new("creator");
/// Administrator.
pub const ADMINISTRATOR: MemberStatusMarker = MemberStatusMarker::new("administrator");
/// Regular member.
pub const MEMBER: MemberStatusMarker = MemberStatusMarker::new("member");
/// Restricted member.
pub const RESTRICTED: MemberStatusMarker = MemberStatusMarker::new("restricted");
/// Left the chat.
pub const LEFT: MemberStatusMarker = MemberStatusMarker::new("left");
/// Banned / kicked.
pub const KICKED: MemberStatusMarker = MemberStatusMarker::new("kicked");

/// Restricted with `is_member = true`.
pub const RESTRICTED_MEMBER: MemberStatusMarker =
    MemberStatusMarker { name: "restricted", is_member: Some(true) };

/// Restricted with `is_member = false`.
pub const RESTRICTED_NOT_MEMBER: MemberStatusMarker =
    MemberStatusMarker { name: "restricted", is_member: Some(false) };

/// Currently a member of the chat (creator | admin | member | +restricted).
pub fn is_member_group() -> MemberStatusGroup {
    MemberStatusGroup::from_markers(&[CREATOR, ADMINISTRATOR, MEMBER, RESTRICTED_MEMBER])
}

/// Currently an admin (creator | administrator).
pub fn is_admin_group() -> MemberStatusGroup {
    MemberStatusGroup::from_markers(&[CREATOR, ADMINISTRATOR])
}

/// Not currently a member (left | kicked | -restricted).
pub fn is_not_member_group() -> MemberStatusGroup {
    MemberStatusGroup::from_markers(&[LEFT, KICKED, RESTRICTED_NOT_MEMBER])
}

/// Join transition: not-member → member.
pub fn join_transition() -> MemberStatusTransition {
    is_not_member_group().then(is_member_group())
}

/// Leave transition: member → not-member.
pub fn leave_transition() -> MemberStatusTransition {
    join_transition().invert()
}

/// Promoted to administrator (from member/restricted/left/kicked).
pub fn promoted_transition() -> MemberStatusTransition {
    MemberStatusGroup::from_markers(&[MEMBER, RESTRICTED, LEFT, KICKED])
        .then(MemberStatusGroup::from_markers(&[ADMINISTRATOR]))
}

/// Pre-built join transition (`IS_NOT_MEMBER >> IS_MEMBER`).
pub static JOIN_TRANSITION: LazyLock<MemberStatusTransition> = LazyLock::new(join_transition);
/// Pre-built leave transition (`IS_MEMBER >> IS_NOT_MEMBER`).
pub static LEAVE_TRANSITION: LazyLock<MemberStatusTransition> = LazyLock::new(leave_transition);
/// Pre-built promotion transition.
pub static PROMOTED_TRANSITION: LazyLock<MemberStatusTransition> =
    LazyLock::new(promoted_transition);
/// Pre-built “is member” status group.
pub static IS_MEMBER: LazyLock<MemberStatusGroup> = LazyLock::new(is_member_group);
/// Pre-built “is admin” status group.
pub static IS_ADMIN: LazyLock<MemberStatusGroup> = LazyLock::new(is_admin_group);
/// Pre-built “is not member” status group.
pub static IS_NOT_MEMBER: LazyLock<MemberStatusGroup> = LazyLock::new(is_not_member_group);

#[cfg(test)]
mod tests {
    use super::*;
    use teloxide_max_core::types::{
        Administrator, ChatMemberKind, Member, Owner, Restricted, UntilDate, User, UserId,
    };

    fn test_user() -> User {
        User {
            id: UserId(1),
            is_bot: false,
            first_name: "Test".into(),
            last_name: None,
            username: None,
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

    fn owner() -> ChatMember {
        cm(ChatMemberKind::Owner(Owner { custom_title: None, is_anonymous: false }))
    }

    fn regular_member() -> ChatMember {
        cm(ChatMemberKind::Member(Member { until_date: None, tag: None }))
    }

    fn left() -> ChatMember {
        cm(ChatMemberKind::Left)
    }

    fn admin() -> ChatMember {
        cm(ChatMemberKind::Administrator(Administrator {
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
        }))
    }

    fn restricted(is_member: bool) -> ChatMember {
        cm(ChatMemberKind::Restricted(Restricted {
            tag: None,
            can_edit_tag: None,
            until_date: UntilDate::Forever,
            is_member,
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
        }))
    }

    #[test]
    fn status_markers() {
        assert!(CREATOR.check(&owner()));
        assert!(MEMBER.check(&regular_member()));
        assert!(LEFT.check(&left()));
        assert!(ADMINISTRATOR.check(&admin()));
    }

    #[test]
    fn restricted_is_member_constraint() {
        let present = restricted(true);
        assert!(RESTRICTED_MEMBER.check(&present));
        assert!(!RESTRICTED_NOT_MEMBER.check(&present));
        assert!(RESTRICTED.check(&present));

        let absent = restricted(false);
        assert!(!RESTRICTED_MEMBER.check(&absent));
        assert!(RESTRICTED_NOT_MEMBER.check(&absent));
    }

    #[test]
    fn join_leave_promoted() {
        assert!(join_transition().check(&left(), &regular_member()));
        assert!(!join_transition().check(&regular_member(), &left()));
        assert!(leave_transition().check(&regular_member(), &left()));
        assert!(promoted_transition().check(&regular_member(), &admin()));
    }

    #[test]
    fn lazy_constants() {
        assert!(JOIN_TRANSITION.check(&left(), &regular_member()));
        assert!(LEAVE_TRANSITION.check(&regular_member(), &left()));
        assert!(PROMOTED_TRANSITION.check(&regular_member(), &admin()));
        assert!(IS_MEMBER.check(&regular_member()));
        assert!(IS_NOT_MEMBER.check(&left()));
        assert!(IS_ADMIN.check(&admin()));
        assert!(IS_ADMIN.check(&owner()));
    }

    #[test]
    fn filter_status_and_transition() {
        let status_filter = ChatMemberUpdatedFilter::from_marker(MEMBER);
        // Build a minimal ChatMemberUpdated via checking groups only (filter
        // path for transitions tested through check()).
        assert!(IS_MEMBER.check(&regular_member()));
        assert!(matches!(status_filter, ChatMemberUpdatedFilter::Status(_)));

        let t_filter = ChatMemberUpdatedFilter::new(join_transition());
        assert!(matches!(t_filter, ChatMemberUpdatedFilter::Transition(_)));
    }

    #[test]
    fn group_or_composition() {
        let group = CREATOR.or(ADMINISTRATOR).or(MEMBER);
        assert!(group.check(&owner()));
        assert!(group.check(&admin()));
        assert!(group.check(&regular_member()));
        assert!(!group.check(&left()));
    }
}
