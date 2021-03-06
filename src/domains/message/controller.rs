use crate::{domains::message::service::MessageService, lib, Requester};
use async_trait::async_trait;
use teloxide;
use crate::lib::errors::MessageControllerError;
use crate::lib::types::MessageContext;

#[async_trait]
pub trait MessageController: Send + Sync {
    async fn check_link_in_message(
        &self, cx: &lib::types::MessageContext,
    ) -> Result<(), lib::errors::MessageControllerError>;
    async fn check_new_member(
        &self, cx: &crate::lib::types::MessageContext, new_member: &teloxide::types::User,
    ) -> Result<(), lib::errors::MessageControllerError>;
    async fn check_author(
        &self, cx: &crate::lib::types::MessageContext,
    ) -> Result<(), lib::errors::MessageControllerError>;
    async fn check_politics_in_text(
        &self, cx: &crate::lib::types::MessageContext,
    ) -> Result<(), lib::errors::MessageControllerError>;
    async fn check_insult_in_text(
        &self, cx: &crate::lib::types::MessageContext,
    ) -> Result<(), lib::errors::MessageControllerError>;
}

struct MessageControllerImpl {
    service: Box<dyn MessageService>,
}

#[async_trait]
impl MessageController for MessageControllerImpl {
    async fn check_link_in_message(
        &self, cx: &lib::types::MessageContext,
    ) -> Result<(), lib::errors::MessageControllerError> {
        let chat_id = cx.update.chat_id();
        let sender = cx.update.from().unwrap();
        let text = cx.update.text().unwrap();

        if let Ok(link) = lib::helpers::check_link_in_text(text) {
            log::info!("Found the prohibited link: {}!", link);

            let result = cx.requester
                .delete_message(chat_id, cx.update.id)
                .await
                .map_err(|err| err.into())
                .map_err(lib::errors::MessageControllerError::CheckLinkInMessage);
            if let Err(error) = result {
                log::warn!(
                    "Oops, error occurred deleting the user (full name: {}, id: {}) : {:#?}",
                    sender.full_name(),
                    sender.id,
                    error,
                );
                let admin_mentions = lib::tg_helpers::get_chat_administrator_mentions(cx)
                    .await
                    .map_err(lib::errors::MessageControllerError::CheckLinkInMessage)?;
                let msg_text = "Замечена подозрительная ссылка! \
                   Администрация проинформирована и разберётся в ситуации за кратчайшие сроки."
                    .to_owned()
                    + &admin_mentions.join("");
                lib::tg_helpers::reply_to(cx, msg_text)
                    .await
                    .map_err(lib::errors::MessageControllerError::CheckLinkInMessage)?;
                return Err(error);
            }

            let result = cx.requester
                .kick_chat_member(chat_id, sender.id)
                .await
                .map_err(|err| err.into())
                .map_err(lib::errors::MessageControllerError::CheckLinkInMessage);

            match result {
                Ok(_) => {
                    lib::tg_helpers::send_message(
                        cx,
                        format!(
                            "Пользователь {} был забанен за запрещённую ссылку в сообщении.",
                            teloxide::utils::html::user_mention_or_link(sender),
                        ),
                    )
                        .await
                        .map_err(lib::errors::MessageControllerError::CheckLinkInMessage)?;
                    log::info!("Ban user {} from chat {}.", sender.full_name(), chat_id);
                }
                Err(error) => {
                    log::warn!(
                        "Oops, error occurred deleting the user (full name: {}, id: {}) : {:#?}",
                        sender.full_name(),
                        sender.id,
                        error,
                    );
                    let admin_mentions = lib::tg_helpers::get_chat_administrator_mentions(cx)
                        .await
                        .map_err(lib::errors::MessageControllerError::CheckLinkInMessage)?;
                    let msg_text = format!(
                        "Замечен подозрительная ссылка у пользователя {}!\
                        Администрация проинформирована и разберётся в ситуации за кратчайшие сроки.",
                        teloxide::utils::html::user_mention_or_link(sender))
                        .to_owned()
                        + &admin_mentions.join("");
                    lib::tg_helpers::send_message(cx, msg_text)
                        .await
                        .map_err(lib::errors::MessageControllerError::CheckLinkInMessage)?;
                }
            }
        }
        Ok(())
    }

    async fn check_new_member(
        &self,
        cx: &lib::types::MessageContext,
        new_member: &teloxide::types::User,
    ) -> Result<(), lib::errors::MessageControllerError> {
        let chat = &cx.update.chat;
        log::info!("new member \"{}\" in chat \"{}\"", new_member.full_name(), chat.title().unwrap());

        let resp = self.service
            .get_cas_status(new_member.id)
            .await
            .map_err(|err| err.into())
            .map_err(lib::errors::MessageControllerError::CheckNewMember)?;

        if resp.ok {
            let result = cx.requester
                .kick_chat_member(chat.id, new_member.id)
                .await
                .map(|_| {
                    log::info!("User {} (id: {}) is banned due to CAS.", new_member.full_name(), new_member.id);
                })
                .map_err(|err| err.into())
                .map_err(lib::errors::MessageControllerError::CheckNewMember);

            match result {
                Ok(_) => {
                    let result = cx.requester
                        .delete_message(chat.id, cx.update.id)
                        .await
                        .map_err(|err| err.into())
                        .map_err(lib::errors::MessageControllerError::CheckLinkInMessage);
                },
                Err(error) => {
                    log::warn!(
                        "Oops, error occurred deleting the user (full name: {}, id: {}) : {:#?}",
                        new_member.full_name(),
                        new_member.id,
                        error,
                    );
                    let admin_mentions = lib::tg_helpers::get_chat_administrator_mentions(cx)
                        .await
                        .map_err(lib::errors::MessageControllerError::CheckLinkInMessage)?;
                    let msg_text = "Замечен подозрительный пользователь в соответствии с CAS! \
                   Администрация проинформирована и разберётся в ситуации за кратчайшие сроки."
                        .to_owned()
                        + &admin_mentions.join("");
                    lib::tg_helpers::reply_to(cx, msg_text)
                        .await
                        .map_err(lib::errors::MessageControllerError::CheckLinkInMessage)?;
                    return Err(error);
                }
            }
        }
        Ok(())
    }

    async fn check_author(&self, cx: &MessageContext) -> Result<(), MessageControllerError> {
        let chat_id = cx.update.chat_id();
        let user = cx.update.from().unwrap();
        if lib::helpers::check_is_full_name_clean(user.full_name()) {
            let result = cx.requester
                .kick_chat_member(chat_id, user.id)
                .await
                .map(|_| {
                    log::info!("User {} (id: {}) is banned due to inappropriate full name.", user.full_name(), user.id);
                })
                .map_err(|err| err.into())
                .map_err(lib::errors::MessageControllerError::CheckLinkInMessage);
            match result {
                Ok(_) => {
                    lib::tg_helpers::reply_to(
                        cx,
                        "Пользователь был забанен за запрещённое имя пользователя.".to_string(),
                    )
                        .await
                        .map_err(lib::errors::MessageControllerError::CheckLinkInMessage)?;
                }
                Err(error) => {
                    log::warn!(
                        "Oops, error occurred deleting the user (full name: {}, id: {}) : {:#?}",
                        user.full_name(),
                        user.id,
                        error,
                    );
                    let admin_mentions = lib::tg_helpers::get_chat_administrator_mentions(cx)
                        .await
                        .map_err(lib::errors::MessageControllerError::CheckLinkInMessage)?;
                    let msg_text = "Замечен пользователь с подозрительным именем пользователя! \
                   Администрация проинформирована и разберётся в ситуации за кратчайшие сроки."
                        .to_owned()
                        + &admin_mentions.join("");
                    lib::tg_helpers::reply_to(cx, msg_text)
                        .await
                        .map_err(lib::errors::MessageControllerError::CheckLinkInMessage)?;
                    return Err(error);
                }
            }
        }
        Ok(())
    }

    async fn check_politics_in_text(&self, cx: &MessageContext) -> Result<(), MessageControllerError> {
        let text = cx.update.text().unwrap();

        if lib::helpers::check_is_politics_in_text(text.to_string()) {
            let msg_text = "Просимо не згадувати політичні теми та не ображати людей за політичною ознакою, щоб уникнути подальшого конфлікту та агресії.

Якщо це повідомлення у відповідь на образу, агресію або пропаганду – використовуйте команду !report. Якщо можливо, видаліть або відредагуйте повідомлення.
Інакше, ви підвищуєте можливість отримати тимчасове обмеження написання повідомлень, у деяких випадках – отримання бана.

Дякуємо за розуміння.



Просим не упоминать политические темы и не оскорблять людей по политическому признаку во избежании дальнейшего конфликта и агрессии.

Если это ответное сообщение на оскорбление, агрессию или пропаганду – используйте команду !report. По возможности, удалите или отредактируйте сообщение.
В противном случае, вы повышаете вероятность получить временное ограничение на написание сообщений, в особых случаях – получение бана.

Благодарим за понимание.
";
            lib::tg_helpers::reply_to(cx, msg_text.to_string())
                .await
                .map_err(lib::errors::MessageControllerError::SendAnswer)?;
        }
        Ok(())
    }

    async fn check_insult_in_text(&self, cx: &MessageContext) -> Result<(), MessageControllerError> {
        let text = cx.update.text().unwrap();

        if lib::helpers::check_is_insult_in_text(text.to_string()) {
            let msg_text = "Просимо виявляти повагу до кожного учасника будь-ким і не використовувати образи, щоб уникнути подальшого конфлікту та агресії.
Якщо це повідомлення у відповідь на образу, агресію або пропаганду – використовуйте команду !report та адміністрація розбереться із ситуацією. Якщо можливо, видаліть або відредагуйте повідомлення.
Інакше, ви підвищуєте можливість отримати тимчасове обмеження написання повідомлень, у деяких випадках – отримання бана.

Дякуємо за розуміння.



Просим проявлять уважение к каждому участнику кем бы он ни был и не использовать оскорбления во избежании дальнейшего конфликта и агрессии.
Если оскорбления были использованы в ответ – используйте команду !report и администрация разберётся с ситуацией. По возможности, удалите или отредактируйте сообщение.
В противном случае, вы повышаете вероятность получить временное ограничение на написание сообщений, в особых случаях – получение бана.

Благодарим за понимание.
";
            lib::tg_helpers::reply_to(cx, msg_text.to_string())
                .await
                .map_err(lib::errors::MessageControllerError::SendAnswer)?;
        }
        Ok(())
    }
}

pub fn new_message_controller(service: Box<dyn MessageService>) -> Box<dyn MessageController> {
    Box::new(MessageControllerImpl { service })
}
