use crate::{lib, domains::chat::{model, repositories::ChatDBRepository}};
use async_trait::async_trait;

#[async_trait]
pub trait ChatService: Send + Sync {
    async fn create(&self, body: model::Chat) -> Result<(), lib::errors::ChatError>;
    async fn create_chat_settings(&self, body: &model::ChatSettings) -> Result<(), lib::errors::ChatError>;
    async fn get_chat_settings(&self, chat_id: i64) -> Result<model::ChatSettings, lib::errors::ChatError>;
    async fn change_chat_settings(&self, body: &model::ChatSettings) -> Result<(), lib::errors::ChatError>;
    async fn migrate_chat(&self, from: i64, to: i64) -> Result<(), lib::errors::ChatError>;
}

struct ChatServiceImpl {
    repo: Box<dyn ChatDBRepository>,
}

#[async_trait]
impl ChatService for ChatServiceImpl {
    async fn create(&self, body: model::Chat) -> Result<(), lib::errors::ChatError> {
        self.repo
            .create(&body)
            .await
            .map(|changed| {
                if changed {
                    log::info!(
                        "Successfully inserted or updated chat (id: {}, title: {})",
                        body.chat_id,
                        body.title
                    );
                }
            })
            .map_err(|err| err.into())
            .map_err(lib::errors::ChatError::Insert)
    }

    async fn create_chat_settings(&self, body: &model::ChatSettings) -> Result<(), lib::errors::ChatError> {
        self.repo
            .create_chat_settings(body)
            .await
            .map(|changed| {
                if changed {
                    log::info!("Successfully inserted chat settings (id: {})", body.chat_id);
                }
            })
            .map_err(|err| err.into())
            .map_err(lib::errors::ChatError::InsertSettings)
    }

    async fn get_chat_settings(&self, chat_id: i64) -> Result<model::ChatSettings, lib::errors::ChatError> {
        self.repo
            .get_chat_settings(chat_id)
            .await
            .map_err(|err| err.into())
            .map_err(lib::errors::ChatError::GetSettings)
    }

    async fn change_chat_settings(&self, body: &model::ChatSettings) -> Result<(), lib::errors::ChatError> {
        self.repo
            .change_chat_settings(body)
            .await
            .map_err(|err| err.into())
            .map_err(lib::errors::ChatError::ChangeSettings)
            .map(|_| {
                log::info!(
                    "Successfully changed chat settings (id: {}, rating_count: {})",
                    body.chat_id,
                    body.is_rating_count
                )
            })
    }

    async fn migrate_chat(&self, from: i64, to: i64) -> Result<(), lib::errors::ChatError> {
        self.repo
            .migrate_chat(from, to)
            .await
            .map(|_| {
                log::info!("Successfully migrated chat (from id: {}, to id: {})", from, to)
            })
            .map_err(|err| err.into())
            .map_err(lib::errors::ChatError::MigrateChat)
    }
}

pub fn new_chat_service(repo: Box<dyn ChatDBRepository>) -> Box<dyn ChatService> {
    Box::new(ChatServiceImpl { repo })
}
