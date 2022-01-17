UPDATE chat_settings
SET is_rating_count = $2, commands_for_admin_only = $3
WHERE chat_settings.chat_id = $1;