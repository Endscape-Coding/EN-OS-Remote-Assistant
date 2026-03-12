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
            log::info!("Command: Input");

            let mut message;
            let config = match config_read() {
                Ok(config) => config,
                Err(e) => {
                    log::error!("Config read failed: {}", e);
                    let _ = bot.send_message(msg.chat.id, "Config error").await;
                    return Ok(());
                }
            };

            if !ydt {
                let message = if config.lang == "ru" { data::INERRRU } else { data::INERREN };

                let _ = bot.send_message(msg.chat.id, message)
                .parse_mode(teloxide::types::ParseMode::Html)
                .await;

                return Err(io::Error::new(io::ErrorKind::PermissionDenied, "Ydotool not installed correctly!"));
            }
            if args.trim().is_empty() {
                let message = if config.lang == "ru" { data::INMENURU } else { data::INMENUEN };
                let buttons = vec![
                vec![
                    KeyboardButton::new("/input up"),
                    KeyboardButton::new("/input down"),
                    KeyboardButton::new("/input left"),
                    KeyboardButton::new("/input right"),
                ],
                vec![
                    KeyboardButton::new("/input enter"),
                    KeyboardButton::new("/input backspace"),
                    KeyboardButton::new("/input escape"),
                    KeyboardButton::new("/input tab"),
                ],
                vec![
                    KeyboardButton::new("/input home"),
                    KeyboardButton::new("/input end"),
                    KeyboardButton::new("/input pageup"),
                    KeyboardButton::new("/input pagedown"),
                ],
                vec![
                    KeyboardButton::new("/input delete"),
                    KeyboardButton::new("/input insert"),
                    KeyboardButton::new("/input space"),
                ],
                vec![
                    KeyboardButton::new("/input volup"),
                    KeyboardButton::new("/input voldown"),
                    KeyboardButton::new("/input mute"),
                    KeyboardButton::new("/input play/pause"),
                ],
                vec![
                    KeyboardButton::new("/input ctrl+c"),
                    KeyboardButton::new("/input ctrl+v"),
                    KeyboardButton::new("/input ctrl+x"),
                    KeyboardButton::new("/input ctrl+z"),
                ],
                vec![
                    KeyboardButton::new("/input ctrl+a"),
                    KeyboardButton::new("/input ctrl+f"),
                    KeyboardButton::new("/input ctrl+t"),
                    KeyboardButton::new("/input ctrl+w"),
                ],
                vec![
                    KeyboardButton::new("/input ctrl+s"),
                    KeyboardButton::new("/input ctrl+r"),
                    KeyboardButton::new("/input ctrl+n"),
                    KeyboardButton::new("/input ctrl+enter"),
                ],
                vec![
                    KeyboardButton::new("/input alt+tab"),
                    KeyboardButton::new("/input alt+f4"),
                    KeyboardButton::new("/input alt+enter"),
                ],
                vec![
                    KeyboardButton::new("/input super"),
                    KeyboardButton::new("/input super+l"),
                    KeyboardButton::new("/input super+d"),
                    KeyboardButton::new("/input super+e"),
                ],
                vec![
                    KeyboardButton::new("/input super+r"),
                    KeyboardButton::new("/input super+t"),
                    KeyboardButton::new("/input super+a"),
                ],
                vec![
                    KeyboardButton::new("/input ctrl+alt+t"),
                    KeyboardButton::new("/input ctrl+alt+delete"),
                    KeyboardButton::new("/input ctrl+shift+esc"),
                ],
                vec![
                    KeyboardButton::new("/input ctrl+shift+c"),
                    KeyboardButton::new("/input ctrl+shift+v"),
                    KeyboardButton::new("/input shift+ctrl+p"),
                ],
                vec![
                    KeyboardButton::new("/input q"), KeyboardButton::new("/input w"),
                    KeyboardButton::new("/input e"), KeyboardButton::new("/input r"),
                ],
                vec![
                    KeyboardButton::new("/input t"), KeyboardButton::new("/input y"),
                    KeyboardButton::new("/input u"), KeyboardButton::new("/input i"),
                ],
                vec![
                    KeyboardButton::new("/input o"), KeyboardButton::new("/input p"),
                ],
                vec![
                    KeyboardButton::new("/input a"), KeyboardButton::new("/input s"),
                    KeyboardButton::new("/input d"), KeyboardButton::new("/input f"),
                ],
                vec![
                    KeyboardButton::new("/input g"), KeyboardButton::new("/input h"),
                    KeyboardButton::new("/input j"), KeyboardButton::new("/input k"),
                ],
                vec![
                    KeyboardButton::new("/input l"),
                ],
                vec![
                    KeyboardButton::new("/input z"), KeyboardButton::new("/input x"),
                    KeyboardButton::new("/input c"), KeyboardButton::new("/input v"),
                ],
                vec![
                    KeyboardButton::new("/input b"), KeyboardButton::new("/input n"),
                    KeyboardButton::new("/input m"),
                ],
                vec![
                    KeyboardButton::new("/input й"), KeyboardButton::new("/input ц"),
                    KeyboardButton::new("/input у"), KeyboardButton::new("/input к"),
                ],
                vec![
                    KeyboardButton::new("/input е"), KeyboardButton::new("/input н"),
                    KeyboardButton::new("/input г"), KeyboardButton::new("/input ш"),
                ],
                vec![
                    KeyboardButton::new("/input щ"), KeyboardButton::new("/input з"),
                    KeyboardButton::new("/input х"), KeyboardButton::new("/input ъ"),
                ],
                vec![
                    KeyboardButton::new("/input ф"), KeyboardButton::new("/input ы"),
                    KeyboardButton::new("/input в"), KeyboardButton::new("/input а"),
                ],
                vec![
                    KeyboardButton::new("/input п"), KeyboardButton::new("/input р"),
                    KeyboardButton::new("/input о"), KeyboardButton::new("/input л"),
                ],
                vec![
                    KeyboardButton::new("/input д"), KeyboardButton::new("/input ж"),
                    KeyboardButton::new("/input э"),
                ],
                vec![
                    KeyboardButton::new("/input я"), KeyboardButton::new("/input ч"),
                    KeyboardButton::new("/input с"), KeyboardButton::new("/input м"),
                ],
                vec![
                    KeyboardButton::new("/input и"), KeyboardButton::new("/input т"),
                    KeyboardButton::new("/input ь"), KeyboardButton::new("/input ю"),
                ],
                vec![
                    KeyboardButton::new("/input ё"),
                ],
                vec![
                    KeyboardButton::new("/input 1"), KeyboardButton::new("/input 2"),
                    KeyboardButton::new("/input 3"), KeyboardButton::new("/input 4"),
                    KeyboardButton::new("/input 5"),
                ],
                vec![
                    KeyboardButton::new("/input 6"), KeyboardButton::new("/input 7"),
                    KeyboardButton::new("/input 8"), KeyboardButton::new("/input 9"),
                    KeyboardButton::new("/input 0"),
                ],
                vec![
                    KeyboardButton::new("/input -"), KeyboardButton::new("/input ="),
                    KeyboardButton::new("/input ["), KeyboardButton::new("/input ]"),
                ],
                vec![
                    KeyboardButton::new("/input ;"), KeyboardButton::new("/input '"),
                    KeyboardButton::new("/input ,"), KeyboardButton::new("/input ."),
                    KeyboardButton::new("/input /"),
                ],
                vec![
                    KeyboardButton::new("/input \\"), KeyboardButton::new("/input `"),
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
                if config.lang == "ru" {
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
        "escape" => press(1)?,
        "enter" => press(28)?,
        "super" => press(125)?,
        "backspace" => press(14)?,
        "tab" => press(15)?,
        "space" => press(57)?,
        "volup" => press(115)?,
        "voldown" => press(114)?,
        "mute" => press(113)?,
        "play/pause" => press(164)?,
        "up" => press(103)?,
        "down" => press(108)?,
        "left" => press(105)?,
        "right" => press(106)?,
        "delete" => press(111)?,
        "insert" => press(110)?,
        "home" => press(102)?,
        "end" => press(107)?,
        "pageup" => press(104)?,
        "pagedown" => press(109)?,

        "a" => press(30)?, "b" => press(48)?, "c" => press(46)?, "d" => press(32)?,
        "e" => press(18)?, "f" => press(33)?, "g" => press(34)?, "h" => press(35)?,
        "i" => press(23)?, "j" => press(36)?, "k" => press(37)?, "l" => press(38)?,
        "m" => press(50)?, "n" => press(49)?, "o" => press(24)?, "p" => press(25)?,
        "q" => press(16)?, "r" => press(19)?, "s" => press(31)?, "t" => press(20)?,
        "u" => press(22)?, "v" => press(47)?, "w" => press(17)?, "x" => press(45)?,
        "y" => press(21)?, "z" => press(44)?,

        "ф" => press(30)?, "и" => press(48)?, "в" => press(46)?, "а" => press(32)?,
        "е" => press(18)?, "у" => press(33)?, "к" => press(34)?, "ш" => press(35)?,
        "з" => press(23)?, "х" => press(36)?, "ъ" => press(37)?, "л" => press(38)?,
        "м" => press(50)?, "ь" => press(49)?, "щ" => press(24)?, "п" => press(25)?,
        "р" => press(16)?, "о" => press(19)?, "с" => press(31)?, "т" => press(20)?,
        "ж" => press(22)?, "ц" => press(47)?, "й" => press(17)?, "ч" => press(45)?,
        "н" => press(21)?, "я" => press(44)?, "ю" => press(26)?, "ё" => press(41)?,
        "э" => press(12)?,

        "0" => press(11)?, "1" => press(2)?, "2" => press(3)?, "3" => press(4)?,
        "4" => press(5)?, "5" => press(6)?, "6" => press(7)?, "7" => press(8)?,
        "8" => press(9)?, "9" => press(10)?,
        "-" => press(12)?, "=" => press(13)?,
        "[" => press(26)?, "]" => press(27)?,
        ";" => press(39)?, "'" => press(40)?,
        "," => press(51)?, "." => press(52)?, "/" => press(53)?,
        "\\" => press(43)?, "`" => press(41)?,

        "ctrl+c" => combotwo(29, 46)?, "ctrl+v" => combotwo(29, 47)?,
        "ctrl+a" => combotwo(29, 30)?, "ctrl+x" => combotwo(29, 45)?,
        "ctrl+z" => combotwo(29, 44)?, "ctrl+y" => combotwo(29, 21)?,
        "ctrl+s" => combotwo(29, 31)?, "ctrl+w" => combotwo(29, 17)?,
        "ctrl+f" => combotwo(29, 33)?, "ctrl+r" => combotwo(29, 19)?,
        "ctrl+t" => combotwo(29, 20)?, "ctrl+n" => combotwo(29, 49)?,
        "ctrl+o" => combotwo(29, 24)?, "ctrl+p" => combotwo(29, 25)?,
        "ctrl+l" => combotwo(29, 38)?, "ctrl+k" => combotwo(29, 37)?,
        "ctrl+enter" => combotwo(29, 28)?,

        "alt+tab" => combotwo(56, 15)?, "alt+f4" => combotwo(56, 62)?,
        "alt+enter" => combotwo(56, 28)?, "alt+space" => combotwo(56, 57)?,

        "shift+tab" => combotwo(42, 15)?, "shift+enter" => combotwo(42, 28)?,

        "super+l" => combotwo(125, 38)?, "super+d" => combotwo(125, 32)?,
        "super+e" => combotwo(125, 18)?, "super+r" => combotwo(125, 19)?,
        "super+t" => combotwo(125, 20)?, "super+a" => combotwo(125, 30)?,

        "ctrl+alt+t" => combothree(29, 56, 20)?,
        "ctrl+alt+delete" => combothree(29, 56, 111)?,
        "shift+ctrl+p" => combothree(54, 29, 25)?,
        "ctrl+shift+esc" => combothree(29, 42, 1)?,
        "ctrl+alt+l" => combothree(29, 56, 38)?,
        "ctrl+alt+a" => combothree(29, 56, 30)?,
        "ctrl+shift+c" => combothree(29, 42, 46)?,
        "ctrl+shift+v" => combothree(29, 42, 47)?,
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

