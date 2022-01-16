INSERT INTO rating(chat_user_id, by_chat_user_id, amount, comment)
VALUES (
        (SELECT cu.chat_user_id FROM chat_user cu INNER JOIN "user" u on u.user_id = cu.user_id WHERE u.telegram_id = $1 AND cu.chat_id = $3),
        (SELECT cu.chat_user_id FROM chat_user cu INNER JOIN "user" u on u.user_id = cu.user_id WHERE u.telegram_id = $2 AND cu.chat_id = $3),
        $4,
        $5
) RETURNING rating_id;