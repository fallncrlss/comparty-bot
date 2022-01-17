use crate::domains::chat::model;
use crate::lib::errors::DBError;
use async_trait::async_trait;
use anyhow::Context;
use std::sync::Arc;

#[async_trait]
pub trait ChatDBRepository: Send + Sync {
    async fn create(&self, body: &model::Chat) -> Result<bool, DBError>;
    async fn get_chat_settings(&self, chat_id: i64) -> Result<model::ChatSettings, DBError>;
    async fn create_chat_settings(&self, body: &model::ChatSettings) -> Result<bool, DBError>;
    async fn change_chat_settings(&self, body: &model::ChatSettings) -> Result<(), DBError>;
    async fn migrate_chat(&self, from: i64, to: i64) -> Result<(), DBError>;
}

struct PgChatDBRepositoryImpl {
    pool: Arc<sqlx::PgPool>,
}

#[async_trait]
impl ChatDBRepository for PgChatDBRepositoryImpl {
    async fn create(&self, body: &model::Chat) -> Result<bool, DBError> {
        sqlx::query_file!(
            "src/domains/chat/repositories/queries/chat/create.sql",
            body.chat_id,
            body.title
        )
            .execute(&*self.pool)
            .await
            .map(|r| r.rows_affected().gt(&0))
            .map_err(anyhow::Error::new)
            .context("Failed to create chat in Postgres")
            .map_err(DBError::Execute)
    }

    async fn get_chat_settings(&self, chat_id: i64) -> Result<model::ChatSettings, DBError> {
        sqlx::query_file_as!(
            model::ChatSettings,
            "src/domains/chat/repositories/queries/chat_settings/fetch.sql",
            chat_id
        )
            .fetch_one(&*self.pool)
            .await
            .map_err(anyhow::Error::new)
            .context("Failed to fetch chat settings in Postgres")
            .map_err(DBError::Execute)
    }

    async fn create_chat_settings(&self, body: &model::ChatSettings) -> Result<bool, DBError> {
        sqlx::query_file!(
            "src/domains/chat/repositories/queries/chat_settings/create.sql",
            body.chat_id,
            body.is_rating_count
        )
            .execute(&*self.pool)
            .await
            .map(|r| r.rows_affected().gt(&0))
            .map_err(anyhow::Error::new)
            .context("Failed to create chat settings in Postgres")
            .map_err(DBError::Execute)
    }

    async fn change_chat_settings(&self, body: &model::ChatSettings) -> Result<(), DBError> {
        sqlx::query_file!(
            "src/domains/chat/repositories/queries/chat_settings/change.sql",
            body.chat_id,
            body.is_rating_count,
            body.commands_for_admin_only,
        )
            .execute(&*self.pool)
            .await
            .map(|_| ())
            .map_err(anyhow::Error::new)
            .context("Failed to change chat settings in Postgres")
            .map_err(DBError::Execute)
    }

    async fn migrate_chat(&self, from: i64, to: i64) -> Result<(), DBError> {
        sqlx::query_file!(
            "src/domains/chat/repositories/queries/chat/migrate_chat_id.sql",
            from,
            to
        )
            .execute(&*self.pool)
            .await
            .map(|_| ())
            .map_err(anyhow::Error::new)
            .context("Failed to migrate chat ids in Postgres")
            .map_err(DBError::Execute)
    }
}

pub fn new_chat_db_repository(pool: Arc<sqlx::PgPool>) -> Box<dyn ChatDBRepository> {
    Box::new(PgChatDBRepositoryImpl { pool })
}
