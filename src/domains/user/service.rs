use crate::{lib, domains::user::{model, repositories::{UserDBRepository, UserCacheRepository}}};
use async_trait::async_trait;
use sqlx;

#[async_trait]
pub trait UserService: Send + Sync {
    async fn get_rating(&self, body: model::UserRatingRequest) -> Result<sqlx::types::BigDecimal, lib::errors::UserError>;
    async fn fetch_top_by_rating(&self, body: model::TopUsersRequest) -> Result<Vec<model::TopUsersResponse>, lib::errors::UserError>;
    async fn create_if_not_exists(&self, body: model::UserRequest) -> Result<bool, lib::errors::UserError>;
    async fn create_rating_record(&self, body: model::RatingRequest) -> Result<sqlx::types::Uuid, lib::errors::UserError>;
    async fn delete_rating_record(&self, record_id: &str) -> Result<(), lib::errors::UserError>;
}

struct UserServiceImpl {
    db_repo: Box<dyn UserDBRepository>,
    cache_repo: Box<dyn UserCacheRepository>,
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn get_rating(&self, body: model::UserRatingRequest) -> Result<sqlx::types::BigDecimal, lib::errors::UserError> {
        self.db_repo
            .fetch_rating(body)
            .await
            .map_err(|err| err.into())
            .map_err(lib::errors::UserError::GetRating)
    }

    async fn fetch_top_by_rating(&self, body: model::TopUsersRequest) -> Result<Vec<model::TopUsersResponse>, lib::errors::UserError> {
        self.db_repo
            .fetch_top_by_rating(body)
            .await
            .map_err(|err| err.into())
            .map_err(lib::errors::UserError::FetchRatingTop)
    }

    async fn create_if_not_exists(&self, body: model::UserRequest) -> Result<bool, lib::errors::UserError> {
        self.db_repo
            .create(&body)
            .await
            .map(|changed| {
                if changed {
                    log::info!(
                        "Successfully inserted user (id: {}, username: {} full name: {} {})",
                        body.telegram_id, body.username.unwrap_or_default(),
                        body.first_name, body.last_name.unwrap_or_default()
                    );
                }
                changed
            })
            .map_err(|err| err.into())
            .map_err(lib::errors::UserError::Insert)
    }

    async fn create_rating_record(&self, body: model::RatingRequest) -> Result<sqlx::types::Uuid, lib::errors::UserError> {
        let request = model::UserRatingActionRequest{ user_id: body.user_tg_id, by_user_id: body.by_user_tg_id, chat_id: body.chat_id };
        let expire_time = self.cache_repo
            .get_rating_action_expired_time(request)
            .await
            .map_err(|err| err.into())
            .map_err(lib::errors::UserError::InsertRating)?;

        if expire_time > 0 {
            return Err(lib::errors::UserError::RepeatingRequestDuringCooldown(
                format!("Вы слишком часто инициируете изменение рейтинга. Подождите {}s", expire_time)
            ));
        }

        let record_id = self.db_repo
            .create_rating_record(&body)
            .await
            .map(|r| {
                log::info!(
                    "Successfully inserted new rating record (user_id: {}, by_user_id: {}, chat_id: {}, comment: {}, amount: {:.2})",
                    body.user_tg_id, body.by_user_tg_id, body.chat_id, body.comment.clone().unwrap_or_default(), body.amount
                );
                r})
            .map_err(|err| err.into())
            .map_err(lib::errors::UserError::InsertRating)?;

        self.cache_repo
            .save_rating_action(request)
            .await
            .map(|_| {
                log::info!(
                    "REDIS: Successfully inserted or updated rating cooldown (user_id: {}, by_user_id: {}, chat_id: {})",
                    body.user_tg_id, body.by_user_tg_id, body.chat_id,
                );
            })
            .map_err(|err| { log::error!("{:?}", err); });

        Ok(record_id)
    }

    async fn delete_rating_record(&self, record_id: &str) -> Result<(), lib::errors::UserError> {
        let record_uuid = sqlx::types::Uuid::parse_str(record_id)
            .map_err(|err| err.into())
            .map_err(lib::errors::UserError::Validation)?;
        self
            .db_repo
            .delete_rating_record(record_uuid)
            .await
            .map(|_| { log::info!("Successfully deleted rating record (id: {})", record_id); })
            .map_err(|err| err.into())
            .map_err(lib::errors::UserError::DeleteRating)
    }
}

pub fn new_user_service(db_repo: Box<dyn UserDBRepository>, cache_repo: Box<dyn UserCacheRepository>) -> Box<dyn UserService> {
    Box::new(UserServiceImpl { db_repo, cache_repo })
}
