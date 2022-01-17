use crate::{domains, injected, lib};
use itertools::Itertools;

pub async fn admin_commands_handler(
    cx: &lib::types::MessageContext,
    domain_holder: &injected::DomainHolder,
) -> Result<(), anyhow::Error> {
    let lowercased_vec = cx.update
        .text()
        .unwrap()
        .split_whitespace()
        .map(str::to_lowercase)
        .collect_vec();
    let result = &lowercased_vec.iter().map(String::as_str).collect_vec()[..];

    let is_admin = lib::helpers::is_admin(&cx)
        .await
        .map_err(lib::errors::AdminCommandsControllerError::GetInfo)?;
    let chat_id = cx.update.chat_id();
    let chat_settings = domain_holder
        .chat
        .controller
        .get_chat_settings(cx)
        .await
        .map_err(anyhow::Error::new)?;

    match result {
        ["!help"] if !chat_settings.commands_for_admin_only || is_admin => {
            return lib::tg_helpers::reply_to(
                cx, "\
<b>Основные команды:</b>
<code>!help</code> – вывести данное сообщение

<code>!report</code> – уведомить всех администраторов чата

<code>!ban</code> – бан ответом на сообщение требуемого пользователя

<code>!ro [time]</code> – read-only mode ответом на сообщение требуемого пользователя на введённое время, пример, <code>!ro 1h</code>.
В качестве единиц возможно использовать <code>s</code> | <code>m</code> | <code>h</code> | <code>d</code> секунды, минуты, часы, дни соответственно.
<i>Важно: при указании срока read-only меньше 30 секунд пользователь получит данный статус на неопределённый период!</i>


<b>Настройка чата:</b>
<code>!settings</code>  – текущие настройки чата

<code>!disable_rating_count</code>  – отключить подсчёт рейтинга. При отключении данные не стираются

<code>!enable_rating_count</code> – включить подсчёт рейтинга (по умолчанию, включён)

<code>!enable_commands_for_admin_only</code> – команды доступны исключительно администраторам чата

<code>!disable_commands_for_admin_only</code> – команды доступны для всех участников (по умолчанию)

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
                .await
        }
        ["!settings"] if is_admin => {
            domain_holder.admin_commands.controller.get_settings(cx, chat_settings).await
        }
        ["!report"] if !chat_settings.commands_for_admin_only => {
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
        ["!enable_rating_count"] if is_admin => {
            domain_holder
                .chat
                .controller
                .change_chat_settings(cx, domains::chat::ChatSettings{
                    chat_id,
                    is_rating_count: true,
                    commands_for_admin_only: chat_settings.commands_for_admin_only,
                })
                .await
        }
        ["!disable_rating_count"] if is_admin => {
            domain_holder
                .chat
                .controller
                .change_chat_settings(cx, domains::chat::ChatSettings {
                    chat_id,
                    is_rating_count: false,
                    commands_for_admin_only: chat_settings.commands_for_admin_only,
                })
                .await
        }
        ["!enable_commands_for_admin_only"] if is_admin => {
            domain_holder
                .chat
                .controller
                .change_chat_settings(cx, domains::chat::ChatSettings{
                    chat_id,
                    is_rating_count: chat_settings.is_rating_count,
                    commands_for_admin_only: true,
                })
                .await
        }
        ["!disable_commands_for_admin_only"] if is_admin => {
            domain_holder
                .chat
                .controller
                .change_chat_settings(cx, domains::chat::ChatSettings {
                    chat_id,
                    is_rating_count: chat_settings.is_rating_count,
                    commands_for_admin_only: false,
                })
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
        let is_admin = lib::helpers::is_admin(&cx)
            .await
            .map_err(lib::errors::UserError::GetInfo)?;

        if (!chat_settings.commands_for_admin_only && chat_settings.is_rating_count) ||
            (chat_settings.commands_for_admin_only && chat_settings.is_rating_count && is_admin) {
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
