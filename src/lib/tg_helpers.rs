use anyhow::Context;
use crate::{lib::types::MessageContext, Request};

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