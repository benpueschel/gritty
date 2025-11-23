#!/usr/bin/env bash

### gritty install script
# This script downloads the latest gritty release for the current platform and
# installs it to /usr/local/bin/gritty. The script requires sudo, curl, jq, and
# shasum to be installed on the system.
#

PLATFORM=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

if [ "$ARCH" == "amd64" ]; then
	ARCH="x86_64"
elif [ "$ARCH" == "arm64" ]; then
	ARCH="aarch64"
fi

VERSION="latest"

# check if sudo is installed
if ! command -v sudo &> /dev/null; then
	echo "sudo is required to install gritty. Please install sudo and try again."
	exit 1
fi

# check if jq is installed
if ! command -v jq &> /dev/null; then
	echo "jq is required to parse JSON responses. Please install jq and try again."
	exit 1
fi

# check if curl is installed
if ! command -v curl &> /dev/null; then
	echo "curl is required to download files. Please install curl and try again."
	exit 1
fi

# check if shasum is installed
if ! command -v shasum &> /dev/null; then
	echo "shasum is required to verify the integrity of the downloaded files. Please install shasum and try again."
	exit 1
fi

print_help() {
	COMMAND=$(basename "$0")
	echo "Usage: $COMMAND [options]"
	echo "Options:"
	echo "  -h, --help        Show this help message and exit"
	echo "  -v, --version     Set the gritty version to download (default: latest)"
	echo "  -p, --platform    Specify the platform to download for (default: $PLATFORM)"
	echo "  -a, --arch        Specify the architecture to download for (default: $ARCH)"
}

while getopts ":h:p:v:" opt; do
	case $opt in
		h)
			print_help
			exit 0
			;;
		p)
			PLATFORM=$OPTARG
			;;
		v)
			VERSION=$OPTARG
			;;
		a)
			ARCH=$OPTARG
			;;
		\?)
			>&2 echo "Invalid option: -$OPTARG" >&2
			print_help
			exit 1
			;;
		:)
			>&2 echo "Option -$OPTARG requires an argument." >&2
			print_help
			exit 1
			;;
	esac
done

echo "Downloading gritty-$VERSION for $ARCH-$PLATFORM..."

# get the latest release's asset url endpoint
if [ "$VERSION" == "latest" ]; then
	ASSETS_URL=$(curl -s "https://api.github.com/repos/benpueschel/gritty/releases/latest" | jq -r '.assets_url')
else
	ASSETS_URL=$(curl -s "https://api.github.com/repos/benpueschel/gritty/releases/tags/$VERSION" | jq -r '.assets_url')
fi

# get all assets matching the platform
ASSETS=$(curl -s "$ASSETS_URL" | jq -c ".[] | select ( .name | contains(\"$PLATFORM\") and contains(\"$ARCH\") )")

# get all download urls
ASSET_URLS=$(echo $ASSETS | jq -r '.browser_download_url')

# download all assets
for url in $ASSET_URLS; do
	echo "Downloading '$url'..."
	curl -LO "$url"
done

# get the archive and sha256 hash files
ARCHIVE=$(echo $ASSETS | jq -rc 'select(.name | endswith(".tar.gz")) | .name')
SHA256=$(echo $ASSETS | jq -rc 'select(.name | endswith(".sha256")) | .name')

# check the sha256 hash against the archive file
echo "Checking sha256 hash for $ARCHIVE..."

cat $SHA256 | shasum -ca 256

if [ $? -ne 0 ]; then exit 1; fi

# find the gritty binary in the archive
BINARY=$(tar ztf "$ARCHIVE" | grep 'gritty')

# extract the binary from the archive and don't expand into a folder, instead
# extract the binary directly into the current directory (./gritty)
tar --strip-components=1 -xzf "$ARCHIVE" "$BINARY"

# make the binary executable
chmod +x gritty

# move the binary to /usr/local/bin
sudo mv gritty /usr/local/bin/gritty

# clean up the downloaded files
rm "$ARCHIVE" "$SHA256"

echo "gritty has been installed to /usr/local/bin/gritty"
