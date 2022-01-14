mod callback;
mod message;
use crate::{injected, lib};

pub async fn message_handler(
    cx: &lib::types::MessageContext,
    domain_holder: injected::DomainHolder,
) -> Result<(), anyhow::Error> {
    if cx.update.chat.is_group() || cx.update.chat.is_supergroup() {
        if cx.update.text().is_some() {
            message::user_init_handler(cx, &domain_holder).await?;
            message::chat_init_handler(cx, &domain_holder).await?;
            message::admin_commands_handler(cx, &domain_holder).await?;
            message::user_commands_handler(cx, &domain_holder).await?;
            message::link_free_text_handler(cx, &domain_holder).await?;
            message::rating_trigger_handler(cx, &domain_holder).await?;
        }

        if cx.update.new_chat_members().is_some() {
            message::new_chat_member_handler(cx, &domain_holder).await?;
        }

        if cx.update.migrate_from_chat_id().is_some() && cx.update.migrate_to_chat_id().is_some() {
            message::chat_migration_handler(cx, &domain_holder).await?;
        }
    }
    Ok(())
}

pub async fn callback_handler(
    cx: &lib::types::CallbackContext,
    domain_holder: injected::DomainHolder,
) -> Result<(), anyhow::Error> {
    callback::cancel_rating_handler(cx, &domain_holder).await?;
    Ok(())
}
