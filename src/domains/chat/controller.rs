use crate::lib::enums::RatingCountSwitch;
use crate::lib::types::MessageContext;
use crate::{domains::chat::{model, service::ChatService}, lib};
use async_trait::async_trait;
use crate::lib::tg_helpers::reply_to;

#[async_trait]
pub trait ChatController: Send + Sync {
    async fn create_if_not_exists(&self, cx: &lib::types::MessageContext) -> Result<(), lib::errors::ChatError>;
    async fn migrate_chat(&self, cx: &lib::types::MessageContext) -> Result<(), lib::errors::ChatError>;
    async fn get_chat_settings(&self, cx: &lib::types::MessageContext) -> Result<model::ChatSettings, lib::errors::ChatError>;
    async fn change_chat_settings(
        &self,
        cx: &lib::types::MessageContext,
        count_switch: lib::enums::RatingCountSwitch
    ) -> Result<(), lib::errors::AdminCommandsControllerError>;
}

struct ChatControllerImpl {
    service: Box<dyn ChatService>,
}

#[async_trait]
impl ChatController for ChatControllerImpl {
    async fn create_if_not_exists(&self, cx: &lib::types::MessageContext) -> Result<(), lib::errors::ChatError> {
        let chat = &cx.update.chat;
        self.service
            .create(model::Chat {
                chat_id: chat.id,
                title: chat.title().unwrap().to_string(),
            })
            .await?;
        self.service
            .create_chat_settings(&model::ChatSettings {
                chat_id: chat.id,
                is_rating_count: true,
            })
            .await
    }

    async fn migrate_chat(&self, cx: &lib::types::MessageContext) -> Result<(), lib::errors::ChatError> {
        let from_chat_id = cx.update.migrate_from_chat_id().unwrap();
        let to_chat_id = cx.update.migrate_to_chat_id().unwrap();
        self.service
            .migrate_chat(from_chat_id, to_chat_id)
            .await
    }

    async fn get_chat_settings(&self, cx: &MessageContext) -> Result<model::ChatSettings, lib::errors::ChatError> {
        self.service
            .get_chat_settings(cx.update.chat_id())
            .await
    }

    async fn change_chat_settings(
        &self,
        cx: &lib::types::MessageContext,
        count_switch: lib::enums::RatingCountSwitch,
    ) -> Result<(), lib::errors::AdminCommandsControllerError> {
        let is_rating_count = match count_switch {
            RatingCountSwitch::On => true,
            RatingCountSwitch::Off => false,
        };
        let result = self
            .service
            .change_chat_settings(&model::ChatSettings {
                chat_id: cx.update.chat_id(),
                is_rating_count,
            })
            .await;
        let msg_text = match result {
            Ok(_) => "Настройки чата успешно изменены",
            Err(_) => "Невозможно изменить настройки чата"
        }.to_string();

        reply_to(cx, msg_text)
            .await
            .map_err(lib::errors::AdminCommandsControllerError::ChangeSettings)
    }
}

pub fn new_chat_controller(service: Box<dyn ChatService>) -> Box<dyn ChatController> {
    Box::new(ChatControllerImpl { service })
}
