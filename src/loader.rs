use crate::config::Config;
use crate::handlers::markups::markuper::callback_query_handlers;
use crate::handlers::{commander::command_handlers, messages::messager::messages_handlers};
use crate::util::{enums::Command, errors::MyError};
use teloxide::dispatching::{Dispatcher, HandlerExt, UpdateFilterExt};
use teloxide::dptree;
use teloxide::prelude::Requester;
use teloxide::types::Update;
use teloxide::utils::command::BotCommands;

pub async fn run() -> Result<(), MyError> {
    let config = Config::new().await;

    let command_menu = Command::bot_commands();
    config
        .get_bot()
        .set_my_commands(command_menu.clone())
        .await?;

    let commands = Update::filter_message()
        .filter_command::<Command>()
        .endpoint(command_handlers);

    let messages = Update::filter_message().endpoint(messages_handlers);
    let callback_queries = Update::filter_callback_query().endpoint(callback_query_handlers);

    let handlers = dptree::entry()
        .branch(commands)
        .branch(callback_queries)
        .branch(messages);

    let bot_name = config.get_bot().get_my_name().await?;

    println!("Bot name: {:?}", bot_name.name);

    Dispatcher::builder(config.get_bot().clone(), handlers)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
    Ok(())
}
