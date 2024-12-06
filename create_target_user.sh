#!/bin/bash

set -e

# Check if script is run with sudo privileges
if [ "$EUID" -ne 0 ]; then 
    echo "Please run as root or with sudo"
    exit 1
fi

# Check if username parameter is provided
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <username>"
    echo "Example: $0 deploy"
    exit 1
fi


# Store username parameter
USERNAME="$1"

# Create a new user if it doesn't already exist
if id "$USERNAME" &>/dev/null; then
    echo "User '$USERNAME' already exists."
else
    sudo useradd -m -s /bin/bash "$USERNAME"
    echo "User '$USERNAME' created."
fi

# Add user to the sudo group
sudo usermod -aG sudo "$USERNAME"
echo "'$USERNAME' user added to sudo group."

# Backup and configure sudoers for user
SUDOERS_FILE="/etc/sudoers.d/$USERNAME"
if [ -f "$SUDOERS_FILE" ]; then
    BACKUP_FILE="${SUDOERS_FILE}.bak.$(date +%Y%m%d_%H%M%S)"
    sudo cp "$SUDOERS_FILE" "$BACKUP_FILE"
    echo "Backed up existing sudoers file to $BACKUP_FILE"
fi

# Configure passwordless sudo
echo "$USERNAME ALL=(ALL) NOPASSWD:ALL" | sudo tee "$SUDOERS_FILE"
sudo chmod 0440 "$SUDOERS_FILE"
echo "Configured passwordless sudo for '$USERNAME'."

# Generate SSH key pair
USER_HOME=$(eval echo ~$USERNAME)
SSH_DIR="$USER_HOME/.ssh"
PRIVATE_KEY="$SSH_DIR/id_rsa"
PUBLIC_KEY="$SSH_DIR/id_rsa.pub"

sudo -u "$USERNAME" mkdir -p "$SSH_DIR"
sudo -u "$USERNAME" chmod 700 "$SSH_DIR"

if [ ! -f "$PRIVATE_KEY" ]; then
    if ! sudo -u "$USERNAME" ssh-keygen -t rsa -b 4096 -f "$PRIVATE_KEY" -N ""; then
        echo "Failed to generate SSH key pair"
        exit 1
    fi
    echo "SSH key pair generated for '$USERNAME'."
else
    echo "SSH key pair already exists for '$USERNAME'."
fi

# Validate key generation results
if [ ! -f "$PRIVATE_KEY" ] || [ ! -f "$PUBLIC_KEY" ]; then
    echo "Error: SSH key generation failed"
    exit 1
fi

# Validate key permissions
if [ "$(stat -c %a "$PRIVATE_KEY")" != "600" ]; then
    echo "Error: Invalid private key permissions"
    exit 1
fi

# Handle authorized_keys more securely
AUTH_KEYS="$SSH_DIR/authorized_keys"
cat "$PUBLIC_KEY" >> "$AUTH_KEYS"
sudo chmod 600 "$AUTH_KEYS"

# Print public key location
echo "Public key location: $PUBLIC_KEY"
echo "Public key content:"
sudo cat "$PUBLIC_KEY"

# Ensure correct permissions
sudo chown -R "$USERNAME:$USERNAME" "$SSH_DIR"
sudo chmod 600 "$PRIVATE_KEY"
sudo chmod 644 "$PUBLIC_KEY"

# Create .hushlogin to suppress login messages
sudo -u "$USERNAME" touch "$USER_HOME/.hushlogin"
echo "Created .hushlogin for cleaner SSH login"

# Configure known_hosts file
KNOWN_HOSTS="$SSH_DIR/known_hosts"
sudo -u "$USERNAME" touch "$KNOWN_HOSTS"
sudo chmod 644 "$KNOWN_HOSTS"

echo "Setup completed successfully for user '$USERNAME'."