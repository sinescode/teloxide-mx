//! Commonly used items.

pub use crate::{
    error_handlers::{LoggingErrorHandler, OnError},
    error_types::TelegramError,
};

pub use crate::respond;

pub use crate::dispatching::{
    dialogue::{Dialogue, DialogueData},
    Dispatcher, HandlerExt as _, MessageFilterExt as _, Router, UpdateFilterExt as _,
};

pub use crate::dispatching::middleware::{Middleware, MiddlewareContext};

pub use crate::{
    sugar::message::MessageExt,
    testing::{MockBot, UpdateBuilder},
    utils::{
        callback_answer::CallbackAnswer,
        callback_data::{CallbackData, CallbackDataExt},
        chat_action::ChatActionSender,
        command_start::CommandStart,
        deep_linking,
        filters::FilterBuilder,
        flags::{FlagKey, Flags},
        i18n::{I18nContext, I18nLoader, Translation},
        keyboard::{InlineKeyboardBuilder, ReplyKeyboardBuilder},
        magic_filter::{ComposedFilter, FilterExt, F},
        media_group::MediaGroupBuilder,
    },
};

#[cfg(feature = "macros")]
pub use crate::macros::CallbackData;

#[cfg(feature = "ctrlc_handler")]
pub use crate::repls::CommandReplExt as _;

pub use teloxide_max_core::{
    requests::ResponseResult,
    types::{
        CallbackQuery, ChatMemberUpdated, ChosenInlineResult, InlineQuery, Message, Poll,
        PollAnswer, PreCheckoutQuery, ShippingQuery, Update,
    },
};

#[doc(no_inline)]
pub use teloxide_max_core::prelude::*;

pub use dptree::{self, prelude::*};
