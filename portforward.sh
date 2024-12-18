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

# Function to check if running on Windows
is_windows() {
    [[ "$(uname)" =~ "MINGW"|"MSYS"|"CYGWIN" ]] || [[ -n "$WINDIR" ]]
}

# Function for Windows-specific port forwarding
windows_forward() {
    echo -e "${BLUE}[INFO]${NC} Windows detected. Using native SSH..."
    
    # Create a temporary script to store credentials (not ideal but temporary solution)
    echo "@echo off" > connect.bat
    echo "echo %SSH_PASSWORD% | ssh -L %LOCAL_PORT%:127.0.0.1:%VPS_PORT% %SSH_USER%@%VPS_IP% -N" >> connect.bat
    
    # Run the batch file
    start connect.bat
    
    echo -e "${GREEN}[SUCCESS]${NC} Port forwarding started. Use ${YELLOW}localhost:$LOCAL_PORT${NC} in your app."
    echo -e "${BLUE}[INFO]${NC} To stop port forwarding, close the command prompt window that opened."
    
    # Clean up the temporary file after a brief delay
    sleep 2
    rm connect.bat 2>/dev/null
}

# Function for Unix-based port forwarding
unix_forward() {
    if ! command -v sshpass &>/dev/null; then
        echo -e "${RED}[ERROR]${NC} sshpass is not installed. Please install it and try again."
        echo -e "For macOS: ${YELLOW}brew install sshpass${NC}"
        echo -e "For Linux: ${YELLOW}sudo apt-get install sshpass${NC} or ${YELLOW}sudo yum install sshpass${NC}"
        exit 1
    fi

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
}

# Main execution
if is_windows; then
    windows_forward
else
    unix_forward
fi