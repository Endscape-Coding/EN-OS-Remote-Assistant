pub const ONRU: &str = r#"
<b>EN-OS Remote Assistant v0.2</b>
Система готова к работе!
/start или /help для справки.
"#;

pub const ONEN: &str = r#"
<b>EN-OS Remote Assistant v0.2</b>
Ready to work!
/start or /help to help
"#;

pub const STARTRU: &str = r#"
<b>EN-OS Remote Assistant v0.3</b>

<i>Доступные команды:</i>

• /start | /help - <i>Вызвать справку</i>
• /setlang - <i>Выбрать язык</i>
• /cd <code>folder</code> - <i>Перейти в папку</i>
• /ls — <i>Просмотреть содержимое папкик</i>
• /download <code>имя файла</code> - <i>Загрузить файл (до 20 МБ)</i>
• /cmd <code>&lt;команда&gt;</code> - <i>Запустить bash-команду</i>
• /screenshot - <i>Отправить скриншот</i>
• /input - <i>Эмуляция ввода</i>

────────────────────
<i>Система готова к работе</i>
"#;

pub const STARTEN: &str = r#"
<b>EN-OS Remote Assistant v0.3</b>

<i>Available commands:</i>

• /start | /help — <i>Show help</i>
• /setlang — <i>Select language</i>
• /cd <code>folder</code> — <i>Change directory</i>
• /ls — <i>List directory contents</i>
• /download <code>filename</code> — <i>Download file (up to 20 MB)</i>
• /cmd <code>&lt;command&gt;</code> — <i>Run bash command</i>
• /screenshot — <i>Send screenshot</i>
• /input - <i>Keyboard emulation</i>

────────────────────
<i>System ready</i>
"#;

//Screenshot
pub const SCREENRU: &str = r#"
Ваш скриншот
"#;

pub const SCREENEN: &str = r#"
Your screenshot
"#;

pub const SCREENRUERR: &str = r#"
Ошибка в создании скриншота:
"#;

pub const SCREENENERR: &str = r#"
Error to capture screenshot:
"#;

//cmd
pub const CMDHELPRU: &str = r#"
Введите команду после cmd! <pre>Пример: /cmd firefox</pre>
"#;

pub const CMDHELPEN: &str = r#"
Enter the command after cmd! <pre>For example: /cmd firefox</pre>
"#;

pub const CMDNOOUTRU: &str = r#"
Но вывода не последовало...
"#;

pub const CMDNOOUTEN: &str = r#"
But there was no conclusion...
"#;

//cd
pub const CDRU: &str = r#"
Перешли в"#;

pub const CDEN: &str = r#"
Change to"#;

pub const CDERRU: &str = r#"
Директория не найдена"#;

pub const CDEREN: &str = r#"
Directory not found"#;

//ls
pub const LSRU: &str = r#"
Содержимое директории"#;

pub const LSEN: &str = r#"
Directory"#;

//Download
pub const DLMSRU: &str = r#"
Размер файла должен быть меньше 20 мегабайт!"#;

pub const DLMSEN: &str = r#"
File size must have lost 20MB"#;

pub const DLNFRU: &str = r#"
Файл не найден"#;

pub const DLNFEN: &str = r#"
File not found"#;

//Input
pub const INMENURU: &str = r#"
<b>Меню ввода</b>
<i>Нажмите на кнопку клавиатуры для эмуляции указанной комбинации клавиш</i>
"#;

pub const INMENUEN: &str = r#"
<b>Input menu</b>
<i>Click to claviature for emulate hotkey</i>
"#;

pub const INERRRU: &str = r#"
<b>Пожалуйста, установите Ydtool корректно!</b>
Инструкция (будет)
"#;

pub const INERREN: &str = r#"
<b>Please install Ydtool correctly!</b>
Instructions (will be)
"#;

pub const INEXECRU: &str = r#"
Эмуляция клавиш:"#;

pub const INEXECEN: &str = r#"
Emulate hotkey:"#;
