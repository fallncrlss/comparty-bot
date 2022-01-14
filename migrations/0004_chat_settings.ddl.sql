BEGIN;
CREATE TABLE IF NOT EXISTS chat_settings (
    chat_id bigint primary key,
    is_rating_count boolean not null
);
COMMIT;
