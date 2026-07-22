//! Telegram deep-link and t.me URL builders.
//!
//! Complements [`crate::utils::deep_linking`] with general-purpose `tg://` and
//! `https://t.me/...` helpers matching aiogram's `utils.link` module.
//!
//! # Example
//!
//! ```rust
//! use teloxide_max::utils::link::{
//!     create_channel_bot_link, create_telegram_link, create_tg_link, ChannelBotPermissions,
//! };
//!
//! assert_eq!(create_tg_link("resolve", &[("domain", "durov")]), "tg://resolve?domain=durov");
//! assert_eq!(create_telegram_link(&["durov"], &[]), "https://t.me/durov");
//!
//! let admin_link = create_channel_bot_link(
//!     "my_bot",
//!     Some("payload"),
//!     ChannelBotPermissions { manage_chat: true, ..Default::default() },
//! );
//! assert!(admin_link.contains("startgroup=payload"));
//! assert!(admin_link.contains("admin=manage_chat"));
//! ```
//!
//! # Migration from aiogram
//!
//! | aiogram | teloxide_max |
//! |---------|--------------|
//! | `create_tg_link(link, **kwargs)` | `create_tg_link(link, query)` |
//! | `create_telegram_link(*path, **kwargs)` | `create_telegram_link(path, query)` |
//! | `create_channel_bot_link(username, ...)` | `create_channel_bot_link(username, parameter, permissions)` |

use std::fmt::Write as _;

/// Permissions requested when adding a bot to a group/channel via a deep link.
///
/// Maps to the `admin` query parameter of
/// `t.me/<bot>?startgroup=...&admin=...`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ChannelBotPermissions {
    pub change_info: bool,
    pub post_messages: bool,
    pub edit_messages: bool,
    pub delete_messages: bool,
    pub restrict_members: bool,
    pub invite_users: bool,
    pub pin_messages: bool,
    pub promote_members: bool,
    pub manage_video_chats: bool,
    pub anonymous: bool,
    pub manage_chat: bool,
}

impl ChannelBotPermissions {
    /// All permissions disabled.
    pub const fn none() -> Self {
        Self {
            change_info: false,
            post_messages: false,
            edit_messages: false,
            delete_messages: false,
            restrict_members: false,
            invite_users: false,
            pin_messages: false,
            promote_members: false,
            manage_video_chats: false,
            anonymous: false,
            manage_chat: false,
        }
    }

    /// Collect enabled permission names for the `admin` query param.
    pub fn as_admin_param(&self) -> Option<String> {
        let mut perms = Vec::new();
        if self.change_info {
            perms.push("change_info");
        }
        if self.post_messages {
            perms.push("post_messages");
        }
        if self.edit_messages {
            perms.push("edit_messages");
        }
        if self.delete_messages {
            perms.push("delete_messages");
        }
        if self.restrict_members {
            perms.push("restrict_members");
        }
        if self.invite_users {
            perms.push("invite_users");
        }
        if self.pin_messages {
            perms.push("pin_messages");
        }
        if self.promote_members {
            perms.push("promote_members");
        }
        if self.manage_video_chats {
            perms.push("manage_video_chats");
        }
        if self.anonymous {
            perms.push("anonymous");
        }
        if self.manage_chat {
            perms.push("manage_chat");
        }
        if perms.is_empty() {
            None
        } else {
            Some(perms.join("+"))
        }
    }
}

/// Create a `tg://` deep link.
///
/// # Arguments
///
/// * `link` — path after `tg://` (e.g. `"resolve"`, `"user"`).
/// * `query` — optional query key/value pairs.
pub fn create_tg_link(link: &str, query: &[(&str, &str)]) -> String {
    format_url(&format!("tg://{link}"), &[], query)
}

/// Create an `https://t.me/...` link.
///
/// # Arguments
///
/// * `path` — path segments after `https://t.me/`.
/// * `query` — optional query key/value pairs.
pub fn create_telegram_link(path: &[&str], query: &[(&str, &str)]) -> String {
    format_url("https://t.me", path, query)
}

/// Create a link that invites a bot into a group/channel with admin rights.
///
/// Corresponds to aiogram's `create_channel_bot_link`.
///
/// # Arguments
///
/// * `username` — bot username **without** `@`.
/// * `parameter` — optional `startgroup` payload.
/// * `permissions` — admin rights to request.
pub fn create_channel_bot_link(
    username: &str,
    parameter: Option<&str>,
    permissions: ChannelBotPermissions,
) -> String {
    let mut query: Vec<(&str, String)> = Vec::new();
    if let Some(p) = parameter {
        query.push(("startgroup", p.to_string()));
    }
    if let Some(admin) = permissions.as_admin_param() {
        query.push(("admin", admin));
    }

    let query_refs: Vec<(&str, &str)> = query.iter().map(|(k, v)| (*k, v.as_str())).collect();
    create_telegram_link(&[username], &query_refs)
}

/// Create a public username profile link: `https://t.me/<username>`.
pub fn create_username_link(username: &str) -> String {
    let name = username.trim_start_matches('@');
    create_telegram_link(&[name], &[])
}

/// Create a `tg://user?id=<user_id>` mention link.
pub fn create_user_id_link(user_id: u64) -> String {
    create_tg_link("user", &[("id", &user_id.to_string())])
}

fn format_url(base: &str, path: &[&str], query: &[(&str, &str)]) -> String {
    let mut url = base.to_string();
    for segment in path {
        if !url.ends_with('/') {
            url.push('/');
        }
        // Strip leading/trailing slashes from segments.
        let seg = segment.trim_matches('/');
        url.push_str(seg);
    }
    if !query.is_empty() {
        url.push('?');
        for (i, (k, v)) in query.iter().enumerate() {
            if i > 0 {
                url.push('&');
            }
            // Values may already contain `+` (admin permissions); don't re-encode.
            let _ = write!(url, "{k}={v}");
        }
    }
    url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tg_link() {
        assert_eq!(create_tg_link("resolve", &[("domain", "durov")]), "tg://resolve?domain=durov");
    }

    #[test]
    fn telegram_link() {
        assert_eq!(create_telegram_link(&["durov"], &[]), "https://t.me/durov");
        assert_eq!(
            create_telegram_link(&["share", "url"], &[("url", "https://example.com")]),
            "https://t.me/share/url?url=https://example.com"
        );
    }

    #[test]
    fn channel_bot_link_with_permissions() {
        let link = create_channel_bot_link(
            "my_bot",
            Some("payload"),
            ChannelBotPermissions {
                manage_chat: true,
                delete_messages: true,
                ..ChannelBotPermissions::none()
            },
        );
        assert_eq!(
            link,
            "https://t.me/my_bot?startgroup=payload&admin=delete_messages+manage_chat"
        );
    }

    #[test]
    fn channel_bot_link_no_params() {
        let link = create_channel_bot_link("my_bot", None, ChannelBotPermissions::none());
        assert_eq!(link, "https://t.me/my_bot");
    }

    #[test]
    fn username_and_user_id() {
        assert_eq!(create_username_link("@durov"), "https://t.me/durov");
        assert_eq!(create_user_id_link(42), "tg://user?id=42");
    }
}
