use crate::{lib, Requester};
use async_trait::async_trait;
use itertools::Itertools;
use std::str::FromStr;

#[async_trait]
pub trait AdminCommandsService: Send + Sync {
    async fn get_chat_administrator_mentions(&self, cx: &lib::types::MessageContext) -> Result<Vec<String>, lib::errors::AdminCommandsError>;
    async fn get_restrict_time(&self, msg_text: &str) -> Result<lib::enums::TimeUnits, lib::errors::AdminCommandsError>;
}

struct AdminCommandsServiceImpl;

#[async_trait]
impl AdminCommandsService for AdminCommandsServiceImpl {
    async fn get_chat_administrator_mentions(
        &self,
        cx: &lib::types::MessageContext,
    ) -> Result<Vec<String>, lib::errors::AdminCommandsError> {
        let admins = cx.requester
            .get_chat_administrators(cx.update.chat_id())
            .await
            .map_err(|err| err.into())
            .map_err(lib::errors::AdminCommandsError::GetAdminMentions)?;

        Ok(admins
            .iter()
            .filter(|admin| !admin.user.is_bot)
            .map(|admin| format!("<a href=\"tg://user?id={}\">&#8288;</a>", admin.user.id))
            .collect_vec())
    }

    async fn get_restrict_time(&self, time: &str) -> Result<lib::enums::TimeUnits, lib::errors::AdminCommandsError> {
        lib::enums::TimeUnits::from_str(time)
            .map_err(|err| lib::errors::AdminCommandsError::GetRestrictMentions(err))
    }
}

pub fn new_admin_commands_service() -> Box<dyn AdminCommandsService> {
    Box::new(AdminCommandsServiceImpl {})
}
