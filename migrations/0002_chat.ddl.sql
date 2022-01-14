BEGIN;
CREATE TABLE IF NOT EXISTS chat (
    chat_id bigint primary key,
    title varchar(128) not null
);
COMMIT;
