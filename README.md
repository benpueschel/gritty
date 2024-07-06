# gritty

Gritty is a command line tool to manage your remote repositories, written in Rust.
Currently supports any Gitea or GitHub instance, with plans to add support for GitLab.

Gritty is designed to be simple and easy to use, with a focus on the most common
repository management tasks. It is not meant to be a full-featured Git client, but
rather a tool to let you focus on what's important: your code.

# Quick Start

1. Install gritty using the [precompiled binaries](#precompiled-binaries) or by building from source.
2. Create a configuration file by running `gritty create-config`.
3. Add your remotes to the configuration file.
4. If you want to change the default `Keyring` secret storage, you can edit the
   configuration file as described in the [Configuration](#configuration) section.
5. Authenticate with a remote using `gritty auth [remote]`.
   This will prompt you for your username and password. Leave the username blank
   to use an access token (required at this point in time). Gritty will store the
   token in whatever secret storage you specified in the configuration file.

Now you can use gritty to manage your repositories!
See the [Usage](#usage) and [Examples](#Examples) sections for more information
on the available commands.

# Installation

## Precompiled Binaries

Precompiled binaries are available on the [releases page](https://github.com/benpueschel/gritty/releases).
These binaries are built for Linux, MacOS, and Windows.

Just download and extract the archive for your platform and add the binary to your PATH.
One great way to do this is to move the binary to `/usr/local/bin` on Linux or
MacOS, which should already be in your PATH.
The releases also include checksums to verify the integrity of the downloaded files.

## From Source

To build from source, you will need to have Rust installed. You can install
Rust by following the instructions on the [official website](https://www.rust-lang.org/tools/install).
I recommend using `rustup` to manage your Rust installation and toolchains.

Once you have rust installed, clone this repository and build gritty in release mode
(which optimizes the binary and strips debug symbols):
```bash
cargo build --release
```
PLEASE NOTE: building from main is not recommended as it may be unstable.
Consider checking out a release tag instead.

The binary will be located at `target/release/gritty`.
You can move this binary to a directory in your PATH to use it globally:
```bash
sudo mv target/release/gritty /usr/local/bin
```
For Windows, the binary will be located at `target\release\gritty.exe`.
Where you move it is up to you, but it must be in your PATH to use it globally.

# Usage

To see the available commands and options, run:
```bash
gritty help
```

If you didn't manually create a configuration file, running `gritty create-config` will
create a default configuration file in the default location for your platform.
You can then edit this file to add your remotes and access tokens.

Gritty currently supports the following subcommands:
- `gritty auth [remote]`: authenticate with the specified remote.
- `gritty create [repo] [remote]`: create a new repository on the specified remote.
- `gritty delete [repo] [remote]`: delete a repository from the specified remote.
- `gritty list [remote]`: list all repositories on the specified remote.

To see the available options for a subcommand, run:
```bash
gritty help [subcommand]
```

After creating your configuration file, you can authenticate with a remote using:
```bash
gritty auth [remote]
```
This command will prompt you for your username and password. Leave the username
blank to use an access token (which you should definitely do because gritty doesn't
support Basic Auth with a password yet).
Please note that this step is required if using keyring storage for access tokens.

After authenticating, you can use the other subcommands to manage your repositories.

## Configuration

Gritty requires a configuration file to manage remotes and access tokens.
This file is located at `~/.config/gritty/config.toml` on Linux and MacOS, but
can use `~/.gritty/config.toml` as a fallback - though you need to manually move
the config to that path.

The configuration file is a TOML file with the following structure:
```toml
# This will use the system keyring to store access tokens.
# On Linux, you will need to have a keyring daemon,
# like gnome-keyring, installed and running.
# On MacOS and Windows, the keyring should work out of the box.
secrets = "Keyring"

# You can also store access tokens in a plaintext file:
# Please note that this is not recommended for security reasons.
#secrets.SecretsFile = "~/.config/gritty/secrets.toml"

# You can also store access tokens directly in the config file:
# Please note that this is by far the least secure option to store your tokens.
#[secrets.Plaintext.github]
#token = "your_access_token"
#[secrets.Plaintext.gitea]
#token = "your_access_token"

# You can add as many remotes as you like. The name of the remote is
# specified in square brackets after the `remotes` keyword. If you want the
# remote name to contain special characters like periods, you can wrap the
# remote name in quotes:
#[remotes."github.com"]
[remotes.github]
provider = "GitHub"
url = "https://github.com"
username = "your_username"

# This adds a second remote to the configuration file, using Gitea as the provider.
[remotes.gitea]
provider = "Gitea"
url = "https://gitea.example.com"
username = "your_username"
```

# Examples

To create a public repository on GitHub:
```bash
gritty create my-repo github
```

To create a private repository:
```bash
gritty create my-private-repo github --private
```
or use the shorthand:
```bash
gritty create -p my-private-repo github
```

To list all repositories on Gitea:
```bash
gritty list gitea
```

To delete a repository from a remote:
```bash
gritty delete my-repo my-awesome-remote
```
This will prompt you for confirmation and show you the last commit before deleting.
