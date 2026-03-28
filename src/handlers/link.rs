//!
//! Link - открытие ссылок
//! 
use std::io;
use teloxide::prelude::*;
use tokio::process::Command;
use crate::handlers::config_read;
use crate::handlers::other::send;
use crate::{Botcommand, handlers::data};

///
/// Все легко: программа открывает ссылку через xdg-open.
/// Пример: ```/openlink en-os.ru```
/// 
pub async fn openlink(bot: Bot, msg: Message, command: Botcommand) -> io::Result<()> {
    match command {
        Botcommand::Openlink(args) => {
            log::info!("Command: openlink");
            let config = match config_read() {
                Ok(config) => config,
                Err(e) => {
                    log::error!("Config read failed: {}", e);
                    let _ = bot.send_message(msg.chat.id, "Config error").await;
                    return Ok(());
                }
            };

            if args.trim().is_empty() {
                let message = if config.lang == "ru" {
                    data::LINKHELPRU
                } else {
                    data::LINKHELPEN
                };
                log::info!("No args");
                send(&bot, &msg, message).await;
                return Ok(());
            } else {
                let message = if config.lang == "ru" {
                    data::OPENLINKRU
                } else {
                    data::OPENLINKEN
                };
                let _ = bot.send_message(msg.chat.id, message).await;
            }

            let mut cmd = Command::new("xdg-open");
            cmd.arg(&args);

            match cmd.spawn() {
                Ok(..) => log::info!("Open link: {}", &args),
                Err(e) => log::info!("Ошибка {e}"),
            }
            Ok(())
        }
        _ => Ok(()),
    }
}
