use std::io;
use std::process::Command;
use crate::Botcommand;
use crate::handlers::data;
use crate::handlers::config_read;
use teloxide::prelude::*;
use teloxide::types::{KeyboardButton, KeyboardMarkup};

pub async fn input(bot: Bot, msg: Message, command: Botcommand, ydt: bool) -> io::Result<()> {
    match command {
        Botcommand::Input(args) => {
            let mut message;
            let config = config_read();
            if !ydt {
                if config.unwrap().lang == "ru" {
                    message = String::from(data::INERRRU);
                } else {
                    message = String::from(data::INERREN);
                }

                let _ = bot.send_message(msg.chat.id, message)
                .parse_mode(teloxide::types::ParseMode::Html)
                .await;

                return Err(io::Error::new(io::ErrorKind::PermissionDenied, "Ydotool not installed correctly!"));
            }
            if args.trim().is_empty() {
                if config.unwrap().lang == "ru" {
                    message = String::from(data::INMENURU);
                } else {
                    message = String::from(data::INMENUEN);
                }

                let buttons = vec![
                    vec![
                        KeyboardButton::new("/start"),
                    ],
                    vec![
                        KeyboardButton::new("/input volup"),
                        KeyboardButton::new("/input voldown"),
                        KeyboardButton::new("/input mute"),
                    ],
                    vec![
                        KeyboardButton::new("/input alt+tab"),
                        KeyboardButton::new("/input alt+f4"),
                        KeyboardButton::new("/input ctrl+alt+delete"),
                    ],
                    vec![
                        KeyboardButton::new("/input up"),
                        KeyboardButton::new("/input down"),
                        KeyboardButton::new("/input left"),
                        KeyboardButton::new("/input right"),
                    ],
                    vec![
                        KeyboardButton::new("/input ctrl+c"),
                        KeyboardButton::new("/input ctrl+v"),
                        KeyboardButton::new("/input ctrl+x"),
                    ],
                    vec![
                        KeyboardButton::new("/input super"),
                        KeyboardButton::new("/input super+l"),
                        KeyboardButton::new("/input super+d"),
                    ],
                    vec![
                        KeyboardButton::new("/input enter"),
                        KeyboardButton::new("/input escape"),
                        KeyboardButton::new("/input backspace"),
                    ],
                    vec![
                        KeyboardButton::new("/input ctrl+alt+t"),
                    ],
                ];

                let keyboard = KeyboardMarkup::new(buttons)
                .resize_keyboard();

                let _ = bot.send_message(msg.chat.id, message)
                .parse_mode(teloxide::types::ParseMode::Html)
                .reply_markup(keyboard)
                .await;

                return Ok(());
            } else {
                if config.unwrap().lang == "ru" {
                    message = format!("{} <code>{}</code>", String::from(data::INEXECRU), args);
                } else {
                    message = format!("{} <code>{}</code>", String::from(data::INEXECEN), args);
                }
                let _ = bot.send_message(msg.chat.id, &message)
                .parse_mode(teloxide::types::ParseMode::Html)
                .await;

                match exec_key(&args.trim()) {
                    Ok(_) => {},
                    Err(e) => match e.kind() {
                        io::ErrorKind::InvalidInput => {
                            message = format!("{} <code>{}</code>", "ERROR", "Invalid Input");
                            log::error!("Invalid input");
                            let _ = bot.send_message(msg.chat.id, &message)
                            .parse_mode(teloxide::types::ParseMode::Html)
                            .await;
                        }
                        io::ErrorKind::NotFound => {
                            message = format!("{} <code>{}</code> {}", "ERROR", "Ydotool not found!", "Please, install ydotool!");
                            log::error!("Invalid input");
                            let _ = bot.send_message(msg.chat.id, &message)
                            .parse_mode(teloxide::types::ParseMode::Html)
                            .await;
                        }
                        _ => {
                            println!("Другая ошибка: {:?}", e);
                            message = format!("{} {} <code>{}</code>", "ERROR", "Ydotool error: ", e);
                            log::error!("Unknown error");
                            let _ = bot.send_message(msg.chat.id, &message)
                            .parse_mode(teloxide::types::ParseMode::Html)
                            .await;
                        }
                    },
                }
            }

            Ok(())
        }
        _ => Ok(())
    }

}

fn exec_key(key: &str) -> io::Result<&str> {
    log::info!("Hotkey: {}", key);

    match check_ydt() {
        true => {},
        false => return Err(io::Error::new(io::ErrorKind::NotFound, "Ydotool not found!")),
    }

    match key {
        //One press
        "escape" => press(1)?,
        "enter" => press(28)?,
        "super" => press(125)?,
        "backspace" => press(14)?,
        "volup" => press(115)?,
        "voldown" => press(114)?,
        "mute" => press(113)?,
        "play/pause" => press(164)?,
        "up" => press(103)?,
        "down" => press(108)?,
        "left" => press(105)?,
        "right" => press(106)?,
        //Two combo
        "ctrl+c" => combotwo(29, 46)?,
        "ctrl+v" => combotwo(29, 47)?,
        "ctrl+a" => combotwo(29, 30)?,
        "ctrl+x" => combotwo(29, 45)?,
        "alt+tab" => combotwo(56, 15)?,
        "super+l" => combotwo(125, 38)?,
        "super+d" => combotwo(125, 32)?,
        "alt+f4" => combotwo(56, 62)?,
        //Three combo
        "ctrl+alt+t" => combothree(29, 56, 20)?,
        "ctrl+alt+delete" => combothree(29, 56, 111)?,
        "shift+ctrl+p" => combothree(54, 29, 25)?,

        _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Неверная комбинация клавиш!")),
    }
    Ok("Success")
}

fn check_ydt() -> bool {
    let cmd = Command::new("which")
    .arg("ydotool")
    .output();

    if cmd.expect("Error..?").status.success() {
        log::info!("Ydotool has been installed!");
        return true;
    } else {
        log::error!("Ydotool not installed!");
        return false;
    }
}

fn press(code: u16) -> io::Result<()> {
    let command = format!("ydotool key {}:1 {}:0", code, code);
    log::info!("Ydotool command: {}", command);
    let cmd = Command::new("sh")
    .args(["-c", &command])
    .output()?;

    if cmd.status.success() {
        Ok(())
    } else {
        let errmsg = String::from_utf8_lossy(&cmd.stdout);
        Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Ydotool failed: {}", errmsg.trim())
            ))
    }
}

fn combotwo(key1: u16, key2: u16) -> io::Result<()> {
    let command = format!("ydotool key {}:1 {}:1 {}:0 {}:0", key1, key2, key2, key1);
    log::info!("Ydotool command: {}", command);
    let cmd = Command::new("sh")
    .args(["-c", &command])
    .output()?;

    if cmd.status.success() {
        Ok(())
    } else {
        let errmsg = String::from_utf8_lossy(&cmd.stdout);
        Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Ydotool failed: {}", errmsg.trim())
            ))
    }
}

fn combothree(key1: u16, key2: u16, key3: u16) -> io::Result<()> {
    let command = format!("ydotool key {}:1 {}:1 {}:1 {}:0 {}:0 {}:0", key1, key2, key3, key3, key2, key1);
    log::info!("Ydotool command: {}", command);
    let cmd = Command::new("sh")
    .args(["-c", &command])
    .output()?;

    if cmd.status.success() {
        Ok(())
    } else {
        let errmsg = String::from_utf8_lossy(&cmd.stdout);
        Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Ydotool failed: {}", errmsg.trim())
            ))
    }
}

