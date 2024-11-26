#!/bin/bash

set -e

# Create a new user 'deploy' if it doesn't already exist
if id "deploy" &>/dev/null; then
    echo "User 'deploy' already exists."
else
    sudo useradd -m -s /bin/bash deploy
    echo "User 'deploy' created."
fi

# Add 'deploy' user to the sudo group
sudo usermod -aG sudo deploy
echo "'deploy' user added to sudo group."

# Configure passwordless sudo for 'deploy'
echo "deploy ALL=(ALL) NOPASSWD:ALL" | sudo tee /etc/sudoers.d/deploy
sudo chmod 0440 /etc/sudoers.d/deploy
echo "Configured passwordless sudo for 'deploy'."

# Generate SSH key pair for 'deploy'
DEPLOY_HOME=$(eval echo ~deploy)
SSH_DIR="$DEPLOY_HOME/.ssh"
PRIVATE_KEY="$SSH_DIR/id_rsa"
PUBLIC_KEY="$SSH_DIR/id_rsa.pub"

sudo -u deploy mkdir -p "$SSH_DIR"
sudo -u deploy chmod 700 "$SSH_DIR"

if [ ! -f "$PRIVATE_KEY" ]; then
    sudo -u deploy ssh-keygen -t rsa -b 4096 -f "$PRIVATE_KEY" -N ""
    echo "SSH key pair generated for 'deploy'."
else
    echo "SSH key pair already exists for 'deploy'."
fi

cat $PUBLIC_KEY >>> $SSH_DIR/authorized_keys

# Print public key location
echo "Public key location: $PUBLIC_KEY"
echo "Public key content:"
sudo cat "$PUBLIC_KEY"

# Ensure correct permissions
sudo chown -R deploy:deploy "$SSH_DIR"
sudo chmod 600 "$PRIVATE_KEY"
sudo chmod 644 "$PUBLIC_KEY"

echo "Setup completed successfully."

