use crate::{domains::message::service::MessageService, lib, Requester};
use async_trait::async_trait;
use teloxide;

#[async_trait]
pub trait MessageController: Send + Sync {
    async fn check_link_in_message(
        &self,
        cx: &lib::types::MessageContext,
        text: &str,
    ) -> Result<(), lib::errors::MessageControllerError>;
    async fn check_new_member(
        &self,
        cx: &crate::lib::types::MessageContext,
        new_member: &teloxide::types::User,
    ) -> Result<(), lib::errors::MessageControllerError>;
}

struct MessageControllerImpl {
    service: Box<dyn MessageService>,
}

#[async_trait]
impl MessageController for MessageControllerImpl {
    async fn check_link_in_message(
        &self,
        cx: &lib::types::MessageContext,
        text: &str,
    ) -> Result<(), lib::errors::MessageControllerError> {
        let chat_id = cx.update.chat_id();
        let sender = cx.update.from().unwrap();

        if let Ok(link) = lib::helpers::check_link_in_text(text) {
            cx.requester
                .kick_chat_member(chat_id, sender.id)
                .await
                .map_err(|err| err.into())
                .map_err(lib::errors::MessageControllerError::CheckLinkInMessage)?;
            lib::tg_helpers::send_message(
                cx,
                format!(
                    "Пользователь {} был забанен за запрещённую ссылку в сообщении.",
                    teloxide::utils::html::user_mention_or_link(sender),
                )
            )
                .await
                .map_err(lib::errors::MessageControllerError::CheckLinkInMessage)?;
            log::info!("Found the prohibited link: {}!", link);
            log::info!("Ban user {} from chat {}.", sender.full_name(), chat_id);
        }
        Ok(())
    }

    async fn check_new_member(
        &self,
        cx: &lib::types::MessageContext,
        new_member: &teloxide::types::User,
    ) -> Result<(), lib::errors::MessageControllerError> {
        let chat = &cx.update.chat;
        log::info!("new member \"{}\" in chat \"{}\"", new_member.full_name(), chat.title().unwrap());

        let resp = self.service
            .get_cas_status(new_member.id)
            .await
            .map_err(|err| err.into())
            .map_err(lib::errors::MessageControllerError::CheckNewMember)?;

        if resp.ok {
            cx.requester
                .kick_chat_member(chat.id, new_member.id)
                .await
                .map(|_| {
                    log::info!("User {} (id: {}) is banned due to CAS.", new_member.full_name(), new_member.id);
                })
                .map_err(|err| err.into())
                .map_err(lib::errors::MessageControllerError::CheckNewMember)?;
        }


        if lib::helpers::check_is_full_name_clean(new_member.full_name()) {
            cx.requester
                .kick_chat_member(chat.id, new_member.id)
                .await
                .map(|_| {
                    log::info!("User {} (id: {}) is banned due to inappropriate full name.", new_member.full_name(), new_member.id);
                })
                .map_err(|err| err.into())
                .map_err(lib::errors::MessageControllerError::CheckLinkInMessage)?;
            lib::tg_helpers::send_message(
                cx,
                format!(
                    "Пользователь {} был забанен за запрещённое имя пользователя.",
                    teloxide::utils::html::user_mention_or_link(sender),
                )
            )
                .await
                .map_err(lib::errors::MessageControllerError::CheckLinkInMessage)?;
        }

        Ok(())
    }
}

pub fn new_message_controller(service: Box<dyn MessageService>) -> Box<dyn MessageController> {
    Box::new(MessageControllerImpl { service })
}
