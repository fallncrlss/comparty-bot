BEGIN;
CREATE TABLE IF NOT EXISTS chat_user (
    chat_user_id uuid primary key default gen_random_uuid(),
    user_id uuid not null,
    chat_id bigint not null,
    UNIQUE (user_id, chat_id),
    constraint fk_user foreign key (user_id) references "user"(user_id) ON DELETE CASCADE,
    constraint fk_chat foreign key (chat_id) references chat(chat_id) ON DELETE CASCADE
);
COMMIT;