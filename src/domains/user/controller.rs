use crate::{
    domains::user::{model, service::UserService},
    lib, Request, Requester,
};
use async_trait::async_trait;
use teloxide::prelude::*;

#[async_trait]
pub trait UserController: Send + Sync {
    async fn create_if_not_exists(&self, user: &teloxide::types::User, chat_id: i64) -> Result<(), lib::errors::UserError>;
    async fn fetch_users_by_rating(&self, cx: &lib::types::MessageContext) -> Result<(), lib::errors::UserError>;
    async fn get_info(&self, cx: &lib::types::MessageContext) -> Result<(), lib::errors::UserError>;
    async fn create_rating_record(
        &self,
        cx: &lib::types::MessageContext,
        rating_trigger: lib::enums::RatingTriggers,
    ) -> Result<(), lib::errors::UserError>;
    async fn delete_rating_record_by_user_request(
        &self,
        cx: &lib::types::CallbackContext,
        user_id: i64,
        record_id: &str,
    ) -> Result<(), lib::errors::UserError>;
}

struct UserControllerImpl {
    service: Box<dyn UserService>,
}

#[async_trait]
impl UserController for UserControllerImpl {
    async fn create_if_not_exists(&self, user: &teloxide::types::User, chat_id: i64) -> Result<(), lib::errors::UserError> {
        let changed = self
            .service
            .create_if_not_exists(model::UserRequest {
                telegram_id: user.id,
                username: user.username.clone(),
                first_name: user.first_name.clone(),
                last_name: user.last_name.clone(),
            })
            .await?;
        if changed {
            return self.service
                .create_rating_record(model::RatingRequest {
                    chat_id,
                    user_tg_id: user.id,
                    by_user_tg_id: user.id, // TODO
                    amount: sqlx::types::BigDecimal::from(lib::config::BASE_RATING),
                    comment: Option::from("Default create record.".to_string()),
                })
                .await
                .map(|_| ());
        }
        Ok(())
    }

    async fn fetch_users_by_rating(&self, cx: &lib::types::MessageContext) -> Result<(), lib::errors::UserError> {
        let users = self
            .service
            .fetch_top_by_rating(model::TopUsersRequest {
                chat_id: cx.update.chat_id(),
                limit: 15,
            })
            .await?;
        let mut text = "Топ самых одобряемых пользователей данного чата:".to_string();
        for (index, user) in users.iter().enumerate() {
            text.push_str(&*format!(
                "\n{}. <b>{}</b> <b>{:.2}</b>",
                index + 1,
                user.full_name,
                user.rating_amount
            ));
        }
        lib::tg_helpers::send_message(cx, text)
            .await
            .map_err(lib::errors::UserError::FetchRatingTop)
    }

    async fn get_info(&self, cx: &lib::types::MessageContext) -> Result<(), lib::errors::UserError> {
        let user = cx.update.from().unwrap();
        let rating = self
            .service
            .get_rating(model::UserRatingRequest {
                user_tg_id: user.id,
                chat_id: cx.update.chat_id(),
            })
            .await
            .map_err(|err| err.into())
            .map_err(lib::errors::UserError::GetInfo)?;

        let text = format!(
            "Пользователь: <b>{}</b>\nРейтинг: <b>{:.2}</b>",
            user.full_name(),
            rating,
        );
        lib::tg_helpers::send_message(cx, text)
            .await
            .map_err(lib::errors::UserError::GetInfo)
    }

    async fn create_rating_record(
        &self,
        cx: &lib::types::MessageContext,
        rating_trigger: lib::enums::RatingTriggers,
    ) -> Result<(), lib::errors::UserError> {
        let chat_id = cx.update.chat_id();
        let user_initiated = cx.update.from().unwrap();
        let user_to_apply = cx.update.reply_to_message().unwrap().from().unwrap();

        let user_initiated_rating_result = self
            .service
            .get_rating(model::UserRatingRequest {
                chat_id,
                user_tg_id: user_initiated.id,
            })
            .await;

        if let Err(_) = user_initiated_rating_result {
            return lib::tg_helpers::send_message(cx, "Невозможно изменить рейтинг".to_string())
                .await
                .map_err(lib::errors::UserError::InsertRating);
        }

        let rating_to_apply_result = rating_trigger
            .valid_amount(user_initiated_rating_result.unwrap());
        if let Err(err) = rating_to_apply_result {
            return lib::tg_helpers::send_message(cx, err)
                .await
                .map_err(lib::errors::UserError::InsertRating);
        }
        let rating_to_apply = rating_to_apply_result.unwrap();

        let rating_record_result = self
            .service
            .create_rating_record(model::RatingRequest {
                chat_id,
                user_tg_id: user_to_apply.id,
                by_user_tg_id: user_initiated.id,
                amount: rating_to_apply.clone(),
                comment: Option::from("".to_string()),
            })
            .await;
        if let Err(ref err) = rating_record_result {
            let text = match err {
                lib::errors::UserError::RepeatingRequestDuringCooldown(msg) => msg.clone(),
                _ => "Невозможно изменить рейтинг".to_string()
            };
            return lib::tg_helpers::send_message(cx, text)
                .await
                .map_err(lib::errors::UserError::InsertRating);
        }

        let user_to_apply_rating = self
            .service
            .get_rating(model::UserRatingRequest {
                user_tg_id: user_to_apply.id,
                chat_id,
            })
            .await
            .map_err(|err| err.into())
            .map_err(lib::errors::UserError::InsertRating)?;

        let text = format!(
            "Пользователь <b>{}</b> изменил рейтинг <b>{}</b> до <b>{:.2}</b> ({}{:.2})",
            user_initiated.full_name(),
            user_to_apply.full_name(),
            user_to_apply_rating,
            rating_trigger.get_sign(),
            rating_to_apply.abs(),
        );

        let keyboard = teloxide::types::InlineKeyboardMarkup::new(vec![vec![
            teloxide::types::InlineKeyboardButton::callback(
                "Отменить".to_string(),
                format!("{} {}", user_initiated.id, rating_record_result.unwrap()),
            ),
        ]]);
        let msg = cx
            .reply_to(text)
            .reply_markup(keyboard)
            .send()
            .await
            .map_err(|err| err.into())
            .map_err(lib::errors::UserError::InsertRating)?;

        let task = cx
            .requester
            .edit_message_reply_markup(msg.chat.id, msg.id)
            .reply_markup(teloxide::types::InlineKeyboardMarkup::default());

        tokio::task::spawn(async {
            tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
            task.await
                .map_err(|err| log::warn!("Delay deleting message errored: {:?}", err));
        });

        Ok(())
    }

    async fn delete_rating_record_by_user_request(
        &self,
        cx: &lib::types::CallbackContext,
        user_id: i64,
        record_id: &str,
    )  -> Result<(), lib::errors::UserError> {
        if cx.update.from.id == user_id {
            self.service
                .delete_rating_record(record_id)
                .await?;
            cx.requester
                .answer_callback_query(&cx.update.id)
                .text("Изменение рейтинга отменено")
                .show_alert(true)
                .send()
                .await
                .map(|_| ())
                .map_err(|err| err.into())
                .map_err(lib::errors::UserError::DeleteRating)?;
            let message = cx.update.message.as_ref().unwrap();
            cx.requester
                .delete_message(message.chat_id(), message.id)
                .send()
                .await
                .map(|_| ())
                .map_err(|err| err.into())
                .map_err(lib::errors::UserError::DeleteRating)
        } else {
            cx.requester
                .answer_callback_query(&cx.update.id)
                .text("Это действие может совершить только инициатор данного действия")
                .show_alert(true)
                .send()
                .await
                .map(|_| ())
                .map_err(|err| err.into())
                .map_err(lib::errors::UserError::DeleteRating)
        }
    }
}

pub fn new_user_controller(service: Box<dyn UserService>) -> Box<dyn UserController> {
    Box::new(UserControllerImpl { service })
}
