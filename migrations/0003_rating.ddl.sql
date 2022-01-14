CREATE TABLE IF NOT EXISTS rating (
    rating_id uuid primary key default gen_random_uuid(),
    user_tg_id bigint,
    by_user_tg_id bigint,
    chat_id bigint,
    amount decimal not null,
    comment text,
    created_at timestamp not null default current_timestamp,
    constraint fk_user foreign key (user_tg_id) references "user"(telegram_id) ON DELETE CASCADE,
    constraint fk_by_user foreign key (by_user_tg_id) references "user"(telegram_id) ON DELETE NO ACTION,
    constraint fk_chat foreign key (chat_id) references chat(chat_id) ON DELETE CASCADE
)
