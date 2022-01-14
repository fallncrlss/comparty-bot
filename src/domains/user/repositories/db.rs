use crate::domains::user::model;
use async_trait::async_trait;
use std::sync::Arc;
use sqlx;
use anyhow::Context;
use crate::lib::errors::DBError;

#[async_trait]
pub trait UserDBRepository: Send + Sync {
    async fn create(&self, body: &model::UserRequest) -> Result<bool, DBError>;

    async fn fetch_rating(
        &self,
        body: model::UserRatingRequest,
    ) -> Result<sqlx::types::BigDecimal, DBError>;

    async fn fetch_top_by_rating(
        &self,
        body: model::TopUsersRequest,
    ) -> Result<Vec<model::TopUsersResponse>, DBError>;

    async fn create_rating_record(
        &self,
        body: &model::RatingRequest,
    ) -> Result<sqlx::types::Uuid, DBError>;

    async fn delete_rating_record(&self, record_id: sqlx::types::Uuid) -> Result<(), DBError>;
}

struct PgUserDBRepositoryImpl {
    pool: Arc<sqlx::PgPool>,
}

#[async_trait]
impl UserDBRepository for PgUserDBRepositoryImpl {
    async fn create(&self, body: &model::UserRequest) -> Result<bool, DBError> {
        sqlx::query_file!(
            "src/domains/user/repositories/queries/user/create.sql",
            body.telegram_id,
            body.username,
            body.first_name,
            body.last_name,
        )
            .execute(&*self.pool)
            .await
            .map(|r| r.rows_affected().gt(&0))
            .map_err(anyhow::Error::new)
            .context("Failed to create user in Postgres")
            .map_err(DBError::Execute)
    }

    async fn fetch_rating(
        &self,
        body: model::UserRatingRequest,
    ) -> Result<sqlx::types::BigDecimal, DBError> {
        sqlx::query_file_scalar!(
            "src/domains/user/repositories/queries/rating/fetch_user_rating.sql",
            body.user_tg_id,
            body.chat_id,
        )
            .fetch_one(&*self.pool)
            .await
            .map_err(anyhow::Error::new)
            .context("Failed to fetch user rating in Postgres")
            .map_err(DBError::Execute)
    }

    async fn fetch_top_by_rating(
        &self,
        body: model::TopUsersRequest,
    ) -> Result<Vec<model::TopUsersResponse>, DBError> {
        sqlx::query_file_as!(
            model::TopUsersResponse,
            "src/domains/user/repositories/queries/user/fetch_top_users_by_rating.sql",
            body.chat_id,
            body.limit,
        )
            .fetch_all(&*self.pool)
            .await
            .map_err(anyhow::Error::new)
            .context("Failed to fetch top users in Postgres")
            .map_err(DBError::Execute)
    }

    async fn create_rating_record(
        &self,
        body: &model::RatingRequest,
    ) -> Result<sqlx::types::Uuid, DBError> {
        sqlx::query_file_scalar!(
            "src/domains/user/repositories/queries/rating/create.sql",
            body.user_tg_id,
            body.by_user_tg_id,
            body.chat_id,
            body.amount,
            body.comment
        )
            .fetch_one(&*self.pool)
            .await
            .map_err(anyhow::Error::new)
            .context("Failed to create rating in Postgres")
            .map_err(DBError::Execute)
    }

    async fn delete_rating_record(&self, record_id: sqlx::types::Uuid) -> Result<(), DBError> {
        sqlx::query_file!(
            "src/domains/user/repositories/queries/rating/delete_record.sql",
            record_id,
        )
            .execute(&*self.pool)
            .await
            .map(|_| ())
            .map_err(anyhow::Error::new)
            .context("Failed to delete rating record in Postgres")
            .map_err(DBError::Execute)
    }
}

pub fn new_user_db_repository(pool: Arc<sqlx::PgPool>) -> Box<dyn UserDBRepository> {
    Box::new(PgUserDBRepositoryImpl { pool })
}
