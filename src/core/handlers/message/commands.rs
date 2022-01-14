use crate::{injected, lib};
use itertools::Itertools;

pub async fn admin_commands_handler(
    cx: &lib::types::MessageContext,
    domain_holder: &injected::DomainHolder,
) -> Result<(), anyhow::Error> {
    let msg_text = cx.update.text().unwrap();
    let lowercased_vec = msg_text
        .split_whitespace()
        .map(str::to_lowercase)
        .collect_vec();
    let result = &lowercased_vec.iter().map(String::as_str).collect_vec()[..];

    match result {
        ["!help"] => {
            return lib::tg_helpers::send_message(
                cx, "\
<b>Основные команды:</b>
<code>!help</code> – вывести данное сообщение

<code>!report</code> – уведомить всех администраторов чата

<code>!ban</code> – бан ответом на сообщение требуемого пользователя

<code>!ro [time]</code> – read-only mode ответом на сообщение требуемого пользователя на введённое время, пример, <code>!ro 1h</code>.
В качестве единиц возможно использовать <code>s</code> | <code>m</code> | <code>h</code> | <code>d</code> секунды, минуты, часы, дни соответственно.
<i>Важно: при указании срока read-only меньше 30 секунд пользователь получит данный статус на неопределённый период!</i>


<b>Настройка чата:</b>
<code>!disable_rating_count</code>  – отключить подсчёт рейтинга. По умолчанию, включён. При отключении данные не стираются

<code>!enable_rating_count</code> – включить подсчёт рейтинга


<b>Рейтинг:</b>
<code>!me</code> – вывести свой рейтинг

<code>!top</code> – вывести топ-15 пользователей по рейтингу

<code>+</code> – добавить рейтинг ответом на сообщение требуемого пользователя.
Валидные способы: <code>+</code>, <code>+1</code>, <code>+1.23</code>, <code>спасибо</code>, <code>спс</code>, <code>благодарю</code>, <code>thanks</code>, <code>thx</code>, <code>thank you</code>, <code>👍</code>

<code>-</code> (minus) – уменьшить рейтинг ответом на сообщение требуемого пользователя.
Валидные способы: <code>-</code>, <code>-1</code>, <code>-1.23</code>, <code>👎</code>

Также этот бот:
- проверяет новых пользователей в чате в соответствии с <a href='https://cas.chat'>CAS</a> и общими ограничениями
- проверяет ссылки в соответствии с общими ограничениями
            ".to_string()
            )
                .await;
        }
        ["!report"] => {
            domain_holder.admin_commands.controller.report(cx).await
        }
        ["!ban"] if lib::helpers::is_admin(&cx).await? => {
            domain_holder.admin_commands.controller.ban_user(cx).await
        }
        ["!ro", time] if lib::helpers::is_admin(&cx).await? => {
            domain_holder
                .admin_commands
                .controller
                .mute_user(cx, time)
                .await
        }
        ["!enable_rating_count"] if lib::helpers::is_admin(&cx).await? => {
            domain_holder
                .chat
                .controller
                .change_chat_settings(cx, lib::enums::RatingCountSwitch::On)
                .await
        }
        ["!disable_rating_count"] if lib::helpers::is_admin(&cx).await? => {
            domain_holder
                .chat
                .controller
                .change_chat_settings(cx, lib::enums::RatingCountSwitch::Off)
                .await
        }
        _ => {Ok(())}
    }
        .map_err(|err| err.into())
}

pub async fn user_commands_handler(
    cx: &lib::types::MessageContext,
    domain_holder: &injected::DomainHolder,
)  -> Result<(), lib::errors::UserError> {
    let msg_text = cx.update.text().unwrap();
    let lowercased_vec = msg_text
        .split_whitespace()
        .map(str::to_lowercase)
        .collect_vec();
    let result = &lowercased_vec.iter().map(String::as_str).collect_vec()[..];

    if let Ok(chat_settings) = domain_holder.chat.controller.get_chat_settings(cx).await {
        if chat_settings.is_rating_count {
            match result {
                ["!top"] => {
                    domain_holder
                        .user
                        .controller
                        .fetch_users_by_rating(cx)
                        .await?;
                }
                ["!me"] => {
                    domain_holder
                        .user
                        .controller
                        .get_info(cx)
                        .await?;
                }
                _ => { return Ok(()); }
            }
        }
    }

    Ok(())
}
