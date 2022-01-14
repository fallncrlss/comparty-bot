use crate::{injected, lib};

pub async fn chat_migration_handler(
    cx: &lib::types::MessageContext,
    domain_holder: &injected::DomainHolder,
) -> Result<(), lib::errors::ChatError> {
    domain_holder.chat.controller.migrate_chat(cx).await
}
