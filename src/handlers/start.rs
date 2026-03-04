use std::io;
use teloxide::prelude::*;
use crate::handlers::data;
use crate::handlers::config_read;
use teloxide::types::{KeyboardButton, KeyboardMarkup};

pub async fn start(bot: Bot, msg: Message) -> io::Result<()> {
    log::info!("Command: start <3");

    let config = match config_read() {
        Ok(config) => config,
        Err(e) => {
            log::error!("Config read failed: {}", e);
            let _ = bot.send_message(msg.chat.id, "Config error").await;
            return Ok(());
        }
    };

    let message = if config.lang == "ru" { data::STARTRU } else { data::STARTEN };

    let buttons = vec![
        vec![KeyboardButton::new("/start"), KeyboardButton::new("/screenshot")],
        vec![KeyboardButton::new("/ls"), KeyboardButton::new("/setlang")],
        vec![KeyboardButton::new("/input"), KeyboardButton::new("/setlang")],
    ];

    let keyboard = KeyboardMarkup::new(buttons)
    .resize_keyboard();

    let _ = bot.send_message(msg.chat.id, message)
    .parse_mode(teloxide::types::ParseMode::Html)
    .reply_markup(keyboard)
    .await;

    Ok(())
}

