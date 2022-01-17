use crate::{injected, lib};
use std::str::FromStr;
use anyhow;

pub async fn user_init_handler(
    cx: &lib::types::MessageContext,
    domain_holder: &injected::DomainHolder,
) -> Result<(), lib::errors::UserError> {
    let user = cx.update.from().unwrap();
    let chat_id = cx.update.chat_id();
    let is_admin = lib::helpers::is_admin(cx)
        .await
        .map_err(lib::errors::UserError::Insert)?;
    domain_holder
        .user
        .controller
        .create_if_not_exists(user, chat_id, is_admin)
        .await
}

pub async fn chat_init_handler(
    cx: &lib::types::MessageContext,
    domain_holder: &injected::DomainHolder,
) -> Result<(), lib::errors::ChatError> {
    domain_holder.chat.controller.create_if_not_exists(cx).await
}

pub async fn link_free_text_handler(
    cx: &lib::types::MessageContext,
    domain_holder: &injected::DomainHolder
) -> Result<(), lib::errors::MessageControllerError> {
    let msg_text = cx.update.text().unwrap();
    domain_holder
        .message
        .controller
        .check_link_in_message(cx, msg_text)
        .await
}

pub async fn rating_trigger_handler(
    cx: &lib::types::MessageContext,
    domain_holder: &injected::DomainHolder,
) -> Result<(), anyhow::Error> {
    let msg_text = cx.update.text().unwrap();
    let chat_id = cx.update.chat_id();
    let is_admin = lib::helpers::is_admin(&cx).await?;

    let chat_settings = domain_holder
        .chat
        .controller
        .get_chat_settings(cx)
        .await
        .map_err(anyhow::Error::new)?;

    if !chat_settings.is_rating_count || (chat_settings.commands_for_admin_only && !is_admin) {
        return Ok(());
    }

    if let Ok(rating_trigger) = lib::enums::RatingTriggers::from_str(msg_text) {
        if let Some(reply_msg) = cx.update.reply_to_message() {
            if let Some(reply_user) = reply_msg.from() {
                if !reply_user.is_bot {
                    domain_holder
                        .user
                        .controller
                        .create_if_not_exists(reply_user, chat_id, is_admin)
                        .await?;

                    domain_holder
                        .user
                        .controller
                        .create_rating_record(cx, rating_trigger)
                        .await?;
                }
            }
        }
    }
    Ok(())
}
