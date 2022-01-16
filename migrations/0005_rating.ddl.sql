BEGIN;
CREATE TABLE IF NOT EXISTS rating (
    rating_id uuid primary key default gen_random_uuid(),
    chat_user_id uuid,
    by_chat_user_id uuid,
    amount decimal not null,
    comment text,
    created_at timestamp not null default current_timestamp,
    constraint fk_chat_user foreign key (chat_user_id) references chat_user(chat_user_id) ON DELETE CASCADE,
    constraint fk_by_chat_user foreign key (by_chat_user_id) references chat_user(chat_user_id) ON DELETE NO ACTION
);
COMMIT;