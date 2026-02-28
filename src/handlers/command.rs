use std::io;
use std::process::Command;
use teloxide::prelude::*;
use crate::Botcommand;
use crate::handlers::data;
use crate::handlers::config_read;

pub async fn cmd(bot: Bot, msg: Message, command: Botcommand) -> io::Result<()> {
    match command {
        Botcommand::Cmd(args) => {
            let message;
            let config = config_read();
            if args.trim().is_empty() {
                if config.unwrap().lang == "ru" {
                    message = String::from(data::CMDHELPRU);
                } else {
                    message = String::from(data::CMDHELPEN);
                }

                let _ = bot.send_message(msg.chat.id, message)
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .await;
                return Ok(());
            }

            let command = Command::new("sh").arg("-c").arg(&args).output()?;
            log::info!("Exec command {}", &args);

            let mut message = match command.status.success() {
                true => String::from_utf8_lossy(&command.stdout).to_string(),
                false => String::from_utf8_lossy(&command.stderr).to_string()
            };
            if message.len() == 0 {
                if config.unwrap().lang == "ru" {
                    message = String::from(data::CMDNOOUTRU);
                } else {
                    message = String::from(data::CMDNOOUTEN);
                }
            }

            let _ = bot.send_message(msg.chat.id, message).await;
            Ok(())
        }
        _ => Ok(())
    }

}

