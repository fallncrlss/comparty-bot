SELECT ROUND(CAST(COALESCE(SUM(amount), 0.00) as numeric), 2) AS "amount!" FROM rating r
INNER JOIN chat_user cu ON cu.chat_user_id = r.chat_user_id
INNER JOIN "user" u ON u.user_id = cu.user_id
WHERE u.telegram_id = $1 AND cu.chat_id = $2;
