
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
    url="https://github.com/joel-hamilton/ask-cli//releases/latest/download/ask-cli-aarch64-apple-darwin.tar.gz"
elif [ "$UNAME" = "Linux" ] ; then
    url="https://github.com/joel-hamilton/ask-cli//releases/latest/download/ask-cli-x86_64-unknown-linux-gnu.tar.gz"
fi

# Downloading the correct binary
echo "Downloading from $url..."
curl -fsSL -o ask-cli.tar.gz $url

# Checking if curl succeeded
if [ $? -ne 0 ] ; then
    echo "Failed to download from $url, please check the url and try again."
    exit 1
fi

# Unzip the download
tar -xvzf ask-cli.tar.gz

# Checking if unzip succeeded
if [ $? -ne 0 ] ; then
    echo "Failed to unzip ask-cli.tar.gz, please check the download."
    exit 1
fi

chmod +x ask-cli 

# Move the file
mv ask-cli /usr/local/bin/ask

# Checking if mv succeeded
if [ $? -ne 0 ] ; then
    echo "Failed to move ask-cli to /usr/local/bin/, please check permissions."
    rm -f ask-cli
    exit 1
fi

echo "Installation completed!"
exit 0