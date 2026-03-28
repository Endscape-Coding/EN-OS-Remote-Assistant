//!
//! Other - вспомогательные функции.  
//! Здесь собраны функции, которые используются несколькими модулями сразу.
//! 
use std::{io, result};
use std::process::Command;
use teloxide::{Bot, prelude::*};
use tokio::time::{Duration};
use notify_rust::{Notification, Timeout};

//Вспомогательные функции...
pub async fn send(bot: &Bot, msg: &Message, text: &str) {
    let result = bot
        .send_message(msg.chat.id, text)
        .parse_mode(teloxide::types::ParseMode::Html)
        .await;

    match result {
        Ok(_) => log::info!("Send message"),
        Err(e) => log::error!("Error send message! Error: {}", e),
    }
}

/// Проверяет наличие программы через which.
pub async fn check_prog(name: &str) -> bool {
    let cmd = Command::new("which").arg(name).output();

    if cmd.expect("Error..?").status.success() {
        log::info!("{} has been installed!", name);
        return true;
    } else {
        log::error!("{} not installed!", name);
        return false;
    }
}

/// Отправляет уведомление
pub async fn notify(sum: &str, body: &str, icon: &str, time: u32) -> io::Result<()> {
    log::info!("Create notify: Summary: {sum}, body: {body}, icon:{icon}");
    let result = Notification::new()
        .summary(&sum)
        .body(&body)
        .icon(&icon)
        .timeout(Timeout::Milliseconds(time))
        .show_async()
        .await;

    match result {
        Ok(_) => log::info!("Notify send!"),
        Err(e) => log::error!("Error send notify! Error: {}", e),
    }

    Ok(())
}

/// Ожидаение интернета
// По логике кончено это должно быть с main, но хз, мне так понравилось.
pub async fn wait_network(bot: &Bot) {
    let mut error = false; 
    loop {
        match bot.get_my_name().await {
            Ok(_) => {
                if error {
                    let _ = notify("Internet connection restored", "Remote assistant work correctly", "task-complete", 7500).await;
                }
                log::info!("Connected to Telegram!");
                break;
            }
            Err(e) => {
                if !error {
                    let _ = notify("Telegram API is unavailable!", "Check EN-OS Remote Assistant logs!", "dialog-warning", 7500).await;
                }
                log::warn!("Telegram API is unavailable, retrying in 5 seconds... (Error: {})", e);
                error = true;
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        }
    }
}
