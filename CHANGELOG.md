# Changelog

## [0.2.0](https://github.com/zms-dev/configuratarr/compare/v0.1.1...v0.2.0) (2026-07-11)


### Features

* add autobrr v1 support ([a01c6ad](https://github.com/zms-dev/configuratarr/commit/a01c6adb7a884a8bfaffe876d37d6f599016a868))
* add bazarr v1 support ([301fe1e](https://github.com/zms-dev/configuratarr/commit/301fe1eca3d7f03b10182ebb7ba918c64ac75e62))
* add central registry for services ([7528fc1](https://github.com/zms-dev/configuratarr/commit/7528fc158c79185e6000eb882c61a293385fe908))
* add jellyfin v11 support ([76393ae](https://github.com/zms-dev/configuratarr/commit/76393ae60672c763141b24eeaaea0b874643e0ba))
* add lidarr support ([2083744](https://github.com/zms-dev/configuratarr/commit/2083744d180ccf23642a0d4caaf03066eaaa7c8e))
* add prowlarr support ([882e339](https://github.com/zms-dev/configuratarr/commit/882e339e828c3ca5b58008888b2fb3918a15fdc8))
* add sonarr-v3 support ([88ebee3](https://github.com/zms-dev/configuratarr/commit/88ebee36a42054f321274bdbfe4b9d54c88f0443))
* additional autobrr resources ([38c32d2](https://github.com/zms-dev/configuratarr/commit/38c32d2eda7e6deb12a81d2b8fe68974d662a4aa))
* **autobrr:** full field coverage + dedup profiles; fix core ref recursion ([a291728](https://github.com/zms-dev/configuratarr/commit/a2917281468bfb9cd07e1bc892f69d1666126d10))
* **autobrr:** full notification/client/external-filter coverage vs source ([b980cc1](https://github.com/zms-dev/configuratarr/commit/b980cc1c8ff8362ac9d3f5888ef54db12b4598c3))
* **autobrr:** model lists and feeds ([135b29e](https://github.com/zms-dev/configuratarr/commit/135b29e6d3fce5a8ed46295ecdd28353c369cdb5))
* **bazarr:** expand settings coverage — default profiles, auth/proxy type, new sections ([f1461fa](https://github.com/zms-dev/configuratarr/commit/f1461fae12b10cc269dbbca4bf45ac2c17b8dad2))
* **core:** wire(null)/wire(int) codec options; codec-drive bazarr custom sync ([55fac61](https://github.com/zms-dev/configuratarr/commit/55fac61bb1dea24bbc85d9cba7eef5adcdfbd355))
* expose waitForHealthy for nix ([a4e4bf7](https://github.com/zms-dev/configuratarr/commit/a4e4bf7d26ad51ac72abbab76de0a0ce5bd55bbe))
* **lazylibrarian:** add LazyLibrarian v1 service + query-apikey auth seam ([cefba97](https://github.com/zms-dev/configuratarr/commit/cefba9742fd62c466174ae0cfcda24b5e05d15f2))


### Bug Fixes

* **autobrr:** don't fetch list titles in e2e (hermetic CI VM has no DNS) ([a71ed6d](https://github.com/zms-dev/configuratarr/commit/a71ed6dbfda5cec1f78633ec408c7f262ec61ec1))
* **autobrr:** reconcile filter actions by identity, prune duplicates ([d8565e9](https://github.com/zms-dev/configuratarr/commit/d8565e9db87f7e95a17b8fa43867a4ead248ed9e))
* **core:** resolve ${ref} to custom-sync resources + nested-FK apply ordering ([facbb92](https://github.com/zms-dev/configuratarr/commit/facbb92f974e41f4a431fbe262a37e974befda7d))
* correct MediaBrowser notification api_key wire name ([4c804ef](https://github.com/zms-dev/configuratarr/commit/4c804ef8e5dea6352fe6c69a9c7aa8cc6b353138)), closes [#31](https://github.com/zms-dev/configuratarr/issues/31)
* docs and module packages ([759ea77](https://github.com/zms-dev/configuratarr/commit/759ea7701a1100f19038d6d0d682765cce193859))
* flag ordering ([21e197e](https://github.com/zms-dev/configuratarr/commit/21e197e480398838b3c10dbff5d28295654a14d9))
* **http:** drive reqwest TLS with bundled webpki roots ([fea277f](https://github.com/zms-dev/configuratarr/commit/fea277f5e37da43e06f77a1741c8aba36dc9bece))
* **lazylibrarian:** green the e2e nixosTest for CI ([9896d30](https://github.com/zms-dev/configuratarr/commit/9896d3061988a7727ecb80256ac4b837b34b8605))
* tools and skills ([36f039b](https://github.com/zms-dev/configuratarr/commit/36f039b150f734889241fce6ccc8227e4daa69b3))

## [0.1.1](https://github.com/zms-dev/configuratarr/compare/v0.1.0...v0.1.1) (2026-06-16)


### Bug Fixes

* add aarch64-linux to extra-platforms in release-please workflow ([e90d322](https://github.com/zms-dev/configuratarr/commit/e90d32200bb4f92b41c38e769d74759cd69aa6ed))

## 0.1.0 (2026-06-16)


### Features

* add Cachix binary cache config to flake.nix ([3a40dfa](https://github.com/zms-dev/configuratarr/commit/3a40dfaecd0f1d693b733a531822ef0080e10a3a))
* configure release-please with multi-platform assets build ([f978d68](https://github.com/zms-dev/configuratarr/commit/f978d68fdba2f2c5b7f59262ba1a8c58f1dc5095))
* implement structured submodules for options and apply nixfmt ([9a3a119](https://github.com/zms-dev/configuratarr/commit/9a3a11936230e52306a0efe3115b9a65194b1efb))
