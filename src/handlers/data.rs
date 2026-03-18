// ───────── Startup / Online ─────────
pub const LIMIT: usize = 4096;
pub const ONRU: &str = r#"
<b>EN-OS Remote Assistant v0.4</b>

✅ Всё готово к работе!
Нажмите /start или /help, чтобы увидеть доступные команды.
"#;

pub const ONEN: &str = r#"
<b>EN-OS Remote Assistant v0.4</b>

✅ System is ready to go!
Use /start or /help to view available commands.
"#;

// ───────── Help / Start ─────────
pub const STARTRU: &str = r#"
<b>EN-OS Remote Assistant v0.4</b>
<i>Удалённое управление вашей системой через Telegram</i>

📋 <b>Доступные команды:</b>

• /start | /help — показать эту справку
• /setlang — изменить язык интерфейса
• /filemanager — управление файлами
• /command — меню команд
• /openlink <code>&lt;ссылка&gt;</code> — открыть ссылку в браузере
• /screenshot — сделать и отправить скриншот
• /input — эмуляция ввода с клавиатуры
• /powerman — управление питанием

────────────────────
🟢 <i>Система активна и ожидает команд</i>
"#;

pub const STARTEN: &str = r#"
<b>EN-OS Remote Assistant v0.3</b>
<i>Remote system control via Telegram</i>

📋 <b>Available commands:</b>

• /start | /help — show this help message
• /setlang — change interface language
• /filemanager — file management
• /command — command menu
• /openlink <code>&lt;link&gt;</code> — open a link in browser
• /screenshot — capture and send a screenshot
• /input — simulate keyboard input
• /powerman — power management

────────────────────
🟢 <i>System is active and ready for commands</i>
"#;

// ───────── Screenshot ─────────
pub const SCREENRU: &str = r#"📸 Вот ваш скриншот:"#;
pub const SCREENEN: &str = r#"📸 Here's your screenshot:"#;

pub const SCREENRUERR: &str =
    r#"❌ Не удалось сделать скриншот. Попробуйте ещё раз или проверьте настройки:"#;
pub const SCREENENERR: &str =
    r#"❌ Couldn't capture screenshot. Please try again or check your settings:"#;

// ───────── CMD ─────────
pub const CMDHELPRU: &str = r#"
⚠️ Пожалуйста, укажите команду после /cmd

<pre>Пример: /cmd ls -la</pre>
"#;

pub const CMDHELPEN: &str = r#"
⚠️ Please specify a command after /cmd

<pre>Example: /cmd ls -la</pre>
"#;

pub const COMMANDRU: &str = r#"Меню команд
/cmd - запуск команды
/cmd_output - запуск команды с парсингом вывода.
"#;
pub const COMMANDEN: &str = r#"Command menu
/cmd <args> - exec command
/cmd_output <args> - exec command with parce output.
"#;

pub const CMDSPAWNRU: &str = r#"🚀 Выполняю:"#;
pub const CMDSPAWNEN: &str = r#"🚀 Executing:"#;

pub const CMDNOOUTRU: &str =
    r#"💤 Команда выполнена. Вывод отсутствует — это нормально для некоторых команд."#;
pub const CMDNOOUTEN: &str =
    r#"💤 Command completed. No output was generated — this is expected for some commands."#;

pub const CMDTIMERU: &str = r#"Таймаут (секунд): "#;
pub const CMDTIMEEN: &str = r#"Timeout (seconds): "#;

pub const CMDTIMEOUTRU: &str = r#"Время вышло! Выполнялась команда:"#;
pub const CMDTIMEOUTEN: &str = r#"Time is out! Exec command:"#;

pub const CMDOUTRU: &str = r#"Команда успешно выполнена! Вывод:"#;
pub const CMDOUTEN: &str = r#"Command exec sucessfully! Output:"#;

pub const CMDOUTERRRU: &str = r#"Команда выполнилась в ошибкой! Вывод:"#;
pub const CMDOUTERREN: &str = r#"Command exec with error! Output:"#;

// ───────── File Manager ─────────
pub const FILEMANRU: &str = r#"
<b>📁 Файловый менеджер</b>

• /cd <code>&lt;путь&gt;</code> — перейти в директорию
• /ls <code>&lt;путь&gt;</code> — показать содержимое
• /download <code>&lt;путь&gt;</code> — скачать файл или папку
• /rm <code>&lt;путь&gt;</code> — удалить файл или папку

💡 <i>Поддерживаются абсолютные и относительные пути</i>
"#;

pub const FILEMANEN: &str = r#"
<b>📁 File Manager</b>

• /cd <code>&lt;path&gt;</code> — change directory
• /ls <code>&lt;path&gt;</code> — list directory contents
• /download <code>&lt;path&gt;</code> — download a file or folder
• /rm <code>&lt;path&gt;</code> — delete a file or folder

💡 <i>Both absolute and relative paths are supported</i>
"#;

// ───────── CD ─────────
pub const CDNOARGSRU: &str = r#"
💡 Укажите путь после /cd

<pre>Пример: /cd /home/user/Documents</pre>
Поддерживаются как абсолютные, так и относительные пути.
"#;

pub const CDNOARGSEN: &str = r#"
💡 Please specify a path after /cd

<pre>Example: /cd /home/user/Documents</pre>
Both absolute and relative paths are supported.
"#;

pub const CDRU: &str = r#"📁 Перешёл в:"#;
pub const CDEN: &str = r#"📁 Now in:"#;

pub const CDERRU: &str = r#"❌ Директория не найдена. Проверьте путь и попробуйте снова."#;
pub const CDEREN: &str = r#"❌ Directory not found. Please check the path and try again."#;

// ───────── LS ─────────
pub const LSRU: &str = r#"📂 Содержимое:"#;
pub const LSEN: &str = r#"📂 Contents:"#;

// ───────── RM (Remove) ─────────
pub const RMNOARGSRU: &str = r#"
💡 Укажите путь к файлу или папке после /rm

<pre>Пример: /rm /tmp/old_file.txt</pre>
"#;

pub const RMNOARGSEN: &str = r#"
💡 Please specify a path after /rm

<pre>Example: /rm /tmp/old_file.txt</pre>
"#;

pub const RMRU: &str = r#"🗑️ Удаляю:"#;
pub const RMEN: &str = r#"🗑️ Removing:"#;

pub const RMSUCRU: &str = r#"✅ Удалено успешно!"#;
pub const RMSUCEN: &str = r#"✅ Successfully removed!"#;

// ───────── Download ─────────
pub const DLMSRU: &str = r#"⚠️ Файл слишком большой. Максимальный размер для отправки — 20 МБ."#;
pub const DLMSEN: &str = r#"⚠️ File is too large. Maximum size for sending is 20 MB."#;

pub const DLNFRU: &str = r#"❌ Файл не найден. Проверьте путь и попробуйте снова."#;
pub const DLNFEN: &str = r#"❌ File not found. Please check the path and try again."#;

//Upload
pub const UPSUCRU: &str = r#"Сохранено в: "#;
pub const UPSUCEN: &str = r#"Saved to: "#;

// ───────── Input Menu ─────────
pub const INMENURU: &str = r#"
<b>⌨️ Меню ввода</b>
<i>Нажмите на кнопку, чтобы эмулировать нажатие клавиши или комбинации</i>
"#;

pub const INMENUEN: &str = r#"
<b>⌨️ Input Menu</b>
<i>Tap a button to simulate a keypress or hotkey combination</i>
"#;

pub const INERRRU: &str = r#"
❌ <b>Ydtool настроен некорректно</b>
<i>Руководство по настройке скоро появится. Пока что проверьте установку утилиты.</i>
"#;

pub const INERREN: &str = r#"
❌ <b>Ydtool is not configured correctly</b>
<i>Setup guide coming soon. For now, please verify the utility installation.</i>
"#;

pub const INEXECRU: &str = r#"⌨️ Эмулирую:"#;
pub const INEXECEN: &str = r#"⌨️ Simulating:"#;

// ───────── Power Manager ─────────
pub const POWERMANRU: &str = r#"
<b>🔋 Менеджер питания</b>

• /shutdown — выключить компьютер
• /reboot — перезагрузить систему
• /sleep — перевести в спящий режим
• /hibernate — уйти в гибернацию

⚠️ <i>Спящий режим и гибернация могут работать не на всех устройствах</i>
"#;

pub const POWERMANEN: &str = r#"
<b>🔋 Power Manager</b>

• /shutdown — power off the system
• /reboot — restart the system
• /sleep — enter sleep mode
• /hibernate — enter hibernation

⚠️ <i>Sleep and hibernation may not be supported on all devices</i>
"#;

pub const SHUTDOWNRU: &str = r#"🔌 Выключаю систему..."#;
pub const SHUTDOWNEN: &str = r#"🔌 Shutting down..."#;

pub const REBOOTRU: &str = r#"🔄 Перезагружаю..."#;
pub const REBOOTEN: &str = r#"🔄 Rebooting..."#;

pub const SLEEPRU: &str = r#"😴 Перехожу в спящий режим..."#;
pub const SLEEPEN: &str = r#"😴 Entering sleep mode..."#;

pub const SUPSRU: &str = r#"💤 Ухожу в гибернацию..."#;
pub const SUPSEN: &str = r#"💤 Entering hibernation..."#;

// ───────── Open Link ─────────
pub const LINKHELPRU: &str = r#"
💡 Отправьте ссылку в формате:

<code>/openlink https://en-os.ru</code>
"#;

pub const LINKHELPEN: &str = r#"
💡 Send a link in this format:

<code>/openlink https://en-os.ru</code>
"#;

pub const OPENLINKRU: &str = r#"🌐 Открываю ссылку..."#;
pub const OPENLINKEN: &str = r#"🌐 Opening your link..."#;
