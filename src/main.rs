extern crate openssl;

use teloxide::prelude::{OnError, Request, Requester, RequesterExt, StreamExt};
use tokio_stream::wrappers::UnboundedReceiverStream;

mod core;
mod domains;
mod injected;
mod lib;

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting bot...");
    core::persistence::run_pg_migrations().await;
    let bot = teloxide::Bot::from_env()
        .parse_mode(teloxide::types::ParseMode::Html)
        .auto_send();

    teloxide::prelude::Dispatcher::new(bot)
        .messages_handler(
            |rx: teloxide::prelude::DispatcherHandlerRx<
                lib::types::ConfBot,
                teloxide::types::Message,
            >| {
                UnboundedReceiverStream::new(rx).for_each_concurrent(None, |cx| async move {
                    let app_environment = injected::setup_environment().await;
                    core::handlers::message_handler(&cx, app_environment.domain_holder)
                        .await
                        .log_on_error()
                        .await;
                })
            },
        )
        .callback_queries_handler(
            |rx: teloxide::prelude::DispatcherHandlerRx<lib::types::ConfBot, teloxide::types::CallbackQuery>| {
                UnboundedReceiverStream::new(rx)
                    .for_each_concurrent(None, |cx| async move {
                        let app_environment = injected::setup_environment().await;
                        core::handlers::callback_handler(&cx, app_environment.domain_holder)
                            .await
                            .log_on_error()
                            .await;
                    })
            },
        )
        .dispatch()
        .await;

    log::info!("Closing bot... Goodbye!");
}

#[tokio::main]
async fn main() {
    run().await;
}
