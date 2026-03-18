use std::io;
use tokio::process::Command;
use teloxide::prelude::*;
use crate::handlers::data;
use crate::handlers::config_read;

//Работа с питанием. В целом ничего сложного нет, код вполне себе читаемый, а так же считаю эту функцию самой идиоматичной во всем моем проекте, т.к. нет ни сложных вложенных конструкций ни прочей херни.
pub async fn powerman(bot: Bot, msg: Message, num: u64) -> io::Result<()> {
    log::info!(
        "Command powerman, num: {}. Reference: 0 - main menu, 1 - shutdown, 2 - reboot, 3 - sleep, 4 - hybernate",
        num
    );
    let config = match config_read() {
        Ok(config) => config,
        Err(e) => {
            log::error!("Config read failed: {}", e);
            let _ = bot.send_message(msg.chat.id, "Config error").await;
            return Ok(());
        }
    };

    let message = match num {
        0 => {
            let message = if config.lang == "ru" {
                data::POWERMANRU
            } else {
                data::POWERMANEN
            };
            let _ = bot
                .send_message(msg.chat.id, message)
                .parse_mode(teloxide::types::ParseMode::Html)
                .await;
            return Ok(());
        }
        1 => {
            if config.lang == "ru" {
                data::SHUTDOWNRU
            } else {
                data::SHUTDOWNEN
            }
        }
        2 => {
            if config.lang == "ru" {
                data::REBOOTRU
            } else {
                data::REBOOTEN
            }
        }
        3 => {
            if config.lang == "ru" {
                data::SLEEPRU
            } else {
                data::SLEEPEN
            }
        }
        4 => {
            if config.lang == "ru" {
                data::SUPSRU
            } else {
                data::SUPSEN
            }
        }
        _ => "Invalid input",
    };

    let _ = bot.send_message(msg.chat.id, message).await;

    let cmd: &str = match num {
        1 => {
            "dbus-send --system --print-reply --dest=org.freedesktop.login1 /org/freedesktop/login1 org.freedesktop.login1.Manager.PowerOff boolean:true"
        }
        2 => {
            "dbus-send --system --print-reply --dest=org.freedesktop.login1 /org/freedesktop/login1 org.freedesktop.login1.Manager.Reboot boolean:true"
        }
        3 => {
            "dbus-send --system --print-reply --dest=org.freedesktop.login1 /org/freedesktop/login1 org.freedesktop.login1.Manager.Suspend boolean:true"
        }
        4 => {
            "dbus-send --system --print-reply --dest=org.freedesktop.login1 /org/freedesktop/login1 org.freedesktop.login1.Manager.Hibernate boolean:true"
        }
        _ => {
            let _ = bot.send_message(msg.chat.id, "Invalid input... How?").await;
            return Ok(());
        }
    };

    let cmd_status = Command::new("sh").arg("-c").arg(cmd).output().await?;

    if !cmd_status.status.success() {
        let _ = bot
            .send_message(
                msg.chat.id,
                format!(
                    "<b>Error shutdown your PC!</b> \nError: <i>{}</i>",
                    String::from_utf8_lossy(&cmd_status.stderr)
                ),
            )
            .parse_mode(teloxide::types::ParseMode::Html)
            .await;
    }

    Ok(())
}
