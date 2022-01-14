use crate::{injected, lib};
use itertools::Itertools;

pub async fn cancel_rating_handler(
    cx: &lib::types::CallbackContext,
    domain_holder: &injected::DomainHolder,
) -> Result<(), lib::errors::UserError> {
    if let Some(data) = &cx.update.data {
        if let [user_id, rating_record_id] = data.split_whitespace().collect_vec()[..] {
            if let Ok(user_id) = user_id.parse::<i64>() {
                domain_holder
                    .user
                    .controller
                    .delete_rating_record_by_user_request(cx, user_id, rating_record_id)
                    .await?;
            }
        }
    }
    Ok(())
}
