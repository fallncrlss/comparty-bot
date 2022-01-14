pub use crate::{
    core::persistence,
    domains::{admin_commands, chat, message, user},
};
use std::sync::Arc;

pub struct DomainHolder {
    pub admin_commands: admin_commands::AdminCommandsDomain,
    pub user: user::UserDomain,
    pub chat: chat::ChatDomain,
    pub message: message::MessageDomain,
}

pub async fn new_domain_holder() -> DomainHolder {
    let db_pool = Arc::new(persistence::new_pg_pool().await);
    let redis_client = Arc::new(persistence::new_redis_client().await);
    DomainHolder {
        admin_commands: admin_commands::new_admin_commands_domain().await,
        message: message::new_message_domain().await,
        user: user::new_user_domain(db_pool.clone(), redis_client.clone()).await,
        chat: chat::new_chat_domain(db_pool.clone()).await,
    }
}
