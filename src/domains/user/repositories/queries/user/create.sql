INSERT INTO "user"(telegram_id, username, first_name, last_name)
VALUES ($1, $2, $3, $4)
ON CONFLICT (telegram_id)
DO UPDATE
SET username = $2, first_name = $3, last_name = $4
WHERE "user".username != $2 OR "user".first_name != $3 OR "user".last_name != $4;