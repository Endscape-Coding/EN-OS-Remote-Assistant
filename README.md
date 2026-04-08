# 🖥️ EN-OS-Remote-Assistant

[![Version](https://img.shields.io/badge/version-0.6-blue.svg)](https://github.com/yourusername/EN-OS-Remote-Assistant)
[![Rust](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-AGPL-green.svg)](LICENSE)

Control your PC remotely via Telegram. A lightweight, Rust-based assistant that allows you to execute commands, manage files, and take screenshots directly from your messenger.

> ⚠️ **SECURITY WARNING**  
> This application grants **full control** over your computer to anyone who has access to your Telegram bot. 
> - Ensure your `TELOXIDE_TOKEN` is kept secret.
> - The `ID` restriction is your primary security layer. **Do not share your bot with others.**
> - Do not run this on a machine containing sensitive data unless you trust the environment.

---

## ✨ Features

- **🚀 Command Execution:** Run system commands directly from Telegram (non-sudo).
- **📸 Screenshots:** Capture and view your desktop instantly.
- **📂 File Management:** Browse, view, and download files or folders (max size: 20MB).
- **🌐 Display Server Support:** Compatible with both **Wayland** and **X11**.
- **🌍 Bilingual:** Interface available in **English** and **Russian**.
- **⌨️ Input emulate:** Emulate keyboard input via **ydotool**.
- **🦀 Written in Rust:** Fast, memory-safe, and reliable.

---

## 🛠️ Prerequisites

Before building, ensure you have the following installed:
- [Rust & Cargo](https://rustup.rs/)
- [Ydotool](https://github.com/ReimuNotMoe/ydotool)
- [Scrot](https://github.com/resurrecting-open-source-projects/scrot) (if you use x11)
- A Telegram Bot Token (get it from [@BotFather](https://t.me/BotFather))
- Your Telegram Chat ID (get it from [@userinfobot](https://t.me/userinfobot))

---

## 📦 Installation & Configuration

### 1. Clone the Repository
```bash
git clone https://github.com/yourusername/EN-OS-Remote-Assistant.git
cd EN-OS-Remote-Assistant
```

### 2. Configure Environment Variables
Create a `.env` file in the root directory and add your credentials:

```env
TELOXIDE_TOKEN=YOUR_BOT_TOKEN_HERE
ID=123456789
```

> 🔒 **Tip:** Add `.env` to your `.gitignore` file to prevent accidentally committing secrets to GitHub.

### 3. Build and Run
1: Install ```cargo``` and ```ydotool```  
2: Compile the project using build script: 
```
bash build.sh
```

---

## 📸 Preview

soon
![Preview](./assets/preview.png)

---

## 📄 License

Distributed under the AGPL License. See `LICENSE` for more information.

---

## 📞 Contact

Endscape - [@Linux_EN_OS](https://t.me/Linux_EN_OS) - endscape.coding@gmail.com

Project Link: [https://github.com/yourusername/EN-OS-Remote-Assistant](https://github.com/yourusername/EN-OS-Remote-Assistant)
