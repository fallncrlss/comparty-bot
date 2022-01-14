use crate::domains::message::{controller, service};

pub struct MessageDomain {
    pub controller: Box<dyn controller::MessageController>,
}

pub async fn new_message_domain() -> MessageDomain {
    let service = service::new_message_service();
    let controller = controller::new_message_controller(service);
    MessageDomain { controller }
}
