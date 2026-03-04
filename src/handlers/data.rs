pub const ONRU: &str = r#"
<b>EN-OS Remote Assistant v0.3</b>
✅ Система готова к работе!
Используйте /start или /help для получения справки.
"#;

pub const ONEN: &str = r#"
<b>EN-OS Remote Assistant v0.3</b>
✅ System is ready!
Use /start or /help to get assistance.
"#;

pub const STARTRU: &str = r#"
<b>EN-OS Remote Assistant v0.3</b>

<i>📋 Доступные команды:</i>

• /start | /help - <i>Показать эту справку</i>
• /setlang - <i>Выбрать язык интерфейса</i>
• /cd <code>папка</code> - <i>Перейти в директорию</i>
• /ls - <i>Просмотреть содержимое папки</i>
• /download <code>файл</code> - <i>Скачать файл (до 20 МБ)</i>
• /cmd <code>&lt;команда&gt;</code> - <i>Выполнить bash-команду</i>
• /cmd_output <code>&lt;команда&gt;</code> - <i>Выполнить команду с выводом (бот может не отвечать до завершения)</i>
• /screenshot - <i>Сделать скриншот экрана</i>
• /input - <i>Эмуляция ввода с клавиатуры</i>

────────────────────
<i>🟢 Система готова к работе</i>
"#;

pub const STARTEN: &str = r#"
<b>EN-OS Remote Assistant v0.3</b>

<i>📋 Available commands:</i>

• /start | /help - <i>Show this help message</i>
• /setlang - <i>Change interface language</i>
• /cd <code>folder</code> - <i>Change current directory</i>
• /ls - <i>List directory contents</i>
• /download <code>filename</code> - <i>Download a file (up to 20 MB)</i>
• /cmd <code>&lt;command&gt;</code> - <i>Execute a bash command</i>
• /cmd_output <code>&lt;command&gt;</code> - <i>Run command with output (bot may not respond until completion)</i>
• /screenshot - <i>Capture and send screenshot</i>
• /input - <i>Simulate keyboard input</i>

────────────────────
<i>🟢 System is ready</i>
"#;

// ───────── Screenshot ─────────
pub const SCREENRU: &str = r#"
📸 Ваш скриншот:
"#;

pub const SCREENEN: &str = r#"
📸 Your screenshot:
"#;

pub const SCREENRUERR: &str = r#"
❌ Не удалось сделать скриншот:
"#;

pub const SCREENENERR: &str = r#"
❌ Failed to capture screenshot:
"#;

// ───────── CMD ─────────
pub const CMDHELPRU: &str = r#"
⚠️ Укажите команду после /cmd!
<pre>Пример: /cmd firefox</pre>
"#;

pub const CMDHELPEN: &str = r#"
⚠️ Please specify a command after /cmd!
<pre>Example: /cmd firefox</pre>
"#;

pub const CMDSPAWNRU: &str = r#"
🚀 Запуск:"#;

pub const CMDSPAWNEN: &str = r#"
🚀 Executing:"#;

pub const CMDNOOUTRU: &str = r#"
💤 Команда выполнена, но вывода нет.
"#;

pub const CMDNOOUTEN: &str = r#"
💤 Command completed, but no output was produced.
"#;

// ───────── CD ─────────
pub const CDRU: &str = r#"
📁 Переход в:"#;

pub const CDEN: &str = r#"
📁 Now in:"#;

pub const CDERRU: &str = r#"
❌ Директория не найдена
"#;

pub const CDEREN: &str = r#"
❌ Directory not found
"#;

// ───────── LS ─────────
pub const LSRU: &str = r#"
📂 Содержимое директории:"#;

pub const LSEN: &str = r#"
📂 Directory contents:"#;

// ───────── Download ─────────
pub const DLMSRU: &str = r#"
⚠️ Размер файла не должен превышать 20 МБ!
"#;

pub const DLMSEN: &str = r#"
⚠️ File size must not exceed 20 MB!
"#;

pub const DLNFRU: &str = r#"
❌ Файл не найден
"#;

pub const DLNFEN: &str = r#"
❌ File not found
"#;

// ───────── Input ─────────
pub const INMENURU: &str = r#"
<b>⌨️ Меню ввода</b>
<i>Нажмите на кнопку, чтобы эмулировать нажатие клавиши или комбинации</i>
"#;

pub const INMENUEN: &str = r#"
<b>⌨️ Input Menu</b>
<i>Tap a button to simulate a keypress or hotkey combination</i>
"#;

pub const INERRRU: &str = r#"
<b>❌ Ydtool установлен некорректно!</b>
<i>Инструкция по настройке скоро появится</i>
"#;

pub const INERREN: &str = r#"
<b>❌ Ydtool is not configured correctly!</b>
<i>Setup guide coming soon</i>
"#;

pub const INEXECRU: &str = r#"
⌨️ Эмуляция:"#;

pub const INEXECEN: &str = r#"
⌨️ Simulating:"#;
