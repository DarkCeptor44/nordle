# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -
## [v0.2.3](https://github.com/DarkCeptor44/nordle/compare/bd355a4424b57dc6deb6340e81b924c9dec50f53..v0.2.3) - 2026-04-23
#### Bug Fixes
- (**api**) prevent panic on invalid guess length - ([5ee61c6](https://github.com/DarkCeptor44/nordle/commit/5ee61c6c7a10fe46d12063baf7d574ec2f0a8f31)) - DarkCeptor44
- (**backend**) add check to avoid processing a guess if it's not valid (in the allowed list) or not a 5-letter word - ([bd355a4](https://github.com/DarkCeptor44/nordle/commit/bd355a4424b57dc6deb6340e81b924c9dec50f53)) - DarkCeptor44

- - -

## [v0.2.2](https://github.com/DarkCeptor44/nordle/compare/fbd9ccba5cab9a937b69640b56134081b7f216cc..v0.2.2) - 2026-04-20
#### Bug Fixes
- (**ci**) use non-slim version of rust image - ([fbd9ccb](https://github.com/DarkCeptor44/nordle/commit/fbd9ccba5cab9a937b69640b56134081b7f216cc)) - DarkCeptor44

- - -

## [v0.2.1](https://github.com/DarkCeptor44/nordle/compare/a5b1a35fcdd38c4ddd8a65893ecb6d2c3bda20d6..v0.2.1) - 2026-04-20
#### Bug Fixes
- (**ci**) pin cargo-chef and rust version in docker - ([a5b1a35](https://github.com/DarkCeptor44/nordle/commit/a5b1a35fcdd38c4ddd8a65893ecb6d2c3bda20d6)) - DarkCeptor44

- - -

## [v0.2.0](https://github.com/DarkCeptor44/nordle/compare/791f214b061e08bc11dc6e1be981d7c0f8b6d935..v0.2.0) - 2026-04-20
#### Features
- (**backend**) change default NORDLE_CACHE_SIZE to 100 - ([8dd5999](https://github.com/DarkCeptor44/nordle/commit/8dd599918b6160108af28a686995f892853ccc3a)) - DarkCeptor44
- (**frontend**) add reset function to allow trying another game without reloading; chooseWord calls it first before changing the word - ([d48121f](https://github.com/DarkCeptor44/nordle/commit/d48121f8c22b170315c7924916415a5b022380a1)) - DarkCeptor44
- ![BREAKING](https://img.shields.io/badge/BREAKING-red) guess endpoint now sends the target word is 6 guesses were made and the game was won - ([6bc35c0](https://github.com/DarkCeptor44/nordle/commit/6bc35c07f1682d224fcc0eb148d54773dde20520)) - DarkCeptor44
- add console function to allow choosing whatever word you want - ([e787ea3](https://github.com/DarkCeptor44/nordle/commit/e787ea3c83c8bd417144f964122dbc725c35b086)) - DarkCeptor44
- add optimizations to manifest - ([791f214](https://github.com/DarkCeptor44/nordle/commit/791f214b061e08bc11dc6e1be981d7c0f8b6d935)) - DarkCeptor44
#### Bug Fixes
- (**backend**) make sure LRU cache uses the cache size instead of a hardcoded one - ([2152058](https://github.com/DarkCeptor44/nordle/commit/2152058a193bdcf4f005c3e4353d5585d66b3763)) - DarkCeptor44
- (**frontend**) flipping animation on guess now works - ([fb7a120](https://github.com/DarkCeptor44/nordle/commit/fb7a1206edd4963e715086a9e65f9325258f550b)) - DarkCeptor44
- (**frontend**) typo in the settings input maxlength - ([41e1b1c](https://github.com/DarkCeptor44/nordle/commit/41e1b1c01de98072f7952de7477f1122300ee9b7)) - DarkCeptor44
- (**frontend**) row shaking animation now works - ([fd7ed82](https://github.com/DarkCeptor44/nordle/commit/fd7ed82b3f921a2e4c9a0a3cb05048a21cef1c6e)) - DarkCeptor44
- (**frontend**) reset keyboard colors when reset function is called - ([8a9ae0d](https://github.com/DarkCeptor44/nordle/commit/8a9ae0d630c1868f9367cd1927152471ef68a708)) - DarkCeptor44
- make sure index.html triggers a rebuild so it doesnt always serve a cached index file - ([dc30325](https://github.com/DarkCeptor44/nordle/commit/dc3032567e87156734107ff2e1990cb7808a0461)) - DarkCeptor44
#### Continuous Integration
- upgrade docker actions to latest versions and add qemu step - ([0c6f4e8](https://github.com/DarkCeptor44/nordle/commit/0c6f4e819d7bddb3cb39de9b416ea0e126bd02f1)) - DarkCeptor44
#### Refactors
- (**frontend**) move keyframes around in css - ([3ca027a](https://github.com/DarkCeptor44/nordle/commit/3ca027a6551b1a4bae2a5b5a2b07c0c87d5334fd)) - DarkCeptor44
- (**frontend**) bypass getting new ID when choosing a word - ([406c314](https://github.com/DarkCeptor44/nordle/commit/406c31496a5fd3ef37e5f75f5da91f7bf89f2999)) - DarkCeptor44
#### Style
- (**frontend**) make toast dark mode - ([ce88034](https://github.com/DarkCeptor44/nordle/commit/ce88034b0bf7843891abac8ad53502610d66459a)) - DarkCeptor44
- add modal for game over screen and settings - ([84dfa0a](https://github.com/DarkCeptor44/nordle/commit/84dfa0affb32a51f6a75d59b93ea6d4310c34af2)) - DarkCeptor44
- improve responsiveness - ([4671491](https://github.com/DarkCeptor44/nordle/commit/4671491bc0d9bc9f8206844c4cdefb0c04e36cd1)) - DarkCeptor44
- prepare for game over modal - ([547f8d5](https://github.com/DarkCeptor44/nordle/commit/547f8d5a488cd0ac6e2bf8befd7b4c0a76f6d948)) - DarkCeptor44

- - -

## [0.1.0](https://github.com/DarkCeptor44/nordle/compare/c8172393addb47edf24ad30b4a54f3bbf71e1a84..0.1.0) - 2026-04-20
#### Features
- (**backend**) change default NORDLE_CACHE_SIZE to 100 - ([8dd5999](https://github.com/DarkCeptor44/nordle/commit/8dd599918b6160108af28a686995f892853ccc3a)) - DarkCeptor44
- (**frontend**) add reset function to allow trying another game without reloading; chooseWord calls it first before changing the word - ([d48121f](https://github.com/DarkCeptor44/nordle/commit/d48121f8c22b170315c7924916415a5b022380a1)) - DarkCeptor44
- ![BREAKING](https://img.shields.io/badge/BREAKING-red) guess endpoint now sends the target word is 6 guesses were made and the game was won - ([6bc35c0](https://github.com/DarkCeptor44/nordle/commit/6bc35c07f1682d224fcc0eb148d54773dde20520)) - DarkCeptor44
- add console function to allow choosing whatever word you want - ([e787ea3](https://github.com/DarkCeptor44/nordle/commit/e787ea3c83c8bd417144f964122dbc725c35b086)) - DarkCeptor44
- add optimizations to manifest - ([791f214](https://github.com/DarkCeptor44/nordle/commit/791f214b061e08bc11dc6e1be981d7c0f8b6d935)) - DarkCeptor44
- add workflow file to build and push images to GHCR - ([ca89a7b](https://github.com/DarkCeptor44/nordle/commit/ca89a7bea5c6230a34bedb3fd4475c8e042cb00b)) - DarkCeptor44
- Dockerfile - ([c4c4832](https://github.com/DarkCeptor44/nordle/commit/c4c48324e802f6e0552489cdc0ea208c4efb87e2)) - DarkCeptor44
- add license - ([709db72](https://github.com/DarkCeptor44/nordle/commit/709db72c3999ed3cf96f65c198e022202a337c00)) - DarkCeptor44
#### Bug Fixes
- (**backend**) make sure LRU cache uses the cache size instead of a hardcoded one - ([2152058](https://github.com/DarkCeptor44/nordle/commit/2152058a193bdcf4f005c3e4353d5585d66b3763)) - DarkCeptor44
- (**frontend**) flipping animation on guess now works - ([fb7a120](https://github.com/DarkCeptor44/nordle/commit/fb7a1206edd4963e715086a9e65f9325258f550b)) - DarkCeptor44
- (**frontend**) typo in the settings input maxlength - ([41e1b1c](https://github.com/DarkCeptor44/nordle/commit/41e1b1c01de98072f7952de7477f1122300ee9b7)) - DarkCeptor44
- (**frontend**) row shaking animation now works - ([fd7ed82](https://github.com/DarkCeptor44/nordle/commit/fd7ed82b3f921a2e4c9a0a3cb05048a21cef1c6e)) - DarkCeptor44
- (**frontend**) reset keyboard colors when reset function is called - ([8a9ae0d](https://github.com/DarkCeptor44/nordle/commit/8a9ae0d630c1868f9367cd1927152471ef68a708)) - DarkCeptor44
- make sure index.html triggers a rebuild so it doesnt always serve a cached index file - ([dc30325](https://github.com/DarkCeptor44/nordle/commit/dc3032567e87156734107ff2e1990cb7808a0461)) - DarkCeptor44
- rename username to lowercase in workflow - ([99177be](https://github.com/DarkCeptor44/nordle/commit/99177bed62266d457f74b6ba93f8d2a8566f1654)) - DarkCeptor44
- get rid of env helpers and rely solely on clap for environment checking - ([59978c6](https://github.com/DarkCeptor44/nordle/commit/59978c6f316c932fa695444899222150b21c490a)) - DarkCeptor44
#### Continuous Integration
- upgrade docker actions to latest versions and add qemu step - ([0c6f4e8](https://github.com/DarkCeptor44/nordle/commit/0c6f4e819d7bddb3cb39de9b416ea0e126bd02f1)) - DarkCeptor44
#### Refactors
- (**frontend**) move keyframes around in css - ([3ca027a](https://github.com/DarkCeptor44/nordle/commit/3ca027a6551b1a4bae2a5b5a2b07c0c87d5334fd)) - DarkCeptor44
- (**frontend**) bypass getting new ID when choosing a word - ([406c314](https://github.com/DarkCeptor44/nordle/commit/406c31496a5fd3ef37e5f75f5da91f7bf89f2999)) - DarkCeptor44
- collapse some if-let chains since MSRV is now 1.88 - ([91d9cdc](https://github.com/DarkCeptor44/nordle/commit/91d9cdc598db0867ae45ec52cc84c743600ac9fa)) - DarkCeptor44
#### Style
- (**frontend**) make toast dark mode - ([ce88034](https://github.com/DarkCeptor44/nordle/commit/ce88034b0bf7843891abac8ad53502610d66459a)) - DarkCeptor44
- add modal for game over screen and settings - ([84dfa0a](https://github.com/DarkCeptor44/nordle/commit/84dfa0affb32a51f6a75d59b93ea6d4310c34af2)) - DarkCeptor44
- improve responsiveness - ([4671491](https://github.com/DarkCeptor44/nordle/commit/4671491bc0d9bc9f8206844c4cdefb0c04e36cd1)) - DarkCeptor44
- prepare for game over modal - ([547f8d5](https://github.com/DarkCeptor44/nordle/commit/547f8d5a488cd0ac6e2bf8befd7b4c0a76f6d948)) - DarkCeptor44

- - -

Changelog generated by [cocogitto](https://github.com/cocogitto/cocogitto).