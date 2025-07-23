#!/bin/bash

set -e

if [ "$1" == "--build-only" ]; then
    echo "Building GambleGuard binary only (Linux)..."
    # Build without sudo so cargo is found
    cargo build --release
    exit 0
fi

echo "Installing GambleGuard on Linux..."

# Ensure build binary exists
if [ ! -f ./target/release/gambleguard ]; then
    echo "Building binary..."
    cargo build --release
fi

# Copy binary
echo "Copying binary to /usr/local/bin..."
sudo cp ./target/release/gambleguard /usr/local/bin/gambleguard
sudo chmod +x /usr/local/bin/gambleguard

# Prepare config directory
echo "Preparing config directory..."
sudo mkdir -p /etc/gambleguard
sudo cp gambleguard_domain_blocklist.txt /etc/gambleguard/gambleguard_domain_blocklist.txt

# Create systemd service file
SERVICE_FILE="/etc/systemd/system/gambleguard.service"

echo "Creating systemd service..."
sudo bash -c "cat > $SERVICE_FILE" <<EOF
[Unit]
Description=GambleGuard Parental Protection
After=network.target

[Service]
Type=simple
ExecStart=/usr/local/bin/gambleguard
Restart=always
RestartSec=5
StandardOutput=syslog
StandardError=syslog
SyslogIdentifier=gambleguard
User=root

[Install]
WantedBy=multi-user.target
EOF

# Reload and enable service
echo "Reloading systemd and enabling service..."
sudo systemctl daemon-reexec
sudo systemctl daemon-reload
sudo systemctl enable gambleguard.service

# Restart or start the service
if systemctl is-active --quiet gambleguard.service; then
    echo "Restarting existing GambleGuard service..."
    sudo systemctl restart gambleguard.service
else
    echo "Starting GambleGuard service..."
    sudo systemctl start gambleguard.service
fi

echo "GambleGuard installed and running on Linux!"
