# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [v0.1.1] - 2024-07-07
### :recycle: Refactors
- [`a92eae9`](https://github.com/benpueschel/gritty/commit/a92eae97b9c8e76b2a16bb8f704ca50d79583b46) - **remote**: move clone to top-level trait *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`b3ec28d`](https://github.com/benpueschel/gritty/commit/b3ec28d58b72429af65c0aea36f06da7c821bfbb) - **remote**: add static COMMIT_COUNT *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`fb49b12`](https://github.com/benpueschel/gritty/commit/fb49b1240ca9ba4c848c5249a2008ec430ee495e) - **github**: code cleanup *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`9c278d7`](https://github.com/benpueschel/gritty/commit/9c278d7a7c00be70b9772c6b3ba4de7d8fc955a9) - unified error type *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`f9483ea`](https://github.com/benpueschel/gritty/commit/f9483ea2ce0d90507999682da9c97526dfa420eb) - **main**: cleanup main function *(commit by [@benpueschel](https://github.com/benpueschel))*

### :wrench: Chores
- [`90fa380`](https://github.com/benpueschel/gritty/commit/90fa38069ee97fe161970ffe1ff5eb65a1e17c9a) - **main**: release v0.1.0 *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`bdd90b6`](https://github.com/benpueschel/gritty/commit/bdd90b60ec82a6226a1d891dff77139751a205e7) - bump version to 0.1.1 *(commit by [@benpueschel](https://github.com/benpueschel))*


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

### :wrench: Chores
- [`241fe0c`](https://github.com/benpueschel/gritty/commit/241fe0c7df3b77261439999a2d9317387a474b7d) - **main**: release v0.0.1 *(commit by [@benpueschel](https://github.com/benpueschel))*


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
