use crate::{injected, lib};

pub async fn new_chat_member_handler(
    cx: &lib::types::MessageContext,
    domain_holder: &injected::DomainHolder,
) -> Result<(), lib::errors::MessageControllerError> {
    let new_members = cx.update.new_chat_members().unwrap();
    for new_member in new_members {
        domain_holder
            .message
            .controller
            .check_new_member(cx, new_member)
            .await?;
    }
    Ok(())
}
