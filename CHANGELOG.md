# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [unreleased](https://github.com/benpueschel/gritty/compare/v1.0.1..HEAD)

### ⚙️ Miscellaneous Tasks

- *(main)* Release v1.0.1
- Use sccache

## [v1.0.1](https://github.com/benpueschel/gritty/compare/v1.0.0..v1.0.1) - 2024-08-15

### 🐛 Bug Fixes

- Correct help for repo names in `gritty help`

### 📚 Documentation

- Add issue templates
- Include build status badges

### ⚙️ Miscellaneous Tasks

- *(main)* Release v1.0.0
- V1.0.1

## [v1.0.0](https://github.com/benpueschel/gritty/compare/v0.8.1..v1.0.0) - 2024-08-07

### 💥 BREAKING CHANGES

- due to [018f00f](https://github.com/benpueschel/gritty/commit/018f00ffbb0ad47202cb7873e46045995b1b4738) - simplify secrets config:
  Old configurations will break. Users will need to
  change the `secrets` section of their configuration:
  ```toml
  [secrets]
  type = "Keyring"
  ```
  Replaces the old `secrets = "Keyring" to use the system keyring to store
  secrets.
  ```toml
  [secrets]
  type = "SecretsFile"
  file = "path/to/file.toml"
  ```
  Replaces the old way to store secrets in a separate plaintext secrets
  file.
  ```toml
  [secrets]
  type = "Plaintext"

  [secrets.your_remote]
  token = "token"
  [secrets.remote]
  username = "user"
  password = "password"
  ```
  Replaces the old way to store secrets inline. Instead of the old
  `[secrets.Plaintext.<provider>`, the config now directly uses
  `[secrets.<provider>`, with a section to denote the type to use
  (`secrets.type`).

- due to [41cc53f](https://github.com/benpueschel/gritty/commit/41cc53f3fbfcfd525591ee3f9526353686f67712) - use PathBuf for config path:
  The System keyring now uses a canonical path to store
  credentials. This may break credential storage. If you encounter the
  error message `Could not find auth for remote <remote_name>`, you will
  need to call `gritty auth <remote_name>` and supply a new token.


### 🚀 Features

- Add script to pull breaking changes
- Nice panic handler
- Include issue url when panicking
- *(remote)* Return full repo upon creation
- Add format option to create/list commands
- Windows support?
- [**breaking**] Use PathBuf for config path
- Print version with -V

### 🐛 Bug Fixes

- *(breaking.sh)* Typo in help overview
- Error when compiling without keyring feature
- Correct pre-color loading error handling
- Correct archive file name
- Add gitlab remote on `gritty create-config`

### 🚜 Refactor

- *(config)* [**breaking**] Simplify secrets config
- Use BTreeMap for config
- Allow non-trailing commas in map macro
- Remove unused import
- Move cli arguments to separate crate
- Split subcommands into separate modules

### 📚 Documentation

- Better cli help

### 🧪 Testing

- *(config)* Add basic tests

### ⚙️ Miscellaneous Tasks

- *(main)* Release v0.8.1
- Fix gritty-clap version
- V1.0.0

### Build

- Add cargo package metadata
- Generate man-pages on build

## [v0.8.1](https://github.com/benpueschel/gritty/compare/v0.8.0..v0.8.1) - 2024-07-30

### 🐛 Bug Fixes

- Load default colors on create-config command

### 📚 Documentation

- Add section for install script

### ⚙️ Miscellaneous Tasks

- *(main)* Release v0.8.0
- V0.8.1

### Build

- Add install script for linux/macos
- *(install.sh)* Allow custom version tags

## [v0.8.0](https://github.com/benpueschel/gritty/compare/v0.7.0..v0.8.0) - 2024-07-27

### 🚀 Features

- Add option to git init current dir
- Dynamically configurable colors :)
- Automatically pull from remote upon creation

### 🚜 Refactor

- *(log)* New color api (again)

### 📚 Documentation

- Add gitlab in create-config section
- Actually add gitlab support (once more)
- Add missing subcommands
- Add color config section
- Add example images
- Makefile all the way

### ⚙️ Miscellaneous Tasks

- *(main)* Release v0.7.0
- V0.8.0

## [v0.7.0](https://github.com/benpueschel/gritty/compare/v0.6.0..v0.7.0) - 2024-07-23

### 🚀 Features

- *(delete)* Add option to force-delete repo

### 🐛 Bug Fixes

- *(config)* Correct error formatting
- *(config)* Actually load the specified config oops

### 🚜 Refactor

- *(config)* Remove default remotes

### 🎨 Styling

- Use colors when printing error messages
- *(remote)* Uppercase WARNING on comment

### ⚙️ Miscellaneous Tasks

- *(main)* Release v0.6.0
- Bump deprecated action versions
- Update dependencies
- V0.7.0

## [v0.6.0](https://github.com/benpueschel/gritty/compare/v0.5.0..v0.6.0) - 2024-07-22

### 💥 BREAKING CHANGES

- due to [95b5d41](https://github.com/benpueschel/gritty/commit/95b5d41fc0a24b2fc6fd6cb33c2609e8f7a2f0b4) - toggle listing private repos:
  Private repositories are not shown by default anymore.
  To list public and private repos, run `gritty list <remote> -p`.


### 🚀 Features

- [**breaking**] Toggle listing private repos
- Add option to show forks

### 🐛 Bug Fixes

- Remove atty, use std method to detect tty
- Correctly filter out private repos on gitlab

### 🚜 Refactor

- Use println macro and custom style to log
- *(log)* Add leftpad function
- Move subcommands into more atomic units

### 🎨 Styling

- *(log)* Remove quotes on repo/remote names
- *(log)* Forgot to remove all quotes oops

### ⚙️ Miscellaneous Tasks

- *(main)* Release v0.5.0
- V0.6.0

## [v0.5.0](https://github.com/benpueschel/gritty/compare/v0.4.0..v0.5.0) - 2024-07-20

### 🚀 Features

- Add global --config (-C) option
- Add option to recursively clone repos

### 🐛 Bug Fixes

- Properly respect NO_COLOR env variable
- Only colorize if stdout is a tty

### 🚜 Refactor

- Move subcommands into dedicated structs

### ⚙️ Miscellaneous Tasks

- *(main)* Release v0.4.0
- Fix automatic releases
- V0.5.0

## [v0.4.0](https://github.com/benpueschel/gritty/compare/v0.3.0..v0.4.0) - 2024-07-18

### 🚀 Features

- Add description option when creating repo
- Add gitlab remote

### 🐛 Bug Fixes

- *(create-config)* Add missing open paren

### 🚜 Refactor

- Use DateTime for commit date

### 📚 Documentation

- Remove chore tasks from changelog

### ⚙️ Miscellaneous Tasks

- *(main)* Release v0.3.0
- *(release)* Only include recent changes
- *(release)* Exclude some types from changelog
- Bump dependencies
- V0.4.0

## [v0.3.0](https://github.com/benpueschel/gritty/compare/v0.2.0..v0.3.0) - 2024-07-17

### 🚀 Features

- Spawn concurrent list-repo tasks
- List configured remotes

### 🚜 Refactor

- *(log)* Remove unused, empty macro

### ⚙️ Miscellaneous Tasks

- *(main)* Release v0.2.0
- V0.3.0

## [v0.2.0](https://github.com/benpueschel/gritty/compare/v0.1.1..v0.2.0) - 2024-07-08

### 🚀 Features

- Interactive configuration
- Clone remote repo
- *(args)* Add colors to help menu

### 🐛 Bug Fixes

- *(config)* Create secrets file if it doesn't exist

### 🚜 Refactor

- *(commands)* Add get_input method
- *(args)* Move from structopt to clap

### 🎨 Styling

- *(remote)* Fix top-level docs

### ⚙️ Miscellaneous Tasks

- *(main)* Release v0.1.1
- Bump dependency versions
- V0.2.0

## [v0.1.1](https://github.com/benpueschel/gritty/compare/v0.1.0..v0.1.1) - 2024-07-07

### 🚜 Refactor

- *(remote)* Move clone to top-level trait
- *(remote)* Add static COMMIT_COUNT
- *(github)* Code cleanup
- Unified error type
- *(main)* Cleanup main function

### ⚙️ Miscellaneous Tasks

- *(main)* Release v0.1.0
- Bump version to 0.1.1

### Build

- Add Makefile

## [v0.1.0](https://github.com/benpueschel/gritty/compare/v0.0.1..v0.1.0) - 2024-07-06

### 💥 BREAKING CHANGES

- due to [bb39c74](https://github.com/benpueschel/gritty/commit/bb39c74e715930001b594b0f51281b4343047fac) - ask for confirmation on delete:
  ask for confirmation on delete

- due to [d80983a](https://github.com/benpueschel/gritty/commit/d80983aff68d077300f2154e26d02602c2efeac4) - change config path:
  Old configs located at ~/.config/gitrc-rs/ will need to
  be relocated to ~/.config/gritty/

- due to [50b2fb4](https://github.com/benpueschel/gritty/commit/50b2fb4a02610a5789c36664db4b841327c69f2e) - add option to clone repo on creation:
  Configs now need a "clone_protocol" option for each
  remote.


### 🚀 Features

- Cli remote authentication
- Use keyring secret by default
- [**breaking**] Ask for confirmation on delete
- List repositories
- Color highlighting
- Make keyring an optional feature
- [**breaking**] Change config path
- Add command to create default config
- [**breaking**] Add option to clone repo on creation
- Implement repo initialization

### 🐛 Bug Fixes

- *(config)* Only save when using plaintext auth
- *(github)* Handle empty repo commit infos
- *(gitea)* Handle empty repo commit infos
- Don't print full error when deleting repo
- Don't print default config on auth
- Git clone ssh url

### 📚 Documentation

- Update README.md

### ⚙️ Miscellaneous Tasks

- *(main)* Release v0.0.1
- Cache rust binaries
- *(release)* Change name to gritty :)

### Other

- Change name to gritty :)

## [v0.0.1](https://github.com/benpueschel/gritty/compare/v0.0.0..v0.0.1) - 2024-07-06

### 💥 BREAKING CHANGES

- due to [419df8e](https://github.com/benpueschel/gritty/commit/419df8e666a03e9a669b38bda70d36b726eeb714) - simplify plaintext auth config:
  Old configs are not valid anymore and will not work.


### 🚀 Features

- Add git remote trait
- Add github remote
- Add gitea impl
- Basic toml config
- [**breaking**] Simplify plaintext auth config
- Add secrets file config option
- Add platform-specific keyring config option
- Add config option to store secrets
- Add option to save config
- Parse args for an actual working version!

### 🐛 Bug Fixes

- Remove log.txt
- Repo commit fetching
- Correct repo clone url
- Set version to 0.0.1

### 🚜 Refactor

- Move map_error to remote/mod.rs

### 📚 Documentation

- Add MIT license

### ⚙️ Miscellaneous Tasks

- Add rust workflow
- Automatic release workflow

<!-- generated by git-cliff -->
