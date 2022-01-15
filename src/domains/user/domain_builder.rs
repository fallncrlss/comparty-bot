use crate::domains::user::{controller, repositories, service};
use std::sync::Arc;
use redis;

pub struct UserDomain {
    pub controller: Box<dyn controller::UserController>,
}

pub async fn new_user_domain(pool: Arc<sqlx::PgPool>, redis_client: Arc<redis::Client>) -> UserDomain {
    let db_repo = repositories::new_user_db_repository(pool);
    let cache_repo = repositories::new_user_cache_repository(redis_client);
    let service = service::new_user_service(db_repo, cache_repo);
    let controller = controller::new_user_controller(service);
    UserDomain { controller }
}
