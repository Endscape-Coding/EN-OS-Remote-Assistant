//! 
//! Mod.rs - все испорты и работа с модулями
//! 
/// Тута все переводы
pub mod data;

/// Стартовая команда (файл start.rs)
pub mod start;
pub use start::start;

/// Информация о программе и системе
pub mod info;
pub use info::info;

/// Получение логов
pub mod log;
pub use log::log;

/// Настройки (файл settings.rs)
pub mod settings;
pub use settings::{settings, set_cmd_timeout, set_notify};

/// Работа с коммандами (файл command.rs)
pub mod command;
pub use command::{cmd, cmd_output, command};

/// Работа с конфигами (Файл config.rs)
pub mod config;
pub use config::{config_read, config_write, get_config};

/// Скриншоты (Файл screen.rs)
pub mod screen;
pub use screen::screen;

/// Работа с языком (Файл lang.rs)
pub mod lang;
pub use lang::{setlang, setlang_callback};

/// Работы с файлами (Файл files.rs)
pub mod files;
pub use files::{cd, download, filemanager, ls, rm, upload};

/// Работа с эмуляцией ввода (Файл input.rs)
pub mod input;
pub use input::input;

/// Работа с питанием (Файл power.rs)
pub mod power;
pub use power::powerman;

/// Открытие ссылок
pub mod link;
pub use link::openlink;

/// Работает с прокси
pub mod proxywork;
pub use proxywork::proxy_work;

/// Вспомогательные функции
pub mod other;
pub use other::{check_prog, notify, wait_network};
