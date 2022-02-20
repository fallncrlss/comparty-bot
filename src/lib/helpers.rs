use anyhow::Context;
use teloxide::prelude::Requester;
use crate::lib;

pub async fn is_admin(cx: &lib::types::MessageContext) -> Result<bool, anyhow::Error> {
    let admins = cx.requester
        .get_chat_administrators(cx.update.chat_id())
        .await
        .map_err(anyhow::Error::new)
        .context("Failed to fetch chat admins")?;
    Ok(admins.iter().any(|admin| cx.update.from().unwrap().id == admin.user.id))
}

pub fn find_url(text: &str) -> Result<String, &str> {
    let regexp = r"(http(s)?://.)?(www\.)?[-a-zA-Z0-9@:%._\+~#=]{2,256}\.[a-z]{2,6}\b([-a-zA-Z0-9@:%_\+.~#?&//=]*)";
    let re = regex::Regex::new(regexp).unwrap();

    for substring in re.captures_iter(text) {
        let url = &substring[0];
        return Ok(String::from(url));
    }
    Err("No URLs found")
}

pub fn check_is_full_name_clean(full_name: String) -> bool {
    lib::config::STOP_FULL_NAME_WORDS
        .iter()
        .any(|word| full_name.contains(word))
}

pub fn check_link_in_text(text: &str) -> Result<String, ()> {
    let link = find_url(text).map_err(|_| {})?;
    if lib::config::STOP_WORDS_IN_LINK.iter().any(|word| link.contains(word)) || link.len() <= 6 {
        return Ok(link);
    }
    Err(())
}

pub fn chars_to_float(chars: &[char]) -> Option<f64> {
    chars
        .iter()
        .collect::<String>()
        .parse()
        .ok()
        .map(|r: f64| math::round::floor(r, 2))
}

pub fn get_envvar(env: &'static str) -> String {
    std::env::var(env).unwrap_or_else(|_| panic!("Cannot get the {} env variable!", env))
}

pub fn bool_to_string_switch(item: bool) -> &'static str {
    match item {
        true => "Включён",
        false => "Отключён"
    }
}