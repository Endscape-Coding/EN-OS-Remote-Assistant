#!/bin/bash

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

error() {
    echo -e "${RED}[ERROR] $1${NC}"
    exit 1
}

warn() {
    echo -e "${YELLOW}[WARN] $1${NC}"
}

success() {
    echo -e "${GREEN}[SUCCESS] $1${NC}"
}

if ! command -v cargo &> /dev/null; then
    error "Cargo not found! Install Rust."
fi
success "Cargo found: $(cargo --version)"

ENV_FILE=".env"

set_env_var() {
    local key=$1
    local prompt_msg=$2
    local current_value=""

    if [ ! -f "$ENV_FILE" ]; then
        touch "$ENV_FILE"
    fi

    if grep -q "^${key}=" "$ENV_FILE"; then
        current_value=$(grep "^${key}=" "$ENV_FILE" | cut -d '=' -f 2-)
    fi

    if [ -z "$current_value" ]; then
        echo -n "Enter $prompt_msg: "
        if [ "$key" == "TELOXIDE_TOKEN" ]; then
            read -s new_value
            echo ""
        else
            read new_value
        fi

        if [ -z "$new_value" ]; then
            new_value=""
        fi

        if grep -q "^${key}=" "$ENV_FILE"; then
            sed -i "s|^${key}=.*|${key}=${new_value}|" "$ENV_FILE"
        else
            echo "${key}=${new_value}" >> "$ENV_FILE"
        fi
        success "$key updated in $ENV_FILE"
    else
        success "$key exists in $ENV_FILE"
    fi
}

set_env_var "TELOXIDE_TOKEN" "Telegram Bot Token"
set_env_var "ID" "Chat/User ID"

if ! command -v ydotool &> /dev/null; then
    warn "ydotool not found. Please install it manually."
else
    success "ydotool found"
fi

if [ ! -e /dev/uinput ]; then
    error "/dev/uinput not found. Load module: sudo modprobe uinput"
fi

ls -l /dev/uinput

UDEV_RULE="/etc/udev/rules.d/99-uinput.rules"
UDEV_CONTENT='KERNEL=="uinput", MODE="0660", GROUP="input", OPTIONS+="static_node=uinput"'

if [ ! -f "$UDEV_RULE" ]; then
    warn "udev rule not found. Creating..."
    echo "$UDEV_CONTENT" | sudo tee "$UDEV_RULE" > /dev/null
    success "udev rule created"
else
    if ! grep -qF "$UDEV_CONTENT" "$UDEV_RULE"; then
        warn "udev rule content mismatch. Updating..."
        echo "$UDEV_CONTENT" | sudo tee "$UDEV_RULE" > /dev/null
        success "udev rule updated"
    else
        success "udev rule correct"
    fi
fi

if ! id -nG | grep -qw input; then
    warn "User not in 'input' group. Adding..."
    sudo usermod -aG input $USER
    success "User added to 'input' group"
else
    success "User already in 'input' group"
fi

sudo mkdir -p /var/log/en-os/remote_assistant
sudo chown $USER:$USER /var/log/en-os/remote_assistant
sudo chmod 755 /var/log/en-os/remote_assistant

echo "Starting compilation..."
if cargo build --release; then
    success "Compilation successful!"
    BIN_NAME=$(grep "^name" Cargo.toml | head -1 | cut -d '"' -f 2)
    if [ -n "$BIN_NAME" ]; then
        echo -e "Binary: ${GREEN}target/release/${BIN_NAME}${NC}"
    fi
else
    error "Compilation failed!"
fi

warn "Changes to groups or udev rules require a reboot to take effect."
read -p "Do you want to reboot now? (y/n): " reboot_choice
if [ "$reboot_choice" == "y" ]; then
    sudo reboot
fi
