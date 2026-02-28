use std::io;
use teloxide::prelude::*;
use crate::handlers::data;
use crate::handlers::config_read;
use teloxide::types::{KeyboardButton, KeyboardMarkup};

pub async fn start(bot: Bot, msg: Message) -> io::Result<()> {
    let message: String;
    let config = config_read();

    if config.unwrap().lang == "ru" {
        message = String::from(data::STARTRU);
    } else {
        message = String::from(data::STARTEN);
    }
    let buttons = vec![
        vec![KeyboardButton::new("/start"), KeyboardButton::new("/screenshot")],
        vec![KeyboardButton::new("/ls"), KeyboardButton::new("/setlang")],
    ];

    let keyboard = KeyboardMarkup::new(buttons)
    .resize_keyboard();

    let _ = bot.send_message(msg.chat.id, message)
    .parse_mode(teloxide::types::ParseMode::Html)
    .reply_markup(keyboard)
    .await;

    Ok(())
}

