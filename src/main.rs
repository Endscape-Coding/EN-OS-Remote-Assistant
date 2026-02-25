use std::env;
use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;
use teloxide::dispatching::Dispatcher;

mod handlers;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
enum Botcommand {
    #[command(description = "Run command")]
    Cmd(String),

    #[command(description = "Set sudo password")]
    Sudo(String),

    Start,

    Help,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Failed foad from .env file");
    println!("EN-OS Remote Assistant");
    pretty_env_logger::init();
    log::info!("Starting bot...");
    let bot = Bot::from_env();

    let id: i64 = env::var("ID").expect("Error").parse::<i64>().expect("Error parce your id");
    handlers::config_read;

    let handler = dptree::entry()
    .branch(
        Update::filter_message()
        .filter_command::<Botcommand>()
        .filter(move |msg:Message| {
            msg.chat.id == ChatId(id)
        })
        .branch(dptree::case![Botcommand::Cmd(args)].endpoint(handlers::cmd))
        .branch(dptree::case![Botcommand::Sudo(args)].endpoint(handlers::sudo))
        .branch(dptree::case![Botcommand::Start].endpoint(handlers::start))
        .branch(dptree::case![Botcommand::Help].endpoint(handlers::start))
    );

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
