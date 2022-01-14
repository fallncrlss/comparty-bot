INSERT INTO chat(chat_id, title)
VALUES ($1, $2)
ON CONFLICT (chat_id) DO UPDATE
SET title = $2
WHERE chat.title != $2;