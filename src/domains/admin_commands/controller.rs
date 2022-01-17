use crate::{domains::{admin_commands::service::AdminCommandsService, chat}, lib, Requester};
use async_trait::async_trait;
use teloxide::payloads::RestrictChatMemberSetters;
use crate::lib::errors::AdminCommandsControllerError;
use crate::lib::types::MessageContext;

#[async_trait]
pub trait AdminCommandsController: Send + Sync {
    async fn report(&self, cx: &lib::types::MessageContext) -> Result<(), lib::errors::AdminCommandsControllerError>;
    async fn mute_user(&self, cx: &lib::types::MessageContext, time: &str) -> Result<(), lib::errors::AdminCommandsControllerError>;
    async fn ban_user(&self, cx: &lib::types::MessageContext) -> Result<(), lib::errors::AdminCommandsControllerError>;
    async fn get_settings(
        &self,
        cx: &lib::types::MessageContext,
        settings: chat::ChatSettings
    ) -> Result<(), lib::errors::AdminCommandsControllerError>;
}

struct AdminCommandsControllerImpl {
    service: Box<dyn AdminCommandsService>,
}

#[async_trait]
impl AdminCommandsController for AdminCommandsControllerImpl {
    async fn report(&self, cx: &lib::types::MessageContext) -> Result<(), lib::errors::AdminCommandsControllerError> {
        let msg_text = match cx.update.reply_to_message() {
            Some(_) => {
                let admin_mentions = self.service
                    .get_chat_administrator_mentions(cx)
                    .await
                    .map_err(|err| err.into())
                    .map_err(lib::errors::AdminCommandsControllerError::Report)?;
                "Благодарим за репорт! Администрация разберётся в ситуации за кратчайшие сроки."
                    .to_owned()
                    + &admin_mentions.join("")
            }
            None => "Используйте эту команду в ответ на сообщение!".to_string(),
        };
        lib::tg_helpers::reply_to(cx, msg_text)
            .await
            .map_err(lib::errors::AdminCommandsControllerError::Report)
    }

    async fn mute_user(&self, cx: &lib::types::MessageContext, time: &str) -> Result<(), lib::errors::AdminCommandsControllerError> {
        let msg_text = match cx.update.reply_to_message() {
            Some(msg) => {
                let sender = msg.from().unwrap();
                let restrict_time = self
                    .service
                    .get_restrict_time(time)
                    .await
                    .map_err(|err| err.into())
                    .map_err(lib::errors::AdminCommandsControllerError::MuteUser)?;

                let result = cx
                    .requester
                    .restrict_chat_member(
                        cx.update.chat_id(),
                        sender.id,
                        teloxide::types::ChatPermissions::default(),
                    )
                    .until_date(restrict_time.to_expire_date(cx.update.date as i64))
                    .await;

                match result {
                    Ok(_) => format!(
                        "Пользователь {} может только читать сообщения на протяжении <b>{}</b>.",
                        teloxide::utils::html::user_mention_or_link(sender),
                        restrict_time.to_string(),
                    ),
                    Err(error) => {
                        log::error!(
                            "Oops, error occurred restricting the user (full name: {}, id: {}) : {:#?}",
                            sender.full_name(),
                            sender.id,
                            error,
                        );
                        "Невозможно ограничить права пользователя. \
                        Пожалуйста, убедитесь, что бот имеет соответствующие права и повторите попытку позже.".to_string()
                    }
                }
            }
            None => "Используйте эту команду в ответ на сообщение!".to_string(),
        };

        lib::tg_helpers::reply_to(cx, msg_text)
            .await
            .map_err(lib::errors::AdminCommandsControllerError::MuteUser)
    }

    async fn ban_user(&self, cx: &lib::types::MessageContext) -> Result<(), lib::errors::AdminCommandsControllerError> {
        let msg_text = match cx.update.reply_to_message() {
            Some(msg) => {
                let sender = msg.from().unwrap();
                let result = cx
                    .requester
                    .kick_chat_member(cx.update.chat_id(), sender.id)
                    .await;

                match result {
                    Ok(_) => format!(
                        "Пользователь {} выгнан из чата.",
                        teloxide::utils::html::user_mention_or_link(sender)
                    ),
                    Err(error) => {
                        log::error!(
                            "Oops, error occurred deleting the user (full name: {}, id: {}) : {:#?}",
                            sender.full_name(),
                            sender.id,
                            error,
                        );
                        "Невозможно выгнать пользователя. \
                        Пожалуйста, убедитесь, что бот имеет соответствующие права и повторите попытку позже."
                            .to_string()
                    }
                }
            }
            None => "Используйте эту команду в ответ на сообщение!".to_string(),
        };
        lib::tg_helpers::reply_to(cx, msg_text)
            .await
            .map_err(lib::errors::AdminCommandsControllerError::BanUser)
    }

    async fn get_settings(&self, cx: &MessageContext, settings: chat::ChatSettings) -> Result<(), AdminCommandsControllerError> {
        let text = format!(
            "\
<b>Настройки чата:</b>
Подсчёт рейтинга: <b>{}</b>
Команды включены исключительно для админов: <b>{}</b>
",
            lib::helpers::bool_to_string_switch(settings.is_rating_count),
            lib::helpers::bool_to_string_switch(settings.commands_for_admin_only),
        );
        lib::tg_helpers::reply_to(cx, text)
            .await
            .map_err(lib::errors::AdminCommandsControllerError::GetSettings)
    }
}

pub fn new_admin_commands_controller(
    service: Box<dyn AdminCommandsService>,
) -> Box<dyn AdminCommandsController> {
    Box::new(AdminCommandsControllerImpl { service })
}
