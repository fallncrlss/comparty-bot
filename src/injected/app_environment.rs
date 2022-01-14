use crate::injected::domain_holder::{new_domain_holder, DomainHolder};

pub struct AppEnvironment {
    pub domain_holder: DomainHolder,
}

pub async fn setup_environment() -> AppEnvironment {
    let domain_holder = new_domain_holder().await;
    AppEnvironment { domain_holder }
}
