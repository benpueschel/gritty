# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [v0.0.1] - 2024-07-06
### :boom: BREAKING CHANGES
- due to [`419df8e`](https://github.com/benpueschel/gitrc-rs/commit/419df8e666a03e9a669b38bda70d36b726eeb714) - simplify plaintext auth config *(commit by [@benpueschel](https://github.com/benpueschel))*:

  Old configs are not valid anymore and will not work.


### :sparkles: New Features
- [`2858ecd`](https://github.com/benpueschel/gitrc-rs/commit/2858ecd00a0d2298dd16044f5a4eb3a423b7a861) - add git remote trait *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`010ca24`](https://github.com/benpueschel/gitrc-rs/commit/010ca24b705834cf3fd8a866fb66ad8bc866c20a) - add github remote *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`892bef4`](https://github.com/benpueschel/gitrc-rs/commit/892bef40d295c05ba5ad0035b9076f4f8ce7fc8e) - add gitea impl *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`f3f14c2`](https://github.com/benpueschel/gitrc-rs/commit/f3f14c287134f22181487ba7f891a9891dc6c169) - basic toml config *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`419df8e`](https://github.com/benpueschel/gitrc-rs/commit/419df8e666a03e9a669b38bda70d36b726eeb714) - simplify plaintext auth config *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`74519cb`](https://github.com/benpueschel/gitrc-rs/commit/74519cbfce79c3764fc358d57f6032561ac23ec9) - add secrets file config option *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`84c076e`](https://github.com/benpueschel/gitrc-rs/commit/84c076e8371011aff075f890b415174ebdeb9000) - add platform-specific keyring config option *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`24fc4d3`](https://github.com/benpueschel/gitrc-rs/commit/24fc4d34d1c278cd26eefc3665e79d2159324194) - add config option to store secrets *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`6520bb2`](https://github.com/benpueschel/gitrc-rs/commit/6520bb2c6bb1fefe791ba19b248ebb709e89dd69) - add option to save config *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`e735c17`](https://github.com/benpueschel/gitrc-rs/commit/e735c17445d1ce5ed430aab2c181d8f4f7b20daf) - parse args for an actual working version! *(commit by [@benpueschel](https://github.com/benpueschel))*

### :bug: Bug Fixes
- [`b073626`](https://github.com/benpueschel/gitrc-rs/commit/b0736262ac0060c39a1bed867ef3dfca9acda320) - remove log.txt *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`4ee6a37`](https://github.com/benpueschel/gitrc-rs/commit/4ee6a372b5177bab67dca7b24187fd64b9defdce) - repo commit fetching *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`9b67aef`](https://github.com/benpueschel/gitrc-rs/commit/9b67aefcc23cbd76b73cee2cad792b161c72e932) - correct repo clone url *(commit by [@benpueschel](https://github.com/benpueschel))*
- [`51086d5`](https://github.com/benpueschel/gitrc-rs/commit/51086d5b5821747e83bfe58bd3689c915ae4fc22) - set version to 0.0.1 *(commit by [@benpueschel](https://github.com/benpueschel))*

### :recycle: Refactors
- [`afe5b09`](https://github.com/benpueschel/gitrc-rs/commit/afe5b09690a63e946042cc37be2369ad08863adc) - move map_error to remote/mod.rs *(commit by [@benpueschel](https://github.com/benpueschel))*

[v0.0.1]: https://github.com/benpueschel/gitrc-rs/compare/v0.0.0...v0.0.1
