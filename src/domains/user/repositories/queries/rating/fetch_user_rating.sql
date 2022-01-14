SELECT ROUND(CAST(COALESCE(SUM(amount), 0.00) as numeric), 2) AS "amount!" FROM rating
INNER JOIN "user" u ON u.telegram_id = rating.user_tg_id
WHERE u.telegram_id = $1 AND chat_id = $2;
