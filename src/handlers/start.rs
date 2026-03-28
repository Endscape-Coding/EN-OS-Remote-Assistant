//!
//! Start - стартовая команда.  
//! Выдает основные кнопки и дает справку по программе
//! 

use std::io;
use teloxide::prelude::*;
use teloxide::types::{KeyboardButton, KeyboardMarkup};
use crate::handlers::{data, get_config};

//Start - Самая база, справка
pub async fn start(bot: Bot, msg: Message) -> io::Result<()> {
    log::info!("Command: start <3");

    let config = match get_config(bot.clone(), msg.chat.id).await {
        Some(c) => c,
        None => return Ok(()),
    };

    let message = if config.lang == "ru" {
        data::STARTRU
    } else {
        data::STARTEN
    };

    let buttons = vec![
        vec![
            KeyboardButton::new("/start"),
            KeyboardButton::new("/screenshot"),
        ],
        vec![
            KeyboardButton::new("/filemanager"),
            KeyboardButton::new("/powerman"),
        ],
        vec![
            KeyboardButton::new("/input"),
            KeyboardButton::new("/setlang"),
        ],
    ];

    let keyboard = KeyboardMarkup::new(buttons).resize_keyboard();

    let _ = bot
        .send_message(msg.chat.id, message)
        .parse_mode(teloxide::types::ParseMode::Html)
        .reply_markup(keyboard)
        .await;

    Ok(())
}
