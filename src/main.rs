extern crate openssl;

use std::sync::Arc;
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

    let app_environment = injected::setup_environment().await;
    let domain_holder = Arc::new(app_environment.domain_holder);
    let domain_holder_callback = domain_holder.clone();

    teloxide::prelude::Dispatcher::new(bot)
        .messages_handler(|rx| {
            UnboundedReceiverStream::new(rx).for_each_concurrent(None, move |cx| {
                let domain_holder_clone = domain_holder.clone();
                async move {
                    core::handlers::message_handler(&cx, domain_holder_clone)
                        .await
                        .log_on_error()
                        .await;
                }
            },
            )
        })
        .callback_queries_handler(|rx| {
            UnboundedReceiverStream::new(rx).for_each_concurrent(None, move |cx| {
                let domain_holder_clone_callback = domain_holder_callback.clone();
                async move {
                    core::handlers::callback_handler(&cx, domain_holder_clone_callback)
                        .await
                        .log_on_error()
                        .await;
                }
            }
            )
        })
        .dispatch()
        .await;

    log::info!("Closing bot... Goodbye!");
}

#[tokio::main]
async fn main() {
    run().await;
}
