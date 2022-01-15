use std::sync::Arc;
use crate::{core, injected::domain_holder::{new_domain_holder, DomainHolder}};

pub struct AppEnvironment {
    pub domain_holder: DomainHolder,
}

pub async fn setup_environment() -> AppEnvironment {
    let db_pool = Arc::new(core::persistence::new_pg_pool().await);
    let redis_client = Arc::new(core::persistence::new_redis_client().await);
    let domain_holder = new_domain_holder(db_pool, redis_client).await;
    AppEnvironment { domain_holder }
}
