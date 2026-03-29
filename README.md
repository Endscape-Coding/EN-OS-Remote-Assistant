# 🖥️ EN-OS-Remote-Assistant

[![Version](https://img.shields.io/badge/version-0.5-blue.svg)](https://github.com/yourusername/EN-OS-Remote-Assistant)
[![Rust](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-AGPL-green.svg)](LICENSE)

Control your PC remotely via Telegram. A lightweight, Rust-based assistant that allows you to execute commands, manage files, and take screenshots directly from your messenger.

> ⚠️ **SECURITY WARNING**  
> This application grants **full control** over your computer to anyone who has access to your Telegram bot. 
> DO NOT GIVE ANYONE ACCESS TO YOUR TELEGRAM ACCOUNT,Otherwise, the attacker will gain full access and control over the computer.

---

## ✨ Features

- **🚀 Command Execution:** Run system commands directly from Telegram (non-sudo).
- **📸 Screenshots:** Capture and view your desktop instantly.
- **📂 File Management:** Browse, view, upload and download files or folders (max size: 20MB).
- **🌐 Display Server Support:** Compatible with both **Wayland** and **X11**.
- **🌍 Bilingual:** Interface available in **English** and **Russian**.
- **⌨️ Input emulate:** Emulate keyboard input via **ydotool**.
- **🦀 Written in Rust:** Fast, memory-safe, and reliable.
- **⚡ Proxy support:** Support HTTP and SOCKS5 proxies.
- **🔌 Power management:** Turn it off, reboot, put it to sleep and hibernate your PC.
- **🧩 Convenient control via a Telegram bot** I have tried to create a user-friendly and uncluttered control panel.

---

## 🛠️ Prerequisites

Before building, ensure you have the following installed:
- [Rustup](https://rustup.rs/)
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
The .env file must be in the same folder as the binary or in the folder where you run the program through the terminal.

> 🔒 **Tip:** Add `.env` to your `.gitignore` file to prevent accidentally committing secrets to GitHub.

### 3. Build and Run
1: Install ```Rustup```, ```ydotool``` and ```scrot``` (if you have x11 environment)
2: Set up the Ydotool  
Install: 
```bash
sudo pacman -S ydotool
```  
Create udev rule: 
```bash 
echo 'KERNEL=="uinput", MODE="0660", GROUP="input", OPTIONS+="static_node=uinput"' | sudo tee "/etc/udev/rules.d/99-uinput.rules" > /dev/null
```  
Install yorself to input group: 
```bash
gpasswd -a $USER input
```  
Reboot yor PC.  
2: Compile the project:
```bash
rustup target add x86_64-unknown-linux-musl
```  
```bash
cargo build --release --target x86_64-unknown-linux-musl
```  
3.Your binary file spawn in ```./target/x86_64-unknown-linux-musl/release/```  
4. You can create desktop file in ~.config/autostart for execution file when turning on the computer.  

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