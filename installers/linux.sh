#!/bin/bash

URL="https://github.com/Jupiee/ezpie/releases/download/Latest/ezpie.exe"

if [[ "$OSTYPE" == "linux-gnu"* ]]; then

    DESTINATION="/usr/local/bin"
else

    echo "Unsupported OS"
    exit 1
fi

curl -L "$URL" -o "ezpie.exe"

mv "ezpie.exe" "$DESTINATION"