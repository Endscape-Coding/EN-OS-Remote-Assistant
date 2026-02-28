use std::fs;
use std::io;
use std::io::Read;
use std::io::Write;
use std::env;
use std::path::Path;
use std::fs::File;
use chrono::Local;
use teloxide::prelude::*;
use teloxide::types::InputFile;
use walkdir::WalkDir;
use zip::write::SimpleFileOptions;
use crate::Botcommand;
use crate::handlers::data;
use crate::handlers::config_read;

pub async fn cd(bot: Bot, msg: Message, command: Botcommand) -> io::Result<()> {
    match command {
        Botcommand::Cd(args) => {
            log::info!("Read config");
            let config = config_read();
            let new_path = Path::new(&args);
            let message: String;

            if env::set_current_dir(&new_path).is_ok() {
                let current = env::current_dir().expect("Не удалось распознать текущую папку! / Couldn't recognize the current folder!");
                if config.unwrap().lang == "ru" {
                    message = format!("{}: <code>{}</code>",data::CDRU,  current.display());
                } else {
                    message = format!("{}: <code>{}</code>",data::CDEN,  current.display());
                }
                let _ = bot.send_message(msg.chat.id, message)
                .parse_mode(teloxide::types::ParseMode::Html)
                .await;
            } else {
                if config.unwrap().lang == "ru" {
                    message = format!("{}: <code>{}</code>",data::CDERRU, args);
                } else {
                    message = format!("{}: <code>{}</code>",data::CDEREN, args);
                }

            let _ = bot.send_message(msg.chat.id, message)
                .parse_mode(teloxide::types::ParseMode::Html)
                .await;
            }
            Ok(())
        }
        _ => Ok(())
    }
}

pub async fn ls(bot: Bot, msg: Message) -> io::Result<()> {
    log::info!("Read config");
    let config = config_read();
    log::info!("Give a current_dir");
    let current = env::current_dir().expect("Не удалось распознать текущую папку! / Couldn't recognize the current folder!");
    let mut message: String;

    if config.unwrap().lang == "ru" {
        message = String::from(data::LSRU);
    } else {
        message = String::from(data::LSEN);
    }

    log::info!("Read dir");
    let paths = fs::read_dir("./").expect("Не удалось прочитать директорию! / Couldn't read the directory");

    let names: String = paths
    .filter_map(|entry| entry.ok())
    .map(|entry| {
        let name = entry.file_name().to_string_lossy().into_owned();
        format!("<code>{}</code>", name)
    })
    .collect::<Vec<String>>()
    .join("\n");

    message = format!("{} \n <i>{}</i>: \n{}", message, current.display(), names);

    let _ = bot.send_message(msg.chat.id, message)
    .parse_mode(teloxide::types::ParseMode::Html)
    .await;

    Ok(())
}

pub async fn download(bot: Bot, msg: Message, command: Botcommand) -> io::Result<()> {
    match command {
        Botcommand::Download(args) => {
            log::info!("Read config");
            let config = config_read();
            let path = Path::new(&args);
            let message: String;

            if path.exists() {
                log::info!("File path exists");
                match fs::metadata(path) {
                    Ok(meta) => {
                        let smsg = bot.send_message(msg.chat.id, "Uploading..").await;
                        let bot_clone = bot.clone();
                        let chat_id = smsg.clone().unwrap().chat.id;
                        let msg_id = smsg.clone().unwrap().id;

                        let animation_task = tokio::spawn(async move {
                            let frames = ["Uploading... 🕛", "Uploading..  🕑","Uploading.   🕓",  "Uploading    🕕",  "Uploading.   🕗",  "Uploading..  🕙"];
                            let mut i = 0;
                            loop {
                                let _ = bot_clone
                                .edit_message_text(chat_id, msg_id, frames[i % frames.len()])
                                .await;
                                i += 1;
                                tokio::time::sleep(std::time::Duration::from_millis(600)).await;
                            }
                        });

                        if path.is_dir() {
                            let size = get_dir_size(path);
                            let sizemb = size / 1024 / 1024;

                            if sizemb > 20 {
                                if config.unwrap().lang == "ru" {
                                    message = String::from(data::DLMSRU);
                                } else {
                                    message = String::from(data::DLMSEN);
                                }
                                animation_task.abort();
                                let _ = bot.delete_message(smsg.clone().unwrap().chat.id, smsg.unwrap().id).await;
                                let _ = bot.send_message(msg.chat.id, message)
                                .await;
                            } else {
                                log::info!("It is a directory, pack to zip");
                                let now = Local::now();
                                let time = now.format("%Y-%m-%d %H.%M.%S").to_string();

                                let zip_name = format!("{} {}.zip", args, time);
                                let _ = zip_dir(&args, &zip_name);

                                message = "Zip file:".to_string();
                                let _ = bot.send_document(msg.chat.id, InputFile::file(&zip_name))
                                .caption(message)
                                .parse_mode(teloxide::types::ParseMode::Html)
                                .await;

                                animation_task.abort();
                                let _ = bot.delete_message(smsg.clone().unwrap().chat.id, smsg.unwrap().id).await;
                                let _ = fs::remove_file(&zip_name);
                            }

                        } else {
                            log::info!("Metadata exists and avaiable");
                            let size = meta.len() / 1024 / 1024;
                            log::info!("File size: {}", size);
                            if size > 20 {
                                if config.unwrap().lang == "ru" {
                                    message = String::from(data::DLMSRU);
                                } else {
                                    message = String::from(data::DLMSEN);
                                }

                                let _ = bot.send_message(msg.chat.id, message)
                                .await;
                            } else {
                                let _ = bot.send_document(msg.chat.id, InputFile::file(path))
                                .caption(format!("<code>{}</code>", args))
                                .parse_mode(teloxide::types::ParseMode::Html)
                                .await;

                                animation_task.abort();
                                let _ = bot.delete_message(smsg.clone().unwrap().chat.id, smsg.unwrap().id).await;
                            }
                        }
                        return Ok(())
                    }
                    Err(e) => eprintln!("Ошибка: {}", e),
                }
            } else {
                if config.unwrap().lang == "ru" {
                    message = String::from(data::DLNFRU);
                } else {
                    message = String::from(data::DLNFEN);
                }
                let _ = bot.send_message(msg.chat.id, message)
                .parse_mode(teloxide::types::ParseMode::Html)
                .await;

            }
            Ok(())
        }
        _ => Ok(())
    }
}

fn zip_dir(src_dir: &str, dst_file: &str) -> zip::result::ZipResult<()> {
    let path = Path::new(src_dir);
    let file = File::create(dst_file)?;
    let mut zip = zip::ZipWriter::new(file);
    let options = SimpleFileOptions::default()
    .compression_method(zip::CompressionMethod::Deflated);

    let walk = WalkDir::new(path);

    for entry in walk.into_iter().filter_map(|e| e.ok()) {
        let entry_path = entry.path();
        let name = entry_path.strip_prefix(path).unwrap();

        if entry_path.is_file() {
            zip.start_file(name.to_string_lossy(), options)?;
            let mut f = File::open(entry_path)?;
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer)?;
            zip.write(&buffer)?;
        } else if !name.as_os_str().is_empty() {
            zip.add_directory(name.to_string_lossy(), options)?;
        }
    }
    zip.finish()?;
    Ok(())
}

fn get_dir_size<P: AsRef<Path>>(path: P) -> u64 {
    WalkDir::new(path)
    .into_iter()
    .filter_map(|entry| entry.ok())
    .filter_map(|entry| entry.metadata().ok())
    .filter(|metadata| metadata.is_file())
    .map(|metadata| metadata.len())
    .sum()
}

