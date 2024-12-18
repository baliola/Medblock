#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

VPS_IP="62.72.47.205"
LOCAL_PORT="4943"
VPS_PORT="4943"
SSH_USER="guest"
SSH_PASSWORD="pw"

# Check if sshpass is installed
if ! command -v sshpass &>/dev/null; then
    echo -e "${RED}[ERROR]${NC} sshpass is not installed. Please install it and try again. Use ${YELLOW}\`brew install sshpass\`${NC} if you are on macOS."
    exit 1
fi

# Create a local port forward to the VPS
echo -e "${BLUE}[INFO]${NC} Forwarding localhost:${GREEN}$LOCAL_PORT${NC} to ${GREEN}$VPS_IP:$VPS_PORT${NC}..."
sshpass -p "$SSH_PASSWORD" ssh -L "$LOCAL_PORT:127.0.0.1:$VPS_PORT" "$SSH_USER@$VPS_IP" -N &
SSH_PID=$!
echo -e "${GREEN}[SUCCESS]${NC} Port forwarding started. Use ${YELLOW}localhost:$LOCAL_PORT${NC} in your app."
echo -e "${BLUE}[INFO]${NC} To stop port forwarding, press ${YELLOW}Ctrl+C${NC}"

# Handle script termination
trap 'kill $SSH_PID 2>/dev/null; echo -e "\n${GREEN}[SUCCESS]${NC} Port forwarding stopped."; exit 0' SIGINT SIGTERM

# Keep script running
while true; do
    sleep 1
done