BEGIN;
CREATE TABLE IF NOT EXISTS "user" (
    user_id uuid primary key default gen_random_uuid(),
    telegram_id bigint unique not null,
    username varchar(32),
    first_name varchar(64) not null,
    last_name varchar(64)
);
COMMIT;
