//!
//! Filemanager - работа с файлами  
//! Позволяет удалять, скачиваать, загружать файлы на компьютер, переходить по папкам и просматривать директории.
//! 
use std::env;
use std::io;
use std::io::Read;
use std::io::Write;
use std::io::{Error, ErrorKind};
use std::path::Path;
use teloxide::net::Download;
use teloxide::prelude::*;
use teloxide::types::{ChatAction, InputFile, KeyboardButton, KeyboardMarkup, Message};
use tokio::fs;
use tokio::fs::File;
use chrono::Local;
use walkdir::WalkDir;
use zip::write::SimpleFileOptions;
use crate::Botcommand;
use crate::handlers::other::send;
use crate::handlers::{config_read, get_config, data};

//Работа с файлами. Самое сложное в этом проекте наверное, т.к. кода писать приходится очень много.
/// Центральная функция
pub async fn filemanager(bot: Bot, msg: Message) -> io::Result<()> {
    log::info!("Command: filemanager");

    let config = match get_config(bot.clone(), msg.chat.id).await {
        Some(c) => c,
        None => return Ok(()),
    };
    let current = env::current_dir()?;

    //Думаю в будущем везде так на match перейти
    let message: String = match config.lang.as_str() {
        "ru" | "ua" => format!(
            "{}\n Вы находитесь в директории <code>{}</code>",
            data::FILEMANRU,
            current.display()
        ),
        _ => format!(
            "{}\n You are in the directory: <code>{}</code>",
            data::FILEMANEN,
            current.display()
        ),
    };

    let buttons = vec![
        vec![KeyboardButton::new("/start")],
        vec![KeyboardButton::new("/ls"), KeyboardButton::new("/cd")],
        vec![KeyboardButton::new("/download"), KeyboardButton::new("/rm")],
    ];

    let keyboard = KeyboardMarkup::new(buttons).resize_keyboard();

    let _ = bot
        .send_message(msg.chat.id, message)
        .parse_mode(teloxide::types::ParseMode::Html)
        .reply_markup(keyboard)
        .await;

    Ok(())
}

/// Хождение по директориям (```/cd```).   
/// Пример: ```/cd Documents```.  
/// Переход в родительскую директорию: ```/cd ..```  
pub async fn cd(bot: Bot, msg: Message, command: Botcommand) -> io::Result<()> {
    match command {
        Botcommand::Cd(args) => {
            log::info!("Command: Cd");

            log::info!("Read config");
            let config = match get_config(bot.clone(), msg.chat.id).await {
                Some(c) => c,
                None => return Ok(()),
            };

            if args.trim().is_empty() {
                let message = if config.lang == "ru" {
                    data::CDNOARGSRU
                } else {
                    data::CDNOARGSEN
                };
                log::info!("No args");
                send(&bot, &msg, message).await;
                return Ok(());
            }

            let new_path = Path::new(&args);
            let message: String;

            if env::set_current_dir(&new_path).is_ok() {
                let current = env::current_dir().expect(
                    "Не удалось распознать текущую папку! / Couldn't recognize the current folder!",
                );
                if config.lang == "ru" {
                    message = format!("{}: <code>{}</code>", data::CDRU, current.display());
                } else {
                    message = format!("{}: <code>{}</code>", data::CDEN, current.display());
                }
                send(&bot, &msg, &message).await;
            } else {
                if config.lang == "ru" {
                    message = format!("{}: <code>{}</code>", data::CDERRU, args);
                } else {
                    message = format!("{}: <code>{}</code>", data::CDEREN, args);
                }

                send(&bot, &msg, &message).await;
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

/// Удаление файлов (```/rm```).  
/// Пример команды: ```/rm file.txt```  
pub async fn rm(bot: Bot, msg: Message, command: Botcommand) -> io::Result<()> {
    match command {
        Botcommand::Rm(args) => {
            log::info!("Command: Rm");

            log::info!("Read config");
            let config = match get_config(bot.clone(), msg.chat.id).await {
                Some(c) => c,
                None => return Ok(()),
            };

            if args.trim().is_empty() {
                let message = if config.lang == "ru" {
                    data::RMNOARGSRU
                } else {
                    data::RMNOARGSEN
                };
                log::info!("No args");
                send(&bot, &msg, message).await;
                return Ok(());
            }

            let new_path = Path::new(&args);

            if new_path.exists() {
                let message = if config.lang == "ru" {
                    data::RMRU
                } else {
                    data::RMEN
                };
                let _ = bot.send_message(msg.chat.id, message).await;
                if new_path.is_dir() {
                    log::info!("Remove dir");
                    match fs::remove_dir_all(new_path).await {
                        Ok(..) => {
                            log::info!("Remove dir succesfully!");
                            let message = if config.lang == "ru" {
                                data::RMSUCRU
                            } else {
                                data::RMSUCEN
                            };
                            send(&bot, &msg, message).await;
                        }
                        Err(e) => {
                            log::error!("Error remove dir!, {}", e);
                            send(&bot, &msg, &format!("Cannot remove, error: {}", e)).await;
                        }
                    }
                } else {
                    match fs::remove_file(new_path).await {
                        Ok(..) => {
                            log::info!("File removed succesfully!");
                            let message = if config.lang == "ru" {
                                data::RMSUCRU
                            } else {
                                data::RMSUCEN
                            };
                            send(&bot, &msg, message).await;
                        }
                        Err(e) => {
                            log::error!("Error remove file!, {}", e);
                            bot.send_message(msg.chat.id, format!("Cannot remove, error: {}", e))
                                .await
                                .ok();
                        }
                    }
                }
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

/// Просмотр содержимого текущей директории (```/ls```).
pub async fn ls(bot: Bot, msg: Message) -> io::Result<()> {
    log::info!("Command: ls");
    let config = config_read();
    log::info!("Give a current_dir");
    let current = env::current_dir()?;

    let header = if config.unwrap().lang == "ru" {
        format!("{} \n <i>{}</i>:", data::LSRU, current.display())
    } else {
        format!("{} \n <i>{}</i>:", data::LSEN, current.display())
    };

    log::info!("Reading directory..");
    let paths = std::fs::read_dir("./")?;
    let mut partmessage = header.clone();

    for entry in paths.filter_map(|e| e.ok()) {
        let name = entry.file_name().to_string_lossy().into_owned();
        let line = format!("\n<code>{}</code>", name);
        if partmessage.len() + line.len() > data::LIMIT {
            log::info!("Message > 4096 symbols, cutting...");
            send(&bot, &msg, &partmessage).await;
            partmessage = header.clone();
        }
        partmessage.push_str(&line);
    }

    let _ = bot
        .send_message(msg.chat.id, partmessage)
        .parse_mode(teloxide::types::ParseMode::Html)
        .await;

    Ok(())
}

///
/// Скачивание файлов c компа (```/download```). Пример: ```/download file.txt```.  
/// Максимальный размер скачиваемого файла/директории не более 20 мегабайт  
/// 
// Самая мутарная и сложная функция, так еще куча вложенных конструкций. Но для переправки 20 мб файлов пойдет.
pub async fn download(bot: Bot, msg: Message, command: Botcommand) -> io::Result<()> {
    match command {
        Botcommand::Download(args) => {
            log::info!("Command: download");

            let config = match get_config(bot.clone(), msg.chat.id).await {
                Some(c) => c,
                None => return Ok(()),
            };

            let path = Path::new(&args);
            let message: String;

            if path.exists() {
                log::info!("File path exists");
                match fs::metadata(path).await {
                    Ok(meta) => {
                        let smsg = bot.send_message(msg.chat.id, "Uploading..").await;
                        let bot_clone = bot.clone();
                        let chat_id = smsg.clone().unwrap().chat.id;
                        let msg_id = smsg.clone().unwrap().id;

                        //Онэмашке
                        let animation_task = tokio::spawn(async move {
                            let frames = [
                                "Uploading... 🕛",
                                "Uploading..  🕑",
                                "Uploading.   🕓",
                                "Uploading    🕕",
                                "Uploading.   🕗",
                                "Uploading..  🕙",
                            ];
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
                                let message = if config.lang == "ru" {
                                    data::DLMSRU
                                } else {
                                    data::DLMSEN
                                };

                                animation_task.abort();
                                let _ = bot
                                    .delete_message(smsg.clone().unwrap().chat.id, smsg.unwrap().id)
                                    .await;
                                send(&bot, &msg, message).await;
                            } else {
                                let _ = bot
                                    .send_chat_action(msg.chat.id, ChatAction::UploadDocument)
                                    .await;
                                log::info!("It is a directory, pack to zip");
                                let now = Local::now();
                                let time = now.format("%Y-%m-%d %H.%M.%S").to_string();

                                let zip_name = format!("{} {}.zip", args, time);
                                let _ = zip_dir(&args, &zip_name);

                                message = "Zip file:".to_string();
                                let _ = bot
                                    .send_document(msg.chat.id, InputFile::file(&zip_name))
                                    .caption(message)
                                    .parse_mode(teloxide::types::ParseMode::Html)
                                    .await;

                                animation_task.abort();
                                let _ = bot
                                    .delete_message(smsg.clone().unwrap().chat.id, smsg.unwrap().id)
                                    .await;

                                if let Err(e) = fs::remove_file(&zip_name).await {
                                    log::warn!(
                                        "Failed to remove temp zip file {}: {}",
                                        zip_name,
                                        e
                                    );
                                }
                            }
                        } else {
                            log::info!("Metadata exists and avaiable");
                            let size = meta.len() / 1024 / 1024;
                            log::info!("File size: {}", size);
                            if size > 20 {
                                let message = if config.lang == "ru" {
                                    data::DLMSRU
                                } else {
                                    data::DLMSEN
                                };

                                send(&bot, &msg, message).await;
                            } else {
                                let _ = bot
                                    .send_chat_action(msg.chat.id, ChatAction::UploadDocument)
                                    .await;
                                let _ = bot
                                    .send_document(msg.chat.id, InputFile::file(path))
                                    .caption(format!("<code>{}</code>", args))
                                    .parse_mode(teloxide::types::ParseMode::Html)
                                    .await;

                                animation_task.abort();
                                let _ = bot
                                    .delete_message(smsg.clone().unwrap().chat.id, smsg.unwrap().id)
                                    .await;
                            }
                        }
                        return Ok(());
                    }
                    Err(e) => eprintln!("Ошибка: {}", e),
                }
            } else {
                let message = if config.lang == "ru" {
                    data::DLNFRU
                } else {
                    data::DLNFEN
                };

                send(&bot, &msg, message).await;
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

//Единственная функция писанная ИИшкой, ненавижу сложные циклы и алгоритмы. Из за того, что ИИ, я тчательно протестировал функцию и кое что переработал.
fn zip_dir(src_dir: &str, dst_file: &str) -> zip::result::ZipResult<()> {
    let path = Path::new(src_dir);
    let file = std::fs::File::create(dst_file)?;
    let mut zip = zip::ZipWriter::new(file);
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    let walk = WalkDir::new(path);

    for entry in walk.into_iter().filter_map(|e| e.ok()) {
        let entry_path = entry.path();
        let name = entry_path.strip_prefix(path).unwrap();

        if entry_path.is_file() {
            zip.start_file(name.to_string_lossy(), options)?;
            let mut f = std::fs::File::open(entry_path)?;
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

///
/// Загрузка файлов НА пк (просто отправь сообщение как файл)
/// 
pub async fn upload(bot: Bot, msg: Message) -> std::io::Result<()> {
    let config = match get_config(bot.clone(), msg.chat.id).await {
        Some(c) => c,
        None => return Ok(()),
    };

    let doc = msg
        .document()
        .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "Сообщение не содержит документа"))?;

    let smsg = bot.send_message(msg.chat.id, "Uploading..").await;

    let file_info = bot
        .get_file(doc.file.id.clone())
        .await
        .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;

    let file_path = &file_info.path;

    let now = Local::now();
    let time = now.format("%Y-%m-%d_%H.%M.%S").to_string();

    let local_path = format!(
        "./uploads/{}_{}",
        time,
        doc.file_name.as_deref().unwrap_or("unknown")
    );

    if let Some(parent) = Path::new(&local_path).parent() {
        fs::create_dir_all(parent).await?;
    }

    let mut dist = File::create(&local_path).await?;
    bot.download_file(file_path, &mut dist)
        .await
        .map_err(|e| Error::new(ErrorKind::BrokenPipe, e.to_string()))?;

    let _ = bot
        .delete_message(smsg.clone().unwrap().chat.id, smsg.unwrap().id)
        .await;

    let full_path = std::fs::canonicalize(&local_path)
        .unwrap_or_else(|_| std::path::PathBuf::from(&local_path));

    let message = if config.lang == "ru" {
        format!("{}: <code>{}</code>", data::UPSUCRU, full_path.display())
    } else {
        format!("{}: <code>{}</code>", data::UPSUCEN, full_path.display())
    };

    send(&bot, &msg, &message).await;

    Ok(())
}

///
/// Отдает размер директории (для ```download```)
/// 
// Вспомогательная функция, может перенесу в other
fn get_dir_size<P: AsRef<Path>>(path: P) -> u64 {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| entry.metadata().ok())
        .filter(|metadata| metadata.is_file())
        .map(|metadata| metadata.len())
        .sum()
}
