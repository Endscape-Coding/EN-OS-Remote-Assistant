//!
//! Log - Отправка логов  
//! Просто отправляет логи по пути: /var/log/en-os/remote_assistant/bot.log  
//!  
//! Пример команды: ```/log```  
//! 
use std::io;
use std::path::Path;
use teloxide::prelude::*;
use crate::handlers::{data, get_config, other::send};
use teloxide::types::{ChatAction, InputFile};

/// Просто отправляет файл пользователю
pub async fn log(bot: Bot, msg: Message) -> io::Result<()> {
    log::info!("Command: log");

    let config = match get_config(bot.clone(), msg.chat.id).await {
        Some(c) => c,
        None => return Ok(()),
    };

    let path = Path::new(data::LOG_PATH);

    if path.exists(){
        log::info!("Log exists");

        let message = if config.lang == "ru" {
            data::LOGRU
        } else {
            data::LOGEN
        };

        let _ = bot
           .send_chat_action(msg.chat.id, ChatAction::UploadDocument).await;
                                
        let _ = bot
            .send_document(msg.chat.id, InputFile::file(path))
            .caption(format!("<code>{}</code>", message))
            .parse_mode(teloxide::types::ParseMode::Html)
            .await;

    } else{
        log::error!("Logs not found");
        let message = match config.lang.as_str() {
            "ru" | "ua" => data::LOGERRRU,
            _ => data::LOGERREN,
        };

        send(&bot, &msg, message).await;
    }
    Ok(())
}