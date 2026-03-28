//!
//! Lang - работа с языком
//! 
use std::io;
use teloxide::prelude::*;
use teloxide::types::{CallbackQuery, InlineKeyboardButton, InlineKeyboardMarkup};
use crate::handlers::config_read;
use crate::handlers::config_write;

fn to_io_err<E: std::fmt::Display>(e: E) -> io::Error {
    io::Error::new(io::ErrorKind::Other, e.to_string())
}

/// Выбор языка через кнопки.
/// Пример: ```/setlang```. Далее выбираете нужный язык.
pub async fn setlang(bot: Bot, msg: Message) -> io::Result<()> {
    let buttons = [[
        InlineKeyboardButton::callback("🇷🇺 Русский", "ru"),
        InlineKeyboardButton::callback("🇺🇸 English", "en"),
    ]];

    let keyboard = InlineKeyboardMarkup::new(buttons);

    bot.send_message(msg.chat.id, "Выберите язык / Select language:")
        .reply_markup(keyboard)
        .await
        .map_err(to_io_err)?;

    Ok(())
}

/// Обращение к бэкенду
pub async fn setlang_callback(bot: Bot, q: CallbackQuery) -> io::Result<()> {
    if let Some(data) = q.data {
        bot.answer_callback_query(q.id).await.map_err(to_io_err)?;

        if let Some(msg) = q.message {
            match data.as_str() {
                "ru" => {
                    set_lang_backend("ru").await?;
                    bot.send_message(msg.chat().id, "Выбран русский язык!")
                        .await
                        .map_err(to_io_err)?;
                }
                "en" => {
                    set_lang_backend("en").await?;
                    bot.send_message(msg.chat().id, "Language has been changed!")
                        .await
                        .map_err(to_io_err)?;
                }
                _ => (),
            }
        }
    }
    Ok(())
}

///
/// Сам бэкенд, просто записывает код языка в конфиг.
/// 
async fn set_lang_backend(lang: &str) -> io::Result<()> {
    if let Ok(mut config) = config_read() {
        config.lang = String::from(lang);
        let _ = config_write(config);

        log::info!("Lang changed: {}", lang)
    }
    Ok(())
}
