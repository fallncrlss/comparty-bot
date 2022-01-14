use crate::domains::user::model::UserRatingActionRequest;
use crate::lib::config::RATING_COOLDOWN;
use redis::{Client, AsyncCommands};
use crate::lib::errors::CacheError;
use async_trait::async_trait;
use std::sync::Arc;
use anyhow::Context;

#[async_trait]
pub trait UserCacheRepository: Send + Sync {
    async fn save_rating_action(&self, body: UserRatingActionRequest) -> Result<(), CacheError>;
    async fn get_rating_action_expired_time(&self, body: UserRatingActionRequest) -> Result<i64, CacheError>;
}

struct RedisUserCacheRepositoryImpl {
    client: Arc<Client>,
}

#[async_trait]
impl UserCacheRepository for RedisUserCacheRepositoryImpl {
    async fn save_rating_action(&self, body: UserRatingActionRequest) -> Result<(), CacheError> {
        let mut conn = self.client
            .get_async_connection()
            .await
            .context("Failed to initiate async Redis connection")?;

        conn
            .set_ex(body.get_key(), true, RATING_COOLDOWN as usize)
            .await
            .context("Unable save document using Redis")
            .map_err(CacheError::Execute)
    }

    async fn get_rating_action_expired_time(&self, body: UserRatingActionRequest) -> Result<i64, CacheError> {
        let mut conn = self.client
            .get_async_connection()
            .await
            .context("Failed to initiate async Redis connection")?;

        conn
            .ttl(body.get_key())
            .await
            .context("Unable get document TTL using Redis")
            .map_err(CacheError::Execute)
    }
}

pub fn new_user_cache_repository(client: Arc<Client>) -> Box<dyn UserCacheRepository> {
    Box::new(RedisUserCacheRepositoryImpl { client })
}
