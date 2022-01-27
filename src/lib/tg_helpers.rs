use anyhow::Context;
use itertools::Itertools;
use teloxide::types::{Chat, User};
use crate::{lib::types::MessageContext, Request, Requester};

pub async fn reply_to(cx: &MessageContext, msg_text: String) -> Result<(), anyhow::Error> {
    cx
        .reply_to(msg_text)
        .send()
        .await
        .map(|_| ())
        .map_err(anyhow::Error::new)
        .context("Telegram API error")
}

pub async fn send_message(cx: &MessageContext, msg_text: String) -> Result<(), anyhow::Error> {
    cx
        .answer(msg_text)
        .send()
        .await
        .map(|_| ())
        .map_err(anyhow::Error::new)
        .context("Telegram API error")
}

pub fn get_user_as_chat(sender_chat: &Chat) -> User {
    teloxide::types::User {
        id: sender_chat.id,
        is_bot: false,
        first_name: sender_chat.title().unwrap().to_string(),
        last_name: Option::from(sender_chat.last_name().unwrap_or("").to_string()),
        username: Option::from(sender_chat.username().unwrap_or("").to_string()),
        language_code: None,
    }
}

pub fn get_user_to_interact(user: User, sender_chat: Option<&Chat>) -> User {
    sender_chat.map_or(user, get_user_as_chat)
}

pub async fn get_chat_administrator_mentions(cx: &MessageContext) -> Result<Vec<String>, anyhow::Error> {
    let admins = cx.requester
        .get_chat_administrators(cx.update.chat_id())
        .await
        .map_err(anyhow::Error::new)?;

    Ok(admins
        .iter()
        .filter(|admin| !admin.user.is_bot)
        .map(|admin| format!("<a href=\"tg://user?id={}\">&#8288;</a>", admin.user.id))
        .collect_vec())
}