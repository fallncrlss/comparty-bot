pub struct ChatSettingsChangeRequest {
    pub chat_id: i64,
    pub is_rating_count: bool,
    pub commands_for_admin_only: bool,
}