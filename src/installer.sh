#!/bin/bash

# Identify the OS and Architecture
UNAME=$(uname)
if [ "$UNAME" != "Linux" -a "$UNAME" != "Darwin" ] ; then
    echo "Sorry, OS not supported: $UNAME"
    exit 1
fi

ARCH=$(uname -m)
if [ "$ARCH" != "x86_64" -a "$ARCH" != "aarch64" -a "$ARCH" != "arm64" ] ; then
    echo "Sorry, architecture not supported: $ARCH"
    exit 1
fi

# Choose binary to download
if [ "$UNAME" = "Darwin" ] ; then
    url="https://link.to/your.app/releases/ask-cli-${ARCH}-apple-darwin"
elif [ "$UNAME" = "Linux" ] ; then
    url="https://link.to/your.app/releases/ask-cli-${ARCH}-unknown-linux-gnu"
fi

# Downloading the correct binary
echo "Downloading from $url..."
curl -fsSL -o ask-cli $url

# Checking if curl succeeded
if [ $? -ne 0 ] ; then
    echo "Failed to download from $url, please check the url and try again."
    exit 1
fi

chmod +x ask-cli
mv ask-cli /usr/local/bin/

echo "Installation completed!"
exit 0