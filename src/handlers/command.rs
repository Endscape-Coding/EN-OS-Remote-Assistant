use std::io;
use std::process::Command;
use teloxide::prelude::*;
use crate::Botcommand;

pub async fn cmd(bot: Bot, msg: Message, command: Botcommand) -> io::Result<()> {
    match command {
        Botcommand::Cmd(args) => {
            if args.trim().is_empty() {
                bot.send_message(msg.chat.id, "Введите команду после cmd! \nПример: <pre>/cmd firefox</pre>")
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .await;
                return Ok(());
            }

            let command = Command::new("sh").arg("-c").arg(&args).output()?;
            log::info!("Exec command {}", &args);

            let message = match command.status.success() {
                true => String::from_utf8_lossy(&command.stdout).to_string(),
                false => String::from_utf8_lossy(&command.stderr).to_string()
            };

            bot.send_message(msg.chat.id, message).await;
            Ok(())
        }
        _ => Ok(())
    }

}

pub async fn sudo(bot: Bot, msg: Message, command: Botcommand) -> io::Result<()> {
    match command {
        Botcommand::Sudo(args) => {
            bot.send_message(msg.chat.id, format!("SUDO, {}", args)).await;
            Ok(())
        }
        _ => Ok(())
    }
}

