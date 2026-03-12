use std::io;
use tokio::process::Command;
use teloxide::prelude::*;
use crate::Botcommand;
use crate::handlers::data;
use crate::handlers::config_read;

//Cmd spawn, то есть без output, зато бот не лагает после выполнения.
pub async fn cmd(bot: Bot, msg: Message, command: Botcommand) -> io::Result<()> {
    match command {
        Botcommand::Cmd(args) => {
            log::info!("Command: cmd");

            let config = match config_read() {
                Ok(config) => config,
                Err(e) => {
                    log::error!("Config read failed: {}", e);
                    let _ = bot.send_message(msg.chat.id, "Config error").await;
                    return Ok(());
                }
            };

            if args.trim().is_empty() {
                let message = if config.lang == "ru" { data::CMDHELPRU } else { data::CMDHELPEN };

                let _ = bot.send_message(msg.chat.id, message)
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .await;
                return Ok(());
            }

            let _ = Command::new("sh").arg("-c").arg(&args).spawn()?;

            let message = if config.lang == "ru" { data::CMDSPAWNRU } else { data::CMDSPAWNEN };

            log::info!("Exec command {}", &args);

            let _ = bot.send_message(msg.chat.id, format!("{} {}",message, &args)).await;
            Ok(())
        }
        _ => Ok(())
    }

}

//Cmd output: Ждет завершения программы.
pub async fn cmd_output(bot: Bot, msg: Message, command: Botcommand) -> io::Result<()> {
    match command {
        Botcommand::CmdOutput(args) => {
            log::info!("Command: cmd_output");

            let config = match config_read() {
                Ok(config) => config,
                Err(e) => {
                    log::error!("Config read failed: {}", e);
                    let _ = bot.send_message(msg.chat.id, "Config error").await;
                    return Ok(());
                }
            };

            if args.trim().is_empty() {
                let message = if config.lang == "ru" { data::CMDHELPRU } else { data::CMDHELPEN };

                let _ = bot.send_message(msg.chat.id, message)
                .parse_mode(teloxide::types::ParseMode::Html)
                .await;
                return Ok(());
            }
            if args.trim().chars().last() == Some('&') {
                let message = "Dont exec command with &, bot can dont responce!".to_string();
                let _ = bot.send_message(msg.chat.id, message)
                .parse_mode(teloxide::types::ParseMode::Html)
                .await;
                return Ok(())
            }

            let command = Command::new("sh").arg("-c").arg(&args).output().await?;
            log::info!("Exec command (with output) {}", &args);

            let mut message = match command.status.success() {
                true => String::from_utf8_lossy(&command.stdout).to_string(),
                false => String::from_utf8_lossy(&command.stderr).to_string()
            };
            if message.len() == 0 {
                message = if config.lang == "ru" { data::CMDNOOUTRU.to_string() } else { data::CMDNOOUTEN.to_string() };
            }

            let _ = bot.send_message(msg.chat.id, message).await;
            Ok(())
        }
         _=> Ok(())
    }

}

