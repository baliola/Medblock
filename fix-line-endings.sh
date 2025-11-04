#!/bin/bash

# fix shell script line endings for WSL compatibility
# this script converts all .sh files from CRLF to LF
# works on both macOS and Linux/WSL

echo "fixing line endings in shell scripts..."

# detect OS and use appropriate sed command
if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS requires backup extension (empty string = no backup)
    find . -name "*.sh" -type f -exec sed -i '' 's/\r$//' {} \;
else
    # Linux/WSL
    find . -name "*.sh" -type f -exec sed -i 's/\r$//' {} \;
fi

echo "done! all shell scripts now have LF line endings."

