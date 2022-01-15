pub use crate::domains::{admin_commands, chat, message, user};
use std::sync::Arc;

pub struct DomainHolder {
    pub admin_commands: admin_commands::AdminCommandsDomain,
    pub user: user::UserDomain,
    pub chat: chat::ChatDomain,
    pub message: message::MessageDomain,
}

pub async fn new_domain_holder(db_pool: Arc<sqlx::PgPool>, cache_client: Arc<redis::Client>) -> DomainHolder {
    DomainHolder {
        admin_commands: admin_commands::new_admin_commands_domain().await,
        message: message::new_message_domain().await,
        user: user::new_user_domain(db_pool.clone(), cache_client.clone()).await,
        chat: chat::new_chat_domain(db_pool.clone()).await,
    }
}
