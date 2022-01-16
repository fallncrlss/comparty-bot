SELECT CONCAT(first_name, ' ', last_name) AS "full_name!", COALESCE(SUM(r.amount), 0.00) AS "rating_amount!"
FROM rating r
INNER JOIN chat_user cu ON cu.chat_user_id = r.chat_user_id
INNER JOIN "user" u ON u.user_id = cu.user_id
WHERE cu.chat_id = $1
GROUP BY u.user_id, u.first_name, u.last_name
ORDER BY "rating_amount!" DESC
LIMIT $2;