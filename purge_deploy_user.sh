#!/bin/bash

set -e

# Check if script is run with sudo privileges
if [ "$EUID" -ne 0 ]; then 
    echo "Please run as root or with sudo"
    exit 1
fi

# Check if deploy user exists
if ! id "deploy" &>/dev/null; then
    echo "User 'deploy' does not exist."
    exit 0
fi

echo "Starting cleanup of deploy user..."

# Remove sudoers file
SUDOERS_FILE="/etc/sudoers.d/deploy"
if [ -f "$SUDOERS_FILE" ]; then
    sudo rm "$SUDOERS_FILE"
    echo "Removed sudoers file."
fi

# Get deploy user's home directory before removing user
DEPLOY_HOME=$(eval echo ~deploy)

# Remove SSH directory and all its contents
SSH_DIR="$DEPLOY_HOME/.ssh"
if [ -d "$SSH_DIR" ]; then
    sudo rm -rf "$SSH_DIR"
    echo "Removed SSH directory and all keys."
fi

# Remove .hushlogin if it exists
if [ -f "$DEPLOY_HOME/.hushlogin" ]; then
    sudo rm "$DEPLOY_HOME/.hushlogin"
    echo "Removed .hushlogin file."
fi

# Remove deploy user and their home directory
sudo userdel -r deploy
echo "Removed deploy user and home directory."

echo "Cleanup completed successfully."