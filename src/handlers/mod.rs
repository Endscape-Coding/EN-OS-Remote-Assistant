//Тута все переводы
pub mod data;

//Работа с коммандами (файл command.rs)
pub mod command;
pub use command::{cmd, cmd_output, command};

//Стартовая команда (файл start.rs)
pub mod start;
pub use start::start;

//Работа с конфигами (Файл config.rs)
pub mod config;
pub use config::{config_read, config_write};

//Скриншоты (Файл screen.rs)
pub mod screen;
pub use screen::screen;

//Работа с языком (Файл lang.rs)
pub mod lang;
pub use lang::{setlang, setlang_callback};

//Работы с файлами (Файл files.rs)
pub mod files;
pub use files::{cd, download, filemanager, ls, rm, upload};

//Работа с эмуляцией ввода (Файл input.rs)
pub mod input;
pub use input::input;

//Работа с питанием (Файл power.rs)
pub mod power;
pub use power::powerman;

pub mod link;
pub use link::openlink;

//Другое...
pub mod other;
pub use other::check_prog;
