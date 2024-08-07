# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [v1.0.0] - 2024-08-07
### :boom: BREAKING CHANGES
- due to [`018f00f`](https://github.com/benpueschel/gritty/commit/018f00ffbb0ad47202cb7873e46045995b1b4738) - simplify secrets config *(commit by [@benpueschel](https://github.com/benpueschel))*:

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

- due to [`41cc53f`](https://github.com/benpueschel/gritty/commit/41cc53f3fbfcfd525591ee3f9526353686f67712) - use PathBuf for config path *(commit by [@benpueschel](https://github.com/benpueschel))*:

  The System keyring now uses a canonical path to store  
  credentials. This may break credential storage. If you encounter the  
  error message `Could not find auth for remote <remote_name>`, you will  
  need to call `gritty auth <remote_name>` and supply a new token.


### :sparkles: New Features
- [`435a202`](https://github.com/benpueschel/gritty/commit/435a202307b39872eb2b74ce321ac7d68feae88d) - add script to pull breaking changes *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`ee84498`](https://github.com/benpueschel/gritty/commit/ee84498830b3515228031e83ef23b9f6633c9067) - nice panic handler *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`b828443`](https://github.com/benpueschel/gritty/commit/b8284436c9493f45fe512235abebb4efe1ad0828) - include issue url when panicking *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`8044fe8`](https://github.com/benpueschel/gritty/commit/8044fe8968a20818800e4c16d3fe5039feee2710) - **remote**: return full repo upon creation *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`f9959b4`](https://github.com/benpueschel/gritty/commit/f9959b4846c5988bed8c7c569558bd6c99a8ee17) - add format option to create/list commands *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`4e94c08`](https://github.com/benpueschel/gritty/commit/4e94c08e9155104581f7e27510edfb3eb469a672) - windows support? *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`41cc53f`](https://github.com/benpueschel/gritty/commit/41cc53f3fbfcfd525591ee3f9526353686f67712) - use PathBuf for config path *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`34e0a27`](https://github.com/benpueschel/gritty/commit/34e0a27e6a571eec0a560c5c0bebb8aee6e2b93a) - print version with -V *(commit by [@benpueschel](https://github.com/benpueschel))*

### :bug: Bug Fixes
- [`77fed2f`](https://github.com/benpueschel/gritty/commit/77fed2fb2c23658adccba3c15a6e726e19b52b0d) - **breaking.sh**: typo in help overview *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`2f1420d`](https://github.com/benpueschel/gritty/commit/2f1420df1f67573a23766d60dc32b836ebc8d4aa) - error when compiling without keyring feature *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`c63ed29`](https://github.com/benpueschel/gritty/commit/c63ed299c9e6630f4ebbcacec606c218fc0f7db1) - correct pre-color loading error handling *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`fbd0c25`](https://github.com/benpueschel/gritty/commit/fbd0c25d3c13cbd493e3b515fe8892c920a44e6b) - correct archive file name *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`2e659f6`](https://github.com/benpueschel/gritty/commit/2e659f6a863f221356c139c5c06cf02c68af4105) - add gitlab remote on `gritty create-config` *(commit by [@benpueschel](https://github.com/benpueschel))*

### :recycle: Refactors
- [`018f00f`](https://github.com/benpueschel/gritty/commit/018f00ffbb0ad47202cb7873e46045995b1b4738) - **config**: simplify secrets config *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`c97317c`](https://github.com/benpueschel/gritty/commit/c97317ca8bd0b7825f36ab55228980753dfe0cd4) - use BTreeMap for config *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`ab6dceb`](https://github.com/benpueschel/gritty/commit/ab6dcebb91b1a53fb9414171a80f32dc30b8fd1d) - allow non-trailing commas in map macro *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`a8c7775`](https://github.com/benpueschel/gritty/commit/a8c7775b5ca53235270a860df15657e11b8c22a8) - remove unused import *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`a8edce0`](https://github.com/benpueschel/gritty/commit/a8edce0158dd65acd9c8aa5536d4ef751899de36) - move cli arguments to separate crate *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`946ee70`](https://github.com/benpueschel/gritty/commit/946ee706f9c26e2924af2cf7452d101c10d0a459) - split subcommands into separate modules *(commit by [@benpueschel](https://github.com/benpueschel))*

### :white_check_mark: Tests
- [`11772b2`](https://github.com/benpueschel/gritty/commit/11772b25d566fc998caf15e3b5202972eed3be2e) - **config**: add basic tests *(commit by [@benpueschel](https://github.com/benpueschel))*


## [v0.8.1] - 2024-07-30
### :bug: Bug Fixes
- [`82ee9e9`](https://github.com/benpueschel/gritty/commit/82ee9e9dac2dcd49970d60a2f4a59ac08dcf5908) - load default colors on create-config command *(commit by [@benpueschel](https://github.com/benpueschel))*


## [v0.8.0] - 2024-07-27
### :sparkles: New Features
- [`79f1f0b`](https://github.com/benpueschel/gritty/commit/79f1f0b2353bded13ab3ae2789a0033962d3bcd6) - add option to git init current dir *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`c1409e9`](https://github.com/benpueschel/gritty/commit/c1409e9768660834be2387c248841d467dc8e43a) - dynamically configurable colors :) *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`427b07f`](https://github.com/benpueschel/gritty/commit/427b07fa8ef10460ad80acf33be20a8f6d8d2ddf) - automatically pull from remote upon creation *(commit by [@benpueschel](https://github.com/benpueschel))*

### :recycle: Refactors
- [`08c1e7c`](https://github.com/benpueschel/gritty/commit/08c1e7c491071dc361c3b8de6a9581845f612a52) - **log**: new color api (again) *(commit by [@benpueschel](https://github.com/benpueschel))*


## [v0.7.0] - 2024-07-23
### :sparkles: New Features
- [`60161ce`](https://github.com/benpueschel/gritty/commit/60161ce3bbb619e47f4054fbaec832e7dce18538) - **delete**: add option to force-delete repo *(commit by [@benpueschel](https://github.com/benpueschel))*

### :bug: Bug Fixes
- [`2709945`](https://github.com/benpueschel/gritty/commit/27099452eb348b3da9d0f0a2ddfe5390e7263a85) - **config**: correct error formatting *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`84ea33e`](https://github.com/benpueschel/gritty/commit/84ea33efeee7107fd26f86dad52e87fd31ad0396) - **config**: actually load the specified config oops *(commit by [@benpueschel](https://github.com/benpueschel))*

### :recycle: Refactors
- [`d3beea1`](https://github.com/benpueschel/gritty/commit/d3beea115f6db7d5d08946203b57ad0f907ddada) - **config**: remove default remotes *(commit by [@benpueschel](https://github.com/benpueschel))*


## [v0.6.0] - 2024-07-22
### :boom: BREAKING CHANGES
- due to [`95b5d41`](https://github.com/benpueschel/gritty/commit/95b5d41fc0a24b2fc6fd6cb33c2609e8f7a2f0b4) - toggle listing private repos *(commit by [@benpueschel](https://github.com/benpueschel))*:

  Private repositories are not shown by default anymore.  
  To list public and private repos, run `gritty list <remote> -p`.


### :sparkles: New Features
- [`95b5d41`](https://github.com/benpueschel/gritty/commit/95b5d41fc0a24b2fc6fd6cb33c2609e8f7a2f0b4) - toggle listing private repos *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`a6a7285`](https://github.com/benpueschel/gritty/commit/a6a7285d7afb491e56444b68965813a76dfda335) - add option to show forks *(commit by [@benpueschel](https://github.com/benpueschel))*

### :bug: Bug Fixes
- [`383ad57`](https://github.com/benpueschel/gritty/commit/383ad5725cd9cb15a8e7ff0fe4feb525026c32a9) - remove atty, use std method to detect tty *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`d6a736d`](https://github.com/benpueschel/gritty/commit/d6a736d345f23186ae9c554258b87a18352713e6) - correctly filter out private repos on gitlab *(commit by [@benpueschel](https://github.com/benpueschel))*

### :recycle: Refactors
- [`d01d8da`](https://github.com/benpueschel/gritty/commit/d01d8da6faa028f37a7aaced5b786485f646229f) - use println macro and custom style to log *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`1e8246b`](https://github.com/benpueschel/gritty/commit/1e8246b7b9f4abfcb2067febc1b7a2aed70e6c2d) - **log**: add leftpad function *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`677f2a3`](https://github.com/benpueschel/gritty/commit/677f2a3e148526bdd6c3ff5679f1ce2f79e3779e) - move subcommands into more atomic units *(commit by [@benpueschel](https://github.com/benpueschel))*


## [v0.5.0] - 2024-07-20
### :sparkles: New Features
- [`b7d17d6`](https://github.com/benpueschel/gritty/commit/b7d17d6b358256ad8cc3101e87d83c1b560f7f4c) - add global --config (-C) option *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`7c7bc8b`](https://github.com/benpueschel/gritty/commit/7c7bc8b38d25eda330f2661a02bde5ba54274469) - add option to recursively clone repos *(commit by [@benpueschel](https://github.com/benpueschel))*

### :bug: Bug Fixes
- [`ae08291`](https://github.com/benpueschel/gritty/commit/ae08291761e7ef659e18cdd07374892bc56515b2) - properly respect NO_COLOR env variable *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`49ab416`](https://github.com/benpueschel/gritty/commit/49ab416be6bad45726a52f8513e9339f5f2c2532) - only colorize if stdout is a tty *(commit by [@benpueschel](https://github.com/benpueschel))*

### :recycle: Refactors
- [`ea8fe55`](https://github.com/benpueschel/gritty/commit/ea8fe55113c378c1620fff02cf7d2ccc91c3f820) - move subcommands into dedicated structs *(commit by [@benpueschel](https://github.com/benpueschel))*


## [v0.4.0] - 2024-07-18
### :sparkles: New Features
- [`0ffd216`](https://github.com/benpueschel/gritty/commit/0ffd2166a51c13fa5941a7b269f0a3e206adcfdf) - add description option when creating repo *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`855c420`](https://github.com/benpueschel/gritty/commit/855c420222cb285960a08027928f21b9614fe944) - add gitlab remote *(commit by [@benpueschel](https://github.com/benpueschel))*

### :bug: Bug Fixes
- [`87fe5ca`](https://github.com/benpueschel/gritty/commit/87fe5ca55c24b919e9092f25ebe5fc0170113e61) - **create-config**: add missing open paren *(commit by [@benpueschel](https://github.com/benpueschel))*

### :recycle: Refactors
- [`5e40db8`](https://github.com/benpueschel/gritty/commit/5e40db8f387bf643f7af05a6251d28c5b0676df0) - use DateTime for commit date *(commit by [@benpueschel](https://github.com/benpueschel))*


## [v0.3.0] - 2024-07-17
### :sparkles: New Features
- [`36ebef1`](https://github.com/benpueschel/gritty/commit/36ebef13c86efc15c1d7dec9507352740f4c46e1) - spawn concurrent list-repo tasks *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`306354e`](https://github.com/benpueschel/gritty/commit/306354ede7b68092d4ac505fc30038ec6bab4cde) - list configured remotes *(commit by [@benpueschel](https://github.com/benpueschel))*

### :recycle: Refactors
- [`2239a46`](https://github.com/benpueschel/gritty/commit/2239a46a89e2d60b2daa4d5398b3ef7ccfd59eaa) - **log**: remove unused, empty macro *(commit by [@benpueschel](https://github.com/benpueschel))*

## [v0.2.0] - 2024-07-08
### :sparkles: New Features
- [`85677d5`](https://github.com/benpueschel/gritty/commit/85677d50eb0fad4da534880602cc61bbbfddc7f6) - interactive configuration *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`a312733`](https://github.com/benpueschel/gritty/commit/a312733b37a2afcafae3db662d665971d6879130) - clone remote repo *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`8e4da46`](https://github.com/benpueschel/gritty/commit/8e4da465a76e32c2fa25cc7b09dbea0a50e3cdc6) - **args**: add colors to help menu *(commit by [@benpueschel](https://github.com/benpueschel))*

### :bug: Bug Fixes
- [`df6d042`](https://github.com/benpueschel/gritty/commit/df6d042d83897620475da84e6ef37d434418e341) - **config**: create secrets file if it doesn't exist *(commit by [@benpueschel](https://github.com/benpueschel))*

### :recycle: Refactors
- [`3377df5`](https://github.com/benpueschel/gritty/commit/3377df5936df5675542a8b5085d9d33a58f69937) - **commands**: add get_input method *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`1fa1e32`](https://github.com/benpueschel/gritty/commit/1fa1e322f91c2d8100b20fb6c18a2f6edfd47099) - **args**: move from structopt to clap *(commit by [@benpueschel](https://github.com/benpueschel))*

## [v0.1.1] - 2024-07-07
### :recycle: Refactors
- [`a92eae9`](https://github.com/benpueschel/gritty/commit/a92eae97b9c8e76b2a16bb8f704ca50d79583b46) - **remote**: move clone to top-level trait *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`b3ec28d`](https://github.com/benpueschel/gritty/commit/b3ec28d58b72429af65c0aea36f06da7c821bfbb) - **remote**: add static COMMIT_COUNT *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`fb49b12`](https://github.com/benpueschel/gritty/commit/fb49b1240ca9ba4c848c5249a2008ec430ee495e) - **github**: code cleanup *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`9c278d7`](https://github.com/benpueschel/gritty/commit/9c278d7a7c00be70b9772c6b3ba4de7d8fc955a9) - unified error type *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`f9483ea`](https://github.com/benpueschel/gritty/commit/f9483ea2ce0d90507999682da9c97526dfa420eb) - **main**: cleanup main function *(commit by [@benpueschel](https://github.com/benpueschel))*

## [v0.1.0] - 2024-07-06
### :boom: BREAKING CHANGES
- due to [`bb39c74`](https://github.com/benpueschel/gritty/commit/bb39c74e715930001b594b0f51281b4343047fac) - ask for confirmation on delete *(commit by [@benpueschel](https://github.com/benpueschel))*:

  ask for confirmation on delete

- due to [`d80983a`](https://github.com/benpueschel/gritty/commit/d80983aff68d077300f2154e26d02602c2efeac4) - change config path *(commit by [@benpueschel](https://github.com/benpueschel))*:

  Old configs located at ~/.config/gitrc-rs/ will need to
  be relocated to ~/.config/gritty/

- due to [`50b2fb4`](https://github.com/benpueschel/gritty/commit/50b2fb4a02610a5789c36664db4b841327c69f2e) - add option to clone repo on creation *(commit by [@benpueschel](https://github.com/benpueschel))*:

  Configs now need a "clone_protocol" option for each
  remote.


### :sparkles: New Features
- [`614fc97`](https://github.com/benpueschel/gritty/commit/614fc9713c6991f7c958007e81d7d1eaa27ec5a3) - cli remote authentication *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`2e9d974`](https://github.com/benpueschel/gritty/commit/2e9d9747cc212a2180411a177c78d88c8b5130a5) - use keyring secret by default *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`bb39c74`](https://github.com/benpueschel/gritty/commit/bb39c74e715930001b594b0f51281b4343047fac) - ask for confirmation on delete *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`f0401ed`](https://github.com/benpueschel/gritty/commit/f0401edad5365710b87c2c40f6b419534158a93b) - list repositories *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`e7130bb`](https://github.com/benpueschel/gritty/commit/e7130bbf93e8f6731d57b684f4debf463e6a362d) - color highlighting *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`dc06dfd`](https://github.com/benpueschel/gritty/commit/dc06dfd4eec03624f1b88dcf53736c2e92501a74) - make keyring an optional feature *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`d80983a`](https://github.com/benpueschel/gritty/commit/d80983aff68d077300f2154e26d02602c2efeac4) - change config path *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`9be661b`](https://github.com/benpueschel/gritty/commit/9be661b03b5f8c693703a3cb6ae14b75bcf77cb3) - add command to create default config *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`50b2fb4`](https://github.com/benpueschel/gritty/commit/50b2fb4a02610a5789c36664db4b841327c69f2e) - add option to clone repo on creation *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`173cdcc`](https://github.com/benpueschel/gritty/commit/173cdcce75ce410e69fa65f39a477a116fab48dc) - implement repo initialization *(commit by [@benpueschel](https://github.com/benpueschel))*

### :bug: Bug Fixes
- [`e0124bd`](https://github.com/benpueschel/gritty/commit/e0124bd4e5eb0da9a51cd16e7b8b5d308ceff12f) - **config**: only save when using plaintext auth *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`ac13c93`](https://github.com/benpueschel/gritty/commit/ac13c9356b3e3ae2ebfeb687f881ba9c0e1cd4e9) - **github**: handle empty repo commit infos *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`ac065cd`](https://github.com/benpueschel/gritty/commit/ac065cd8ab97432369800f21713312fe0e75023b) - **gitea**: handle empty repo commit infos *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`71cffcd`](https://github.com/benpueschel/gritty/commit/71cffcd8aa551a3dc6b4afa39e4927297a38e49b) - don't print full error when deleting repo *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`db82eec`](https://github.com/benpueschel/gritty/commit/db82eec5d5b921fd06026bb0e45b7aa54b9914bd) - don't print default config on auth *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`0b78193`](https://github.com/benpueschel/gritty/commit/0b78193d467c5cd89c37dedfdf374d247af2c2b5) - git clone ssh url *(commit by [@benpueschel](https://github.com/benpueschel))*

## [v0.0.1] - 2024-07-06
### :boom: BREAKING CHANGES
- due to [`419df8e`](https://github.com/benpueschel/gritty/commit/419df8e666a03e9a669b38bda70d36b726eeb714) - simplify plaintext auth config *(commit by [@benpueschel](https://github.com/benpueschel))*:

  Old configs are not valid anymore and will not work.


### :sparkles: New Features
- [`2858ecd`](https://github.com/benpueschel/gritty/commit/2858ecd00a0d2298dd16044f5a4eb3a423b7a861) - add git remote trait *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`010ca24`](https://github.com/benpueschel/gritty/commit/010ca24b705834cf3fd8a866fb66ad8bc866c20a) - add github remote *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`892bef4`](https://github.com/benpueschel/gritty/commit/892bef40d295c05ba5ad0035b9076f4f8ce7fc8e) - add gitea impl *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`f3f14c2`](https://github.com/benpueschel/gritty/commit/f3f14c287134f22181487ba7f891a9891dc6c169) - basic toml config *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`419df8e`](https://github.com/benpueschel/gritty/commit/419df8e666a03e9a669b38bda70d36b726eeb714) - simplify plaintext auth config *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`74519cb`](https://github.com/benpueschel/gritty/commit/74519cbfce79c3764fc358d57f6032561ac23ec9) - add secrets file config option *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`84c076e`](https://github.com/benpueschel/gritty/commit/84c076e8371011aff075f890b415174ebdeb9000) - add platform-specific keyring config option *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`24fc4d3`](https://github.com/benpueschel/gritty/commit/24fc4d34d1c278cd26eefc3665e79d2159324194) - add config option to store secrets *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`6520bb2`](https://github.com/benpueschel/gritty/commit/6520bb2c6bb1fefe791ba19b248ebb709e89dd69) - add option to save config *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`e735c17`](https://github.com/benpueschel/gritty/commit/e735c17445d1ce5ed430aab2c181d8f4f7b20daf) - parse args for an actual working version! *(commit by [@benpueschel](https://github.com/benpueschel))*

### :bug: Bug Fixes
- [`b073626`](https://github.com/benpueschel/gritty/commit/b0736262ac0060c39a1bed867ef3dfca9acda320) - remove log.txt *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`4ee6a37`](https://github.com/benpueschel/gritty/commit/4ee6a372b5177bab67dca7b24187fd64b9defdce) - repo commit fetching *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`9b67aef`](https://github.com/benpueschel/gritty/commit/9b67aefcc23cbd76b73cee2cad792b161c72e932) - correct repo clone url *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`51086d5`](https://github.com/benpueschel/gritty/commit/51086d5b5821747e83bfe58bd3689c915ae4fc22) - set version to 0.0.1 *(commit by [@benpueschel](https://github.com/benpueschel))*

### :recycle: Refactors
- [`afe5b09`](https://github.com/benpueschel/gritty/commit/afe5b09690a63e946042cc37be2369ad08863adc) - move map_error to remote/mod.rs *(commit by [@benpueschel](https://github.com/benpueschel))*

[v0.0.1]: https://github.com/benpueschel/gritty/compare/v0.0.0...v0.0.1
[v0.1.0]: https://github.com/benpueschel/gritty/compare/v0.0.1...v0.1.0
[v0.1.1]: https://github.com/benpueschel/gritty/compare/v0.1.0...v0.1.1
[v0.2.0]: https://github.com/benpueschel/gritty/compare/v0.1.1...v0.2.0
[v0.3.0]: https://github.com/benpueschel/gritty/compare/v0.2.0...v0.3.0
[v0.4.0]: https://github.com/benpueschel/gritty/compare/v0.3.0...v0.4.0
[v0.5.0]: https://github.com/benpueschel/gritty/compare/v0.4.0...v0.5.0
[v0.6.0]: https://github.com/benpueschel/gritty/compare/v0.5.0...v0.6.0
[v0.7.0]: https://github.com/benpueschel/gritty/compare/v0.6.0...v0.7.0
[v0.8.0]: https://github.com/benpueschel/gritty/compare/v0.7.0...v0.8.0
[v0.8.1]: https://github.com/benpueschel/gritty/compare/v0.8.0...v0.8.1
[v1.0.0]: https://github.com/benpueschel/gritty/compare/v0.8.1...v1.0.0
