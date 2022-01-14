use crate::domains::chat::{controller, repositories, service};
use sqlx::PgPool;
use std::sync::Arc;

pub struct ChatDomain {
    pub controller: Box<dyn controller::ChatController>,
}

pub async fn new_chat_domain(pool: Arc<PgPool>) -> ChatDomain {
    let repo = repositories::new_chat_db_repository(pool);
    let service = service::new_chat_service(repo);
    let controller = controller::new_chat_controller(service);
    ChatDomain { controller }
}
