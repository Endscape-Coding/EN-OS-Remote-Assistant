use std::io;
use teloxide::prelude::*;
use teloxide::types::{KeyboardButton, KeyboardMarkup};
use tokio::process::Command;
use tokio::time::{Duration, timeout};
use crate::Botcommand;
use crate::handlers::{config_read, data};

//Command - справочник по командам.
pub async fn command(bot: Bot, msg: Message) -> io::Result<()> {
    log::info!("Command: command");

    let config = match config_read() {
        Ok(config) => config,
        Err(e) => {
            log::error!("Config read failed: {}", e);
            let _ = bot.send_message(msg.chat.id, "Config error").await;
            return Ok(());
        }
    };

    let message = if config.lang == "ru" {
        data::COMMANDRU
    } else {
        data::COMMANDEN
    };

    let buttons = vec![vec![KeyboardButton::new("/start")]];

    let keyboard = KeyboardMarkup::new(buttons).resize_keyboard();

    let _ = bot
        .send_message(msg.chat.id, message)
        .parse_mode(teloxide::types::ParseMode::Html)
        .reply_markup(keyboard)
        .await;

    Ok(())
}

//Cmd spawn, то есть не выдает output, зато бот не лагает после выполнения.
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
                let message = if config.lang == "ru" {
                    data::CMDHELPRU
                } else {
                    data::CMDHELPEN
                };

                let _ = bot
                    .send_message(msg.chat.id, message)
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .await;
                return Ok(());
            }

            let _ = Command::new("sh").arg("-c").arg(&args).spawn()?;

            let message = if config.lang == "ru" {
                data::CMDSPAWNRU
            } else {
                data::CMDSPAWNEN
            };

            log::info!("Exec command {}", &args);

            let _ = bot
                .send_message(msg.chat.id, format!("{} {}", message, &args))
                .await;
            Ok(())
        }
        _ => Ok(()),
    }
}

//Cmd output: Ждет завершения программы или таймаута.
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

            let timeout_secs = Duration::from_secs(config.cmd_timeout);

            if args.trim().is_empty() {
                let message = if config.lang == "ru" {
                    data::CMDHELPRU
                } else {
                    data::CMDHELPEN
                };

                let _ = bot
                    .send_message(msg.chat.id, message)
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .await;
                return Ok(());
            }
            if args.trim().chars().last() == Some('&') {
                let message = "Dont exec command with &, bot can dont responce!".to_string();
                let _ = bot
                    .send_message(msg.chat.id, message)
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .await;
                return Ok(());
            }

            let message = if config.lang == "ru" {
                format!(
                    "{} <code>{}</code> \n {}{}. Бот не будет отвечать, пока команда не выполнится или таймаут не завершится",
                    data::CMDSPAWNRU,
                    args,
                    data::CMDTIMERU,
                    config.cmd_timeout
                )
            } else {
                format!(
                    "{}: <code>{}</code> \n {}{}",
                    data::CMDSPAWNEN,
                    args,
                    data::CMDTIMEEN,
                    config.cmd_timeout
                )
            };

            let _ = bot
                .send_message(msg.chat.id, message)
                .parse_mode(teloxide::types::ParseMode::Html)
                .await;

            let command = timeout(
                timeout_secs,
                Command::new("sh").arg("-c").arg(&args).output(),
            )
            .await;
            log::info!("Exec command (with output) {}", &args);

            let output = match command {
                Ok(Ok(out)) => out,
                Ok(Err(e)) => {
                    log::error!("Exec error: {}", e);
                    let _ = bot.send_message(msg.chat.id, "Exec error: {e}").await;
                    return Ok(());
                }
                Err(_) => {
                    log::warn!("Command timeout: {}", args);
                    let message = if config.lang == "ru" {
                        format!("{}\n<code>{}</code>", data::CMDTIMEOUTRU, args)
                    } else {
                        format!("{}\n<code>{}</code>", data::CMDTIMEOUTEN, args)
                    };
                    let _ = bot
                        .send_message(msg.chat.id, message)
                        .parse_mode(teloxide::types::ParseMode::Html)
                        .await;
                    return Ok(());
                }
            };

            let message: String;
            if String::from_utf8_lossy(&output.stdout).to_string().len() == 0 {
                message = if config.lang == "ru" {
                    data::CMDNOOUTRU.to_string()
                } else {
                    data::CMDNOOUTEN.to_string()
                };
            } else {
                match output.status.success() {
                    true => {
                        message = if config.lang == "ru" {
                            format!(
                                "{}:\n<code>{}</code>",
                                data::CMDOUTRU,
                                String::from_utf8_lossy(&output.stdout).to_string()
                            )
                        } else {
                            format!(
                                "{}:\n<code>{}</code>",
                                data::CMDOUTEN,
                                String::from_utf8_lossy(&output.stdout).to_string()
                            )
                        };
                    }
                    false => {
                        message = if config.lang == "ru" {
                            format!(
                                "{}:\n<code>{}</code>",
                                data::CMDOUTERRRU,
                                String::from_utf8_lossy(&output.stdout).to_string()
                            )
                        } else {
                            format!(
                                "{}:\n<code>{}</code>",
                                data::CMDOUTERREN,
                                String::from_utf8_lossy(&output.stdout).to_string()
                            )
                        };
                    }
                };
            }

            let _ = bot
                .send_message(msg.chat.id, message)
                .parse_mode(teloxide::types::ParseMode::Html)
                .await;
            Ok(())
        }
        _ => Ok(()),
    }
}
