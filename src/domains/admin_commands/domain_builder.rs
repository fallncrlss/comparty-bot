use crate::domains::admin_commands::{controller, service};

pub struct AdminCommandsDomain {
    pub controller: Box<dyn controller::AdminCommandsController>,
}

pub async fn new_admin_commands_domain() -> AdminCommandsDomain {
    let service = service::new_admin_commands_service();
    let controller = controller::new_admin_commands_controller(service);
    AdminCommandsDomain { controller }
}
