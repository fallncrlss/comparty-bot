use sqlx;

pub struct UserRequest {
    pub telegram_id: i64,
    pub username: Option<String>,
    pub first_name: String,
    pub last_name: Option<String>,
}

#[derive(Debug)]
pub struct TopUsersRequest {
    pub chat_id: i64,
    pub limit: i64,
}
#[derive(Debug)]
pub struct TopUsersResponse {
    pub full_name: String,
    pub rating_amount: sqlx::types::BigDecimal,
}

#[derive(Debug)]
pub struct RatingRequest {
    pub user_tg_id: i64,
    pub by_user_tg_id: i64,
    pub chat_id: i64,
    pub comment: Option<String>,
    pub amount: sqlx::types::BigDecimal,
}

pub struct UserRatingRequest {
    pub user_tg_id: i64,
    pub chat_id: i64,
}

#[derive(Clone, Copy)]
pub struct UserRatingActionRequest {
    pub user_id: i64,
    pub by_user_id: i64,
    pub chat_id: i64,
}

impl UserRatingActionRequest {
    pub fn get_key(&self) -> String {
        format!("{}-{}-{}", self.user_id, self.by_user_id, self.chat_id)
    }
}