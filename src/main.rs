use std::env;
use std::process::Command;
use teloxide::dispatching::Dispatcher;
use teloxide::prelude::*;
use teloxide::types::Message;
use teloxide::utils::command::BotCommands;
use crate::handlers::data;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "."]
#[include = "log4rs.yaml"]
struct Asset;

mod handlers;

const VERSION: f64 = 0.4;

//Команды для ботов

#[derive(BotCommands, Clone)]
#[command(rename_rule = "snake_case")]
enum Botcommand {
    #[command(description = "Run command")]
    Cmd(String),

    #[command(description = "Run command with output")]
    CmdOutput(String),

    #[command(description = "Change Directory")]
    Cd(String),

    #[command(description = "CRemove file/dir")]
    Rm(String),

    #[command(description = "Download file")]
    Download(String),

    #[command(description = "Emulate input")]
    Input(String),

    #[command(description = "Open link")]
    Openlink(String),

    Command,

    Filemanager,

    Ls,

    Start,

    Help,

    Powerman,

    Shutdown,

    Reboot,

    Sleep,

    Hibernate,

    Screenshot,

    Setlang,
}

//Главный handler
#[tokio::main]
async fn main() {
    let message;
    dotenvy::dotenv().expect("Failed foad from .env file");
    println!("EN-OS Remote Assistant v{}", VERSION);

    if !std::path::Path::new("log4rs.yaml").exists() {
        let embedded = Asset::get("log4rs.yaml").unwrap();
        let data = embedded.data.to_vec();
        std::fs::write("log4rs.yaml", data).unwrap();
        println!("Create log4rs.yaml");
    }

    //Инициализация логгера.
    log4rs::init_file("log4rs.yaml", Default::default()).expect("Failed to initialize logging");

    log::info!("Starting bot...");
    let bot = Bot::from_env();
    let id: i64 = env::var("ID")
        .expect("Error")
        .parse::<i64>()
        .expect("Error parce your id");

    let config = handlers::config_read;

    let ydt: bool = exec_ydt().await;

    if !ydt {
        log::warn!("Ydotool not installed!");
        let _ = bot
            .send_message(
                ChatId(id),
                "Ydotool not install or installed incorrectly! Input may not work!",
            )
            .parse_mode(teloxide::types::ParseMode::Html)
            .await;
    }

    if config().unwrap().lang == "ru" {
        message = String::from(data::ONRU);
    } else {
        message = String::from(data::ONEN);
    }

    let _ = bot
        .send_message(ChatId(id), message)
        .parse_mode(teloxide::types::ParseMode::Html)
        .await;

    let handler = dptree::entry()
        .branch(
            Update::filter_message()
                //Проверка на id (даниил колбасенко)
                .filter(move |msg: Message| msg.chat.id == ChatId(id))
                .branch(
                    //Проверка на файл (для загрузки)
                    dptree::filter(|msg: Message| msg.document().is_some())
                        .endpoint(handlers::upload),
                )
                .branch(
                    //Обработчики команд
                    dptree::entry()
                        .filter_command::<Botcommand>()
                        .branch(dptree::case![Botcommand::Command].endpoint(handlers::command))
                        .branch(dptree::case![Botcommand::Cmd(args)].endpoint(handlers::cmd))
                        .branch(
                            dptree::case![Botcommand::CmdOutput(args)]
                                .endpoint(handlers::cmd_output),
                        )
                        .branch(dptree::case![Botcommand::Start].endpoint(handlers::start))
                        .branch(dptree::case![Botcommand::Help].endpoint(handlers::start))
                        .branch(dptree::case![Botcommand::Setlang].endpoint(handlers::setlang))
                        .branch(dptree::case![Botcommand::Screenshot].endpoint(handlers::screen))
                        .branch(
                            dptree::case![Botcommand::Filemanager].endpoint(handlers::filemanager),
                        )
                        .branch(dptree::case![Botcommand::Cd(args)].endpoint(handlers::cd))
                        .branch(dptree::case![Botcommand::Rm(args)].endpoint(handlers::rm))
                        .branch(
                            dptree::case![Botcommand::Download(args)].endpoint(handlers::download),
                        )
                        .branch(dptree::case![Botcommand::Ls].endpoint(handlers::ls))
                        .branch(dptree::case![Botcommand::Input(args)].endpoint(
                            move |bot: Bot, msg: Message, args: String| {
                                handlers::input(bot, msg, Botcommand::Input(args), ydt)
                            },
                        ))
                        .branch(dptree::case![Botcommand::Powerman].endpoint(
                            move |bot: Bot, msg: Message| handlers::powerman(bot, msg, 0),
                        ))
                        .branch(dptree::case![Botcommand::Shutdown].endpoint(
                            move |bot: Bot, msg: Message| handlers::powerman(bot, msg, 1),
                        ))
                        .branch(dptree::case![Botcommand::Reboot].endpoint(
                            move |bot: Bot, msg: Message| handlers::powerman(bot, msg, 2),
                        ))
                        .branch(dptree::case![Botcommand::Sleep].endpoint(
                            move |bot: Bot, msg: Message| handlers::powerman(bot, msg, 3),
                        ))
                        .branch(dptree::case![Botcommand::Hibernate].endpoint(
                            move |bot: Bot, msg: Message| handlers::powerman(bot, msg, 4),
                        ))
                        .branch(
                            dptree::case![Botcommand::Openlink(args)].endpoint(handlers::openlink),
                        ),
                ),
        )
        .branch(
            Update::filter_callback_query()
                .filter(move |q: CallbackQuery| q.from.id == UserId(id as u64))
                .endpoint(handlers::setlang_callback),
        );

    //Включаем dispatcher в работу.
    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

//Проверка ydotool. Она есть если не ошибаюсь и в input, ненавижу дублирование кода, но у них чут чут разные цели.
async fn exec_ydt() -> bool {
    if !handlers::check_prog("ydotool").await {
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
