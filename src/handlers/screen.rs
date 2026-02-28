use url::Url;
use std::io;
use std::fs;
use std::path::PathBuf;
use teloxide::prelude::*;
use teloxide::types::InputFile;
use crate::handlers::data;
use crate::handlers::config_read;
use ashpd::desktop::screenshot::Screenshot;


pub async fn screen(bot: Bot, msg: Message) -> io::Result<()> {
    let message: String;
    let config = config_read();
    let screen = screenshot().await;

    match screen {
        Ok(p) => {
            let smsg = bot.send_message(msg.chat.id, "Wait").await;
            let bot_clone = bot.clone();
            let chat_id = smsg.clone().unwrap().chat.id;
            let msg_id = smsg.clone().unwrap().id;

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

            log::info!("Screen saved to: {}", p.display());
            let photo = InputFile::file(&p);

            if config.unwrap().lang == "ru" {
                message = String::from(data::SCREENRU);
            } else {
                message = String::from(data::SCREENEN);
            }

            let _ = bot.send_photo(msg.chat.id, photo).caption(message).await;
            animation_task.abort();

            let _ = match fs::remove_file(p) {
                Ok(()) => log::info!("Remove screenshot file succesfully!"),
                Err(e) => log::info!("Failed delete screenshot file: {}", e)
            };

            let _ = bot.delete_message(smsg.clone().unwrap().chat.id, smsg.unwrap().id).await;
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            if config.unwrap().lang == "ru" {
                message = String::from(data::SCREENRUERR);
            } else {
                message = String::from(data::SCREENENERR);
            }

            let _ = bot.send_message(msg.chat.id, format!("<b>{}</b><code>{}</code>", message, e))
            .parse_mode(teloxide::types::ParseMode::Html)
            .await;
        }
    }
    Ok(())
}

async fn screenshot() -> ashpd::Result<PathBuf> {
    let response = Screenshot::request()
    .interactive(false)
    .modal(false)
    .send()
    .await.expect("Error")
    .response()?;

    let uri = response.uri();

    let url = Url::parse(uri.as_str())
    .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?;

    let path = url.to_file_path()
    .map_err(|_| std::io::Error::new(std::io::ErrorKind::NotFound, "Not a file URI"))?;
    log::info!("Path: {}", path.display());

    Ok(path)
}
