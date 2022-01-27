use crate::lib;
use async_trait::async_trait;
use std::str::FromStr;

#[async_trait]
pub trait AdminCommandsService: Send + Sync {
    async fn get_restrict_time(&self, msg_text: &str) -> Result<lib::enums::TimeUnits, lib::errors::AdminCommandsError>;
}

struct AdminCommandsServiceImpl;

#[async_trait]
impl AdminCommandsService for AdminCommandsServiceImpl {
    async fn get_restrict_time(&self, time: &str) -> Result<lib::enums::TimeUnits, lib::errors::AdminCommandsError> {
        lib::enums::TimeUnits::from_str(time)
            .map_err(|err| lib::errors::AdminCommandsError::GetRestrictMentions(err))
    }
}

pub fn new_admin_commands_service() -> Box<dyn AdminCommandsService> {
    Box::new(AdminCommandsServiceImpl {})
}
