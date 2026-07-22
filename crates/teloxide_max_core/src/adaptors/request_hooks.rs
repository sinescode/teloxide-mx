use std::{
    fmt::Debug,
    future::{Future, IntoFuture},
    pin::Pin,
    sync::Arc,
    task::{self, Poll},
};

use futures::ready;
use url::Url;

use crate::{
    requests::{HasPayload, Output, Payload, Request, Requester},
    types::*,
};

/// Hook functions for outgoing API requests.
///
/// # Example
///
/// ```rust,no_run
/// use teloxide_max_core::{adaptors::request_hooks::RequestHooks, prelude::*};
///
/// let bot = Bot::from_env().with_hooks(RequestHooks {
///     before: |payload_name| {
///         log::info!("Sending request: {}", payload_name);
///     },
///     after: |payload_name, result| {
///         log::info!("Response from {}: {:?}", payload_name, result);
///     },
/// });
/// ```
#[derive(Clone, Debug)]
pub struct RequestHooks {
    /// Called before each request is sent.
    /// Receives the payload name (e.g. "SendMessage").
    pub before: fn(&str),

    /// Called after each request completes.
    /// Receives the payload name and whether it succeeded.
    pub after: fn(&str, bool),
}

impl RequestHooks {
    /// Creates hooks with both before and after callbacks.
    pub fn new(before: fn(&str), after: fn(&str, bool)) -> Self {
        Self { before, after }
    }

    /// Creates hooks that only log requests.
    pub fn log_all() -> Self {
        Self {
            before: |name| log::trace!("Sending `{name}` request"),
            after: |name, ok| {
                if ok {
                    log::trace!("Got OK from `{name}` request");
                } else {
                    log::trace!("Got error from `{name}` request");
                }
            },
        }
    }

    /// Creates hooks that only call the before function.
    pub fn before_only(before: fn(&str)) -> Self {
        Self { before, after: |_, _| {} }
    }

    /// Creates hooks that only call the after function.
    pub fn after_only(after: fn(&str, bool)) -> Self {
        Self { before: |_| {}, after }
    }
}

/// A bot adaptor that calls hooks before and after each API request.
///
/// This is useful for logging, metrics, or custom request processing.
///
/// # Example
///
/// ```rust,no_run
/// use teloxide_max_core::{
///     adaptors::request_hooks::{RequestHooks, RequestHooksAdaptor},
///     prelude::*,
/// };
///
/// let bot = RequestHooksAdaptor::new(Bot::from_env(), RequestHooks::log_all());
/// ```
#[derive(Clone, Debug)]
pub struct RequestHooksAdaptor<B> {
    inner: B,
    hooks: Arc<RequestHooks>,
}

impl<B> RequestHooksAdaptor<B> {
    /// Creates a new adaptor wrapping the given bot with the given hooks.
    pub fn new(inner: B, hooks: RequestHooks) -> Self {
        Self { inner, hooks: Arc::new(hooks) }
    }

    pub fn inner(&self) -> &B {
        &self.inner
    }

    pub fn into_inner(self) -> B {
        self.inner
    }
}

macro_rules! fty {
    ($T:ident) => {
        RequestHooksRequest<B::$T>
    };
}

macro_rules! fwd_inner {
    ($m:ident $this:ident ($($arg:ident : $T:ty),*)) => {
        RequestHooksRequest {
            inner: $this.inner().$m($($arg),*),
            hooks: $this.hooks.clone(),
        }
    };
}

impl<B> Requester for RequestHooksAdaptor<B>
where
    B: Requester,
{
    type Err = B::Err;

    requester_forward! {
        get_me,
        log_out,
        close,
        get_updates,
        set_webhook,
        delete_webhook,
        get_webhook_info,
        forward_message,
        forward_messages,
        copy_message,
        copy_messages,
        send_message,
        send_photo,
        send_audio,
        send_document,
        send_video,
        send_animation,
        send_voice,
        send_video_note,
        send_paid_media,
        send_media_group,
        send_location,
        edit_message_live_location,
        edit_message_live_location_inline,
        stop_message_live_location,
        stop_message_live_location_inline,
        edit_message_checklist,
        send_venue,
        send_contact,
        send_poll,
        send_checklist,
        send_dice,
        send_chat_action,
        set_message_reaction,
        get_user_profile_photos,
        set_user_emoji_status,
        get_file,
        kick_chat_member,
        ban_chat_member,
        unban_chat_member,
        restrict_chat_member,
        promote_chat_member,
        set_chat_administrator_custom_title,
        ban_chat_sender_chat,
        unban_chat_sender_chat,
        set_chat_permissions,
        export_chat_invite_link,
        create_chat_invite_link,
        edit_chat_invite_link,
        create_chat_subscription_invite_link,
        edit_chat_subscription_invite_link,
        revoke_chat_invite_link,
        set_chat_photo,
        delete_chat_photo,
        set_chat_title,
        set_chat_description,
        pin_chat_message,
        unpin_chat_message,
        unpin_all_chat_messages,
        leave_chat,
        get_chat,
        get_chat_administrators,
        get_chat_members_count,
        get_chat_member_count,
        get_chat_member,
        set_chat_sticker_set,
        delete_chat_sticker_set,
        get_forum_topic_icon_stickers,
        create_forum_topic,
        edit_forum_topic,
        close_forum_topic,
        reopen_forum_topic,
        delete_forum_topic,
        unpin_all_forum_topic_messages,
        edit_general_forum_topic,
        close_general_forum_topic,
        reopen_general_forum_topic,
        hide_general_forum_topic,
        unhide_general_forum_topic,
        unpin_all_general_forum_topic_messages,
        answer_callback_query,
        get_user_chat_boosts,
        set_my_commands,
        get_business_connection,
        get_my_commands,
        set_my_name,
        get_my_name,
        set_my_description,
        get_my_description,
        set_my_short_description,
        get_my_short_description,
        set_chat_menu_button,
        get_chat_menu_button,
        set_my_default_administrator_rights,
        get_my_default_administrator_rights,
        delete_my_commands,
        answer_inline_query,
        answer_web_app_query,
        save_prepared_inline_message,
        edit_message_text,
        edit_message_text_inline,
        edit_message_caption,
        edit_message_caption_inline,
        edit_message_media,
        edit_message_media_inline,
        edit_message_reply_markup,
        edit_message_reply_markup_inline,
        stop_poll,
        approve_suggested_post,
        decline_suggested_post,
        delete_message,
        delete_messages,
        send_sticker,
        get_sticker_set,
        get_custom_emoji_stickers,
        upload_sticker_file,
        create_new_sticker_set,
        add_sticker_to_set,
        set_sticker_position_in_set,
        delete_sticker_from_set,
        replace_sticker_in_set,
        set_sticker_set_thumbnail,
        set_custom_emoji_sticker_set_thumbnail,
        set_sticker_set_title,
        delete_sticker_set,
        set_sticker_emoji_list,
        set_sticker_keywords,
        set_sticker_mask_position,
        get_available_gifts,
        send_gift,
        send_gift_chat,
        gift_premium_subscription,
        verify_user,
        verify_chat,
        remove_user_verification,
        remove_chat_verification,
        read_business_message,
        delete_business_messages,
        set_business_account_name,
        set_business_account_username,
        set_business_account_bio,
        set_business_account_profile_photo,
        remove_business_account_profile_photo,
        set_business_account_gift_settings,
        get_business_account_star_balance,
        transfer_business_account_stars,
        get_business_account_gifts,
        convert_gift_to_stars,
        upgrade_gift,
        transfer_gift,
        post_story,
        edit_story,
        delete_story,
        send_invoice,
        create_invoice_link,
        answer_shipping_query,
        answer_pre_checkout_query,
        get_my_star_balance,
        get_star_transactions,
        refund_star_payment,
        edit_user_star_subscription,
        set_passport_data_errors,
        send_game,
        set_game_score,
        set_game_score_inline,
        get_game_high_scores,
        approve_chat_join_request,
        send_message_draft,
            get_user_gifts,
            get_chat_gifts,
            repost_story,
            set_my_profile_photo,
            remove_my_profile_photo,
            get_user_profile_audios,
            set_chat_member_tag,
            get_managed_bot_token,
            replace_managed_bot_token,
            save_prepared_keyboard_button,
            answer_chat_join_request_query,
            send_chat_join_request_web_app,
            delete_message_reaction,
            delete_all_message_reactions,
            answer_guest_query,
            send_live_photo,
            get_managed_bot_access_settings,
            set_managed_bot_access_settings,
            get_user_personal_chat_messages,
            send_rich_message,
            send_rich_message_draft,
            edit_ephemeral_message_text,
            edit_ephemeral_message_caption,
            edit_ephemeral_message_media,
            edit_ephemeral_message_reply_markup,
            delete_ephemeral_message,
            decline_chat_join_request
        => fwd_inner, fty
    }
}

#[must_use = "Requests are lazy and do nothing unless sent"]
#[derive(Clone)]
pub struct RequestHooksRequest<R> {
    inner: R,
    hooks: Arc<RequestHooks>,
}

impl<R> HasPayload for RequestHooksRequest<R>
where
    R: HasPayload,
{
    type Payload = R::Payload;

    fn payload_mut(&mut self) -> &mut Self::Payload {
        self.inner.payload_mut()
    }

    fn payload_ref(&self) -> &Self::Payload {
        self.inner.payload_ref()
    }
}

impl<R> Request for RequestHooksRequest<R>
where
    R: Request,
    R::Payload: Payload,
{
    type Err = R::Err;

    type Send = Send<R::Send>;

    type SendRef = Send<R::SendRef>;

    fn send(self) -> Self::Send {
        let name = R::Payload::NAME;
        (self.hooks.before)(name);

        Send { hooks: self.hooks.clone(), name, inner: self.inner.send() }
    }

    fn send_ref(&self) -> Self::SendRef {
        let name = R::Payload::NAME;
        (self.hooks.before)(name);

        Send { hooks: self.hooks.clone(), name, inner: self.inner.send_ref() }
    }
}

impl<R> IntoFuture for RequestHooksRequest<R>
where
    R: Request,
    R::Payload: Payload,
{
    type Output = Result<Output<Self>, <Self as Request>::Err>;
    type IntoFuture = <Self as Request>::Send;

    fn into_future(self) -> Self::IntoFuture {
        self.send()
    }
}

#[pin_project::pin_project]
pub struct Send<F>
where
    F: Future,
{
    hooks: Arc<RequestHooks>,
    name: &'static str,
    #[pin]
    inner: F,
}

impl<F, T, E> Future for Send<F>
where
    F: Future<Output = Result<T, E>>,
{
    type Output = Result<T, E>;

    fn poll(self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        let ret = ready!(this.inner.poll(cx));
        (this.hooks.after)(this.name, ret.is_ok());
        Poll::Ready(ret)
    }
}
