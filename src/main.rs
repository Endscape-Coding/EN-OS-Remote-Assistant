//! EN-OS Remote Assistant
//! 
//! Это главный файл он инициализирует бота, запускает dispatcher, хендлит все команды и проверяет id аккаунта.
//! Так же проверяет наличие ydotool для эмуляции ввода.
//!
use std::env;
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;
use teloxide::dispatching::Dispatcher;
use teloxide::prelude::*;
use teloxide::types::Message;
use teloxide::utils::command::BotCommands;
use teloxide::types::BotCommand;
use crate::handlers::{data, notify, wait_network, proxy_work, config_read};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "."]
#[include = "log4rs.yaml"]
struct Asset;

mod handlers;

const RETRIES: u32 = 15;

///Команды для ботов
#[derive(BotCommands, Clone)]
#[command(rename_rule = "snake_case")]
enum Botcommand {
    /// Запуск команды (с cmd.spawn без выводв)
    #[command(description = "Run command")]
    Cmd(String),
    /// Запуск команды с выводом
    #[command(description = "Run command with output")]
    CmdOutput(String),
    /// Переход по директориям
    #[command(description = "Change Directory")]
    Cd(String),
    /// Удаление файлов
    #[command(description = "CRemove file/dir")]
    Rm(String),
    /// Загрузка файла с компьютера
    #[command(description = "Download file")]
    Download(String),
    /// Эмуляция ввода
    #[command(description = "Emulate input")]
    Input(String),
    /// Открыть ссылку
    #[command(description = "Open link")]
    Openlink(String),
    /// Выбор таймаута для ```/cmd_output```
    #[command(description = "Set /cmd_output timeout")]
    SetTimeout(u64),
    /// Просто справка для команд
    Command,
    /// Справка для работы с файлами
    Filemanager,
    /// Просмотр директории 
    Ls,
    /// Главная команда/справка
    Start,
    /// Главная команда/справка
    Help,
    /// Информация о программе и компьютере
    Info,
    /// Получение логов
    Log,
    /// Настройки бота: выбор языка, настрйока уведомлений, изменения таймаута
    Settings,
    /// Настройка уведомлений (включить/выключить)
    SetNotify,
    /// Справка по работе с питанием компьютера
    Powerman,
    /// Выключить компьютер
    Shutdown,
    /// Перезагрузить компьютер
    Reboot,
    /// Переход компьютера в сон
    Sleep,
    /// Переход компьютера в гибернацию
    Hibernate,
    /// Создание и отправка скриншота (при первом запуске на wayland будет запрашиваться разрешение на скрин)
    Screenshot,
    /// Выбор языка (английский/русский)
    Setlang,
}

///Главный handler
#[tokio::main]
async fn main() {
    let message;
    dotenvy::dotenv().expect("Failed foad from .env file");
    println!("EN-OS Remote Assistant v{}", data::VERSION);

    let log_config = Asset::get("log4rs.yaml").expect("Failed to find log4rs.yaml in assets");

    let strconfig = std::str::from_utf8(log_config.data.as_ref())
        .expect("Error yaml parsing: (no valid UTF8)");

    let l4sconfig: log4rs::config::RawConfig = serde_yaml::from_str(strconfig)
        .expect("Failed to parse log4rs.yaml");

    log4rs::init_raw_config(l4sconfig).expect("Failed to initialize logging");

    let token = env::var("TELOXIDE_TOKEN")
    .expect("TELEGRAM_BOT_TOKEN must be set");

    log::info!("Starting bot...");
    let http_client = proxy_work();
    
    let bot = Bot::with_client(token, http_client.await);
    setcommand(&bot).await;

    let id: i64 = env::var("ID")
        .expect("Error")
        .parse::<i64>()
        .expect("Error parce your id");

    wait_network(&bot).await;

    let config = handlers::config_read;
    
    let home = env::var_os("HOME")
        .map(PathBuf::from);

    if let Some(path) = home {
        let _ = env::set_current_dir(path);
    }
        
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

    if config().unwrap().notify{
        let _ = notify("Remote assistant", "Program has been started", "/usr/share/icons/hicolor/64x64/apps/en-os-remote-assistant-creator.png", 2500).await;
    }

    if config().unwrap().lang == "ru" {
        message = String::from(data::ONRU);
    } else {
        message = String::from(data::ONEN);
    }

    let start_message = bot
        .send_message(ChatId(id), message)
        .parse_mode(teloxide::types::ParseMode::Html)
        .await;

    match start_message {
        Ok(_) => log::info!("Start message send succesfully!"),
        Err(e) => {if config().unwrap().notify{
                let _ = notify("Error send start message!", "Check your internet connection", "error", 7500).await;
                log::error!("Error send start message! Error: {}", e)
            }
        }
    }

    // Главный хендлер: 
    // - Проверяет сообщения на файл 
    // - Фильтрует по id (даниил колбасенко) 
    let handler = dptree::entry()
        .branch(
            Update::filter_message()
                .filter(move |msg: Message| msg.chat.id == ChatId(id))
                .branch(
                    dptree::filter(|msg: Message| msg.document().is_some())
                        .endpoint(handlers::upload),
                )
                .branch(
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
                        .branch(dptree::case![Botcommand::Info].endpoint(handlers::info))
                        .branch(dptree::case![Botcommand::Log].endpoint(handlers::log))
                        .branch(dptree::case![Botcommand::Settings].endpoint(handlers::settings))
                        .branch(dptree::case![Botcommand::SetNotify].endpoint(handlers::set_notify))
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
                            dptree::case![Botcommand::SetTimeout(args)]
                                .endpoint(handlers::set_cmd_timeout),
                        )

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

    let mut i = 0;

    //Включаем dispatcher в работу.
    loop {
        let mut dispatcher = Dispatcher::builder(bot.clone(), handler.clone())
            .enable_ctrlc_handler()
            .build();

        log::info!("Try start dispatching...");

        let result = dispatcher.dispatch().await;

        if i == 0 {
            log::info!("Dispatcher stopped gracefully. Exiting...");
            break;
        }
        i += 1;
        log::error!("Dispatcher failed to start: {:?}", result);
        if i >= RETRIES {
            log::error!("Max retries ({}) reached. Exiting...", RETRIES);
            break;
        }
        log::info!("Retrying in 10 seconds... (attempt {}/{})", i, RETRIES);
        tokio::time::sleep(Duration::from_secs(10)).await;
    }
}

/// Устанавливает комманды в меню.
async fn setcommand(bot: &Bot) {
    let config = match config_read() {
        Ok(config) => config,
        Err(e) => {
            log::error!("Config read failed: {}", e);
            return;
        }
    };

    let commands = match config.lang.as_str() {
        "ru" | "ua" =>
            vec![
                BotCommand::new("start", "Справка"),
                BotCommand::new("filemanager", "Работа с файлами"),
                BotCommand::new("command", "Запуск команд"),
                BotCommand::new("powerman", "Power manager"),
                BotCommand::new("screenshot", "Получить скриншот"),
                BotCommand::new("settings", "Настройки Remote Assistant"),
                BotCommand::new("log", "Получить логи Remote Assistant"),
                BotCommand::new("info", "О программе"),
            ],
        _ =>
            vec![
                BotCommand::new("start", "Help"),
                BotCommand::new("filemanager", "Work with files"),
                BotCommand::new("command", "Exec commands"),
                BotCommand::new("powerman", "Power manager"),
                BotCommand::new("screenshot", "Get screenshot"),
                BotCommand::new("settings", "Remote Assistant settings"),
                BotCommand::new("log", "Remote Assistants logs"),
                BotCommand::new("info", "О программе"),
            ],
    };

    match bot.set_my_commands(commands).await {
        Ok(_) => log::info!("Set command menu"),
        Err(e) => log::error!("Error setting menu: {}", e),
    }
}

/// Проверка наличия ydotool.
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
