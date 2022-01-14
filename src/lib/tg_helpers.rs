use anyhow::Context;
use teloxide::prelude::Request;
use crate::lib::types::MessageContext;

pub async fn send_message(cx: &MessageContext, msg_text: String) -> Result<(), anyhow::Error> {
    cx
        .reply_to(msg_text)
        .send()
        .await
        .map(|_| ())
        .map_err(anyhow::Error::new)
        .context("Telegram API error")
}