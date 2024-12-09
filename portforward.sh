#!/bin/bash

VPS_IP="62.72.47.205"
LOCAL_PORT="4943"
VPS_PORT="4943"
SSH_USER="guest"
SSH_PASSWORD="pw"

# Check if sshpass is installed
if ! command -v sshpass &>/dev/null; then
    echo "Error: sshpass is not installed. Please install it and try again."
    exit 1
fi

# Create a local port forward to the VPS
echo "Forwarding localhost:$LOCAL_PORT to $VPS_IP:$VPS_PORT..."
sshpass -p "$SSH_PASSWORD" ssh -L "$LOCAL_PORT:127.0.0.1:$VPS_PORT" "$SSH_USER@$VPS_IP" -N &
SSH_PID=$!
echo "Port forwarding started. Use localhost:$LOCAL_PORT in your app."
echo "To stop port forwarding, press Ctrl+C"

# Handle script termination
trap 'kill $SSH_PID 2>/dev/null; echo -e "\nPort forwarding stopped."; exit 0' SIGINT SIGTERM

# Keep script running
while true; do
    sleep 1
done
