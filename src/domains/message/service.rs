use anyhow::Context;
use crate::{lib, domains::message::model};
use async_trait::async_trait;

#[async_trait]
pub trait MessageService: Send + Sync {
    async fn get_cas_status(&self, user_id: i64) -> Result<model::CASResponse, lib::errors::MessageError>;
}

struct MessageServiceImpl;

#[async_trait]
impl MessageService for MessageServiceImpl {
    async fn get_cas_status(&self, user_id: i64) -> Result<model::CASResponse, lib::errors::MessageError> {
        let result = reqwest::get(format!("https://api.cas.chat/check?user_id={}", user_id))
            .await
            .context(format!("Cannot send request to check CAS for user (id: {})", user_id))
            .map_err(lib::errors::MessageError::GetCASStatus)?; // TODO: web repo

        let cas_response = result
            .json::<model::CASResponse>()
            .await
            .context("Cannot serialize CAS response")
            .map_err(lib::errors::MessageError::GetCASStatus)?;
        Ok(cas_response)
    }
}

pub fn new_message_service() -> Box<dyn MessageService> {
    Box::new(MessageServiceImpl {})
}
