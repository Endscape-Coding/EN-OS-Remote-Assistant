use std::io;
use teloxide::prelude::*;
use crate::handlers::data;
use crate::handlers::config_read;

pub async fn start(bot: Bot, msg: Message) -> io::Result<()> {
    let mut message: String;
    let config = config_read();

    if config.unwrap().lang == "ru" {
        message = String::from(data::STARTRU);
    } else {
        message = String::from(data::STARTEN);
    }

    bot.send_message(msg.chat.id, message)
    .parse_mode(teloxide::types::ParseMode::Html)
    .await;

    Ok(())
}

