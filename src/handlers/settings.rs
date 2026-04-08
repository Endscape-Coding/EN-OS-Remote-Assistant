//!
//! Настройки - работа с конфигом.  
//! Позволяет менять язык, уведомления (включить/выключить) и менять timeout на ```/cmd_output```  
//! 
//! Пример команды: ```/settings```
//! 
use std::io;
use teloxide::prelude::*;
use crate::Botcommand;
use crate::handlers::data;
use crate::handlers::other::send;
use crate::handlers::{config_read, config_write, get_config};

/// Основная функция со справкой (```/settings```)
pub async fn settings(bot: Bot, msg: Message) -> io::Result<()> {
    log::info!("Command: setings");

    let config = match get_config(bot.clone(), msg.chat.id).await {
        Some(c) => c,
        None => return Ok(()),
    };

    let message = if config.lang == "ru" {
        data::SETTINGSRU
    } else {
        data::SETTINGSEN
    };

    send(&bot, &msg, message).await;

    let _ = send_config(bot, msg).await;

    Ok(())
}

/// Отправляет конфиг на данный момент
async fn send_config(bot: Bot, msg: Message) -> io::Result<()> {
    let config = match get_config(bot.clone(), msg.chat.id).await {
        Some(c) => c,
        None => return Ok(()),
    };

    let message: String = match config.lang.as_str() {
        "ru" | "ua" => format!("Конфигурация: \n\nЯзык: {} \nТаймаут команды: {}\nУведомления: {}", config.lang, config.cmd_timeout, config.notify),
        _ => format!("Config: \n\nLanguage: {} \nCmd output timeout: {}\nNotify: {}", config.lang, config.cmd_timeout, config.notify),
    };

    send(&bot, &msg, &message).await;

    Ok(())
}

/// Устанавливает таймаут для команды с выводом
pub async fn set_cmd_timeout(bot: Bot, msg: Message, command: Botcommand) -> io::Result<()> {
    match command {
        Botcommand::SetTimeout(args) => {
            /*
            let config = match get_config(bot.clone(), msg.chat.id).await {
                Some(c) => c,
                None => return Ok(()),
            };
            */

            if let Ok(mut config) = config_read() {
                config.cmd_timeout = args;
                let _ = config_write(config);

                let message = format!("Timeout Changed {}", args);

                send(&bot, &msg, &message).await;

                log::info!("Timeout changed: {}", args)
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

/// Включает или выключает уведомления
pub async fn set_notify(bot: Bot, msg: Message) -> io::Result<()> {
    if let Ok(mut config) = config_read() {
        config.notify = !config.notify;
        
        let status = if config.notify { "Enabled" } else { "Disabled" };
        let _ = config_write(config);
        let message = format!("Notify Changed: {}", status);

        send(&bot, &msg, &message).await;
        log::info!("Notify changed")
    }
    Ok(())
}