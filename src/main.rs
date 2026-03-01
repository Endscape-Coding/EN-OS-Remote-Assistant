use std::env;
use std::process::Command;
use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;
use teloxide::dispatching::Dispatcher;
use crate::handlers::data;

mod handlers;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
enum Botcommand {
    #[command(description = "Run command")]
    Cmd(String),

    #[command(description = "Change Directory")]
    Cd(String),

    #[command(description = "Download file")]
    Download(String),

    #[command(description = "Download file")]
    Input(String),

    Ls,

    Start,

    Help,

    Screenshot,

    Setlang,
}

#[tokio::main]
async fn main() {
    let message;
    dotenvy::dotenv().expect("Failed foad from .env file");
    println!("EN-OS Remote Assistant");
    pretty_env_logger::init();
    log::info!("Starting bot...");
    let bot = Bot::from_env();

    let id: i64 = env::var("ID").expect("Error").parse::<i64>().expect("Error parce your id");

    let config = handlers::config_read;

    let ydt: bool = exec_ydt().await;

    if !ydt {
        log::warn!("Ydotool not installed..");
        let _ = bot.send_message(ChatId(id), "Ydotool not install or installed incorrectly! Input may not work!")
        .parse_mode(teloxide::types::ParseMode::Html)
        .await;
    }

    if config().unwrap().lang == "ru" {
        message = String::from(data::ONRU);
    } else {
        message = String::from(data::ONEN);
    }

    let _ = bot.send_message(ChatId(id), message)
    .parse_mode(teloxide::types::ParseMode::Html)
    .await;

    let handler = dptree::entry()
    .branch(
        Update::filter_message()
        .filter_command::<Botcommand>()
        .filter(move |msg:Message| {
            msg.chat.id == ChatId(id)
        })
        .branch(dptree::case![Botcommand::Cmd(args)].endpoint(handlers::cmd))
        .branch(dptree::case![Botcommand::Start].endpoint(handlers::start))
        .branch(dptree::case![Botcommand::Help].endpoint(handlers::start))
        .branch(dptree::case![Botcommand::Setlang].endpoint(handlers::setlang))
        .branch(dptree::case![Botcommand::Screenshot].endpoint(handlers::screen))
        .branch(dptree::case![Botcommand::Cd(args)].endpoint(handlers::cd))
        .branch(dptree::case![Botcommand::Download(args)].endpoint(handlers::download))
        .branch(dptree::case![Botcommand::Ls].endpoint(handlers::ls))
        .branch(dptree::case![Botcommand::Input(args)].endpoint(move |bot: Bot, msg: Message, args: String| {handlers::input(bot, msg, Botcommand::Input(args), ydt)}))
    )
    .branch(
        Update::filter_callback_query()
        .filter(move |q: CallbackQuery| q.from.id == UserId(id as u64))
        .endpoint(handlers::setlang_callback)
    );

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

async fn exec_ydt() -> bool {
    if !check_ydt().await {
        return false;
    }

    log::info!("Starting ydotoold...");
    match Command::new("ydotoold").spawn() {
        Ok(child) => {
            log::info!("Ydotoold started with PID: {}", child.id());
            true
        }
        Err(e) => {
            log::error!("Failed to start ydotoold: {}", e);
            false
        }
    }
}

async fn check_ydt() -> bool {
    let cmd = Command::new("which")
    .arg("ydotool")
    .output();

    if cmd.expect("Error..?").status.success() {
        log::info!("Ydotool has been installed!");
        return true;
    } else {
        log::error!("Ydotool not installed!");
        return false;
    }
}
