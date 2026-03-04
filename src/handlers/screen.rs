use url::Url;
use std::io;
use std::fs;
use std::env;
use std::path::PathBuf;
use std::path::Path;
use std::process::Command;
use chrono::Local;
use teloxide::prelude::*;
use teloxide::types::InputFile;
use crate::handlers;
use crate::handlers::data;
use crate::handlers::config_read;
use ashpd::desktop::screenshot::Screenshot;


pub async fn screen(bot: Bot, msg: Message) -> io::Result<()> {
    log::info!("Command: Screen");

    let config = match config_read() {
        Ok(config) => config,
        Err(e) => {
            log::error!("Config read failed: {}", e);
            let _ = bot.send_message(msg.chat.id, "Config error").await;
            return Ok(());
        }
    };

    let result: io::Result<PathBuf> = if mbwayland() {
        match screenshot().await {
            Ok(path) => Ok(path),
            Err(e) => Err(e)
        }
    } else {
        match screenshot_x11().await {
            Ok(path) => Ok(path),
            Err(e) => Err(e)
        }
    };

    match result {
        Ok(path) => {
            let smsg = match bot.send_message(msg.chat.id, "Wait").await {
                Ok(message) => message,
                Err(e) => {
                    log::error!("Error send message {}", e);
                    return Ok(())
                }
            };
            let bot_clone = bot.clone();
            let chat_id = smsg.chat.id;
            let msg_id = smsg.id;

            let animation_task = tokio::spawn(async move {
                let frames = ["Wait... 🕛", "Wait... 🕑","Wait... 🕓",  "Wait... 🕕",  "Wait... 🕗",  "Wait... 🕙"];
                let mut i = 0;
                loop {
                    let _ = bot_clone
                    .edit_message_text(chat_id, msg_id, frames[i % frames.len()])
                    .await;
                    i += 1;
                    tokio::time::sleep(std::time::Duration::from_millis(600)).await;
                }
            });

            log::info!("Screen saved to: {}", path.display());
            let photo = InputFile::file(&path);

            let message = if config.lang == "ru" { data::SCREENRU } else { data::SCREENEN };

            let _ = bot.send_photo(msg.chat.id, photo).caption(message).await;
            animation_task.abort();

            let _ = match fs::remove_file(path) {
                Ok(()) => log::info!("Remove screenshot file succesfully!"),
                Err(e) => log::info!("Failed delete screenshot file: {}", e)
            };

            let _ = bot.delete_message(chat_id, msg_id).await;
        }
        Err(e) => {
            log::error!("Screenshot failed: {}", e);
            let message = if config.lang == "ru" { data::SCREENRUERR } else { data::SCREENENERR };
            let _ = bot.send_message(msg.chat.id, format!("{}: {}", message, e)).await;
            return Ok(());
        }
    }

    Ok(())
}

async fn screenshot() -> io::Result<PathBuf> {
    let response = Screenshot::request()
    .interactive(false)
    .modal(false)
    .send()
    .await
    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?
    .response()
    .map_err(|e| std::io::Error::new(std::io::ErrorKind::PermissionDenied, e))?;

    let uri = response.uri();

    let url = Url::parse(uri.as_str())
    .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?;

    let path = url.to_file_path()
    .map_err(|_| std::io::Error::new(std::io::ErrorKind::NotFound, "Not a file URI"))?;
    log::info!("Path: {}", path.display());

    Ok(path)
}

async fn screenshot_x11() -> io::Result<PathBuf> {
    if !handlers::check_prog("scrot").await{
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Scrot not found!"));
    }
    let now = Local::now();
    let time = now.format("%Y-%m-%d %H.%M.%S").to_string();
    let screen = format!("screen{}.png", time);

    Command::new("scrot")
    .arg(&screen)
    .status()?;

    let path = Path::new(&screen);
    Ok(path.to_path_buf())
}

fn mbwayland() -> bool {
    let output = env::var("XDG_SESSION_TYPE").unwrap_or("x11".to_string());

    match &*output {
        "wayland" => true,
        "x11" => false,
        _ => false,
    }
}
