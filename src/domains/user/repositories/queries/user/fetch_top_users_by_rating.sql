SELECT CONCAT(first_name, last_name) AS "full_name!", COALESCE(SUM(rating.amount), 0.00) AS "rating_amount!"
FROM rating
INNER JOIN "user" ON "user".telegram_id = rating.user_tg_id
WHERE rating.chat_id = $1
group by "user".user_id, "user".first_name, "user".last_name
ORDER BY "rating_amount!" DESC
LIMIT $2;