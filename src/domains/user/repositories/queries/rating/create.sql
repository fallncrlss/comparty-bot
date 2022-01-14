INSERT INTO rating (user_tg_id, by_user_tg_id, chat_id, amount, comment) VALUES ($1, $2, $3, $4, $5) RETURNING rating_id;
