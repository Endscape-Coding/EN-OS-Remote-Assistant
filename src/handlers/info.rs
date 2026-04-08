//!
//! Info - информация о системе и программе.  
//! 
//! Выдает:  
//! 1: Назвавние системы  
//! 2: Версия системы  
//! 3: Загрузка процессора  
//! 4: Использование опетративной памяти  
//! 5: Версия программы по пакету cargo  
//! 6: Коммит сборки  
//! 7: Дата сборки  
//! 8: Режим запуска (debug, release)  
//!   
//! Пример команды: ```/info```  
//! 
use std::io;
use teloxide::prelude::*;
use sysinfo::{System};
use crate::handlers::{data, get_config, other::send};
use shadow_rs::shadow;

shadow!(build);

/// Info - Информация о программе и системе
pub async fn info(bot: Bot, msg: Message) -> io::Result<()> {
    log::info!("Command: info");

    let config = match get_config(bot.clone(), msg.chat.id).await {
        Some(c) => c,
        None => return Ok(()),
    };

    //Инициализация информации о системных компонентах
    let mut sys = System::new_all();
    sys.refresh_all();

    let message = if config.lang == "ru" {
       format!(
            "{header}\n\
            🖥 <b>Система:</b>\n\
            ├ Имя: <code>{os_name}</code>\n\
            └ Версия: <code>{os_ver}</code>\n\n\
            📈 <b>Ресурсы:</b>\n\
            ├ ЦП: <code>{cpu:.2}%</code>\n\
            └ ОЗУ: <code>{ram} МБ</code>\n\n\
            🛠 <b>Сборка:</b>\n\
            ├ Версия (по пакету): <code>{pkg_ver}</code>\n\
            ├ Коммит: <code>{commit}</code>\n\
            ├ Дата сборки: <code>{build_date}</code>\n\
            └ Режим: <code>{debug}</code>\n\n\
            👨‍💻 <b>Разработчик:</b> <a href='https://github.com/Endscape-Coding'>Endscape</a>\n\
            📢 <b>Канал:</b> @Linux_EN_OS\n\n\
            <i>Сделано с любовью для пользователей EN-OS ❤️</i>",
            header = data::INFORU,
            os_name = System::name().unwrap_or_else(|| "Unknown".into()),
            os_ver = System::os_version().unwrap_or_else(|| "N/A".into()),
            cpu = sys.global_cpu_usage(),
            ram = sys.used_memory() / 1024 / 1024,
            pkg_ver = build::PKG_VERSION,
            commit = build::SHORT_COMMIT,
            build_date = build::BUILD_TIME,
            debug = if shadow_rs::is_debug() { "🪲 Debug" } else { "🚀 Release" }
        )
    } else {
        format!(
            "{header}\n\
            🖥 <b>System:</b>\n\
            ├ OS: <code>{os_name}</code>\n\
            └ Version: <code>{os_ver}</code>\n\n\
            📈 <b>Resources:</b>\n\
            ├ CPU: <code>{cpu:.2}%</code>\n\
            └ RAM: <code>{ram} MB</code>\n\n\
            🛠 <b>Build Info:</b>\n\
            ├ Version: <code>{pkg_ver}</code>\n\
            ├ Commit: <code>{commit}</code>\n\
            ├ Built at: <code>{build_date}</code>\n\
            └ Mode: <code>{debug}</code>\n\n\
            👨‍💻 <b>Developer:</b> <a href='https://github.com/Endscape-Coding'>Endscape</a>\n\
            📢 <b>Channel:</b> @Linux_EN_OS\n\n\
            <i>Made with love for EN-OS users ❤️</i>",
            header = data::INFOEN,
            os_name = System::name().unwrap_or_else(|| "Unknown".into()),
            os_ver = System::os_version().unwrap_or_else(|| "N/A".into()),
            cpu = sys.global_cpu_usage(),
            ram = sys.used_memory() / 1024 / 1024,
            pkg_ver = build::PKG_VERSION,
            commit = build::SHORT_COMMIT,
            build_date = build::BUILD_TIME,
            debug = if shadow_rs::is_debug() { "🪲 Debug" } else { "🚀 Release" }
        )
    };

    send(&bot, &msg, &message).await;

    Ok(())
}
