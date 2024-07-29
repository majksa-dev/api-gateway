# Changelog

## [0.4.1](https://github.com/majksa-dev/api-gateway/compare/v0.4.0...v0.4.1) (2024-07-29)


### Bug Fixes

* **deps:** bump serde_json in the dependencies group ([ff3413f](https://github.com/majksa-dev/api-gateway/commit/ff3413fe766a5f5a6480a9aae6a7b68df80732b6))
* **deps:** bump version ([ef528ac](https://github.com/majksa-dev/api-gateway/commit/ef528acfd28cb052e0997c4e1b43276bc541a996))

## [0.4.0](https://github.com/majksa-dev/api-gateway/compare/v0.3.5...v0.4.0) (2024-07-28)


### Features

* **auth:** implement endpoint based auth ([a1714cd](https://github.com/majksa-dev/api-gateway/commit/a1714cdd060481dddcbf2434971dee77482806f3))

## [0.3.5](https://github.com/majksa-dev/api-gateway/compare/v0.3.4...v0.3.5) (2024-07-28)


### Bug Fixes

* **deps:** bump the dependencies group with 5 updates ([ce3240a](https://github.com/majksa-dev/api-gateway/commit/ce3240ab3ae5f5f655d7da40392534c97c7d15c8))
* update api usage of gateway tcp upstream ([8a40926](https://github.com/majksa-dev/api-gateway/commit/8a40926856b9924a934ab7cc75417fde565e27fc))

## [0.3.4](https://github.com/majksa-dev/api-gateway/compare/v0.3.3...v0.3.4) (2024-07-04)


### Bug Fixes

* **deps:** bump gateway from 0.5.4 to 0.6.0 ([ba87f89](https://github.com/majksa-dev/api-gateway/commit/ba87f8927b3177cf24190e4d919f99b2ccf157ee))

## [0.3.3](https://github.com/majksa-dev/api-gateway/compare/v0.3.2...v0.3.3) (2024-07-04)


### Bug Fixes

* **deps:** bump serde_json from 1.0.118 to 1.0.120 ([87530a3](https://github.com/majksa-dev/api-gateway/commit/87530a3a1ffd6f18086eeb1c8743a00ab27ab80b))

## [0.3.2](https://github.com/majksa-dev/api-gateway/compare/v0.3.1...v0.3.2) (2024-06-28)


### Bug Fixes

* correctly setup tests ([2f44baa](https://github.com/majksa-dev/api-gateway/commit/2f44baab9fdeaca7843acaf8a6d1115efd4aab7b))
* **deps:** bump testing-utils from 0.1.4 to 0.1.5 ([60eed0c](https://github.com/majksa-dev/api-gateway/commit/60eed0ccfcc21906ef8f0d6c96ca857903fe4f9e))
* remove lint pr action ([879c3ee](https://github.com/majksa-dev/api-gateway/commit/879c3ee4e8418fa823acb212f527eafbeec5a39c))
* run tests setup with --release ([1d088ae](https://github.com/majksa-dev/api-gateway/commit/1d088aec40b0487ca0610a049644e163b89797b8))

## [0.3.1](https://github.com/majksa-dev/api-gateway/compare/v0.3.0...v0.3.1) (2024-06-28)


### Bug Fixes

* deploy only to linux/amd64 platform ([ea15af7](https://github.com/majksa-dev/api-gateway/commit/ea15af7445ca05af3f57fe0c68325e0dbb5caba1))

## [0.3.0](https://github.com/majksa-dev/api-gateway/compare/v0.2.0...v0.3.0) (2024-06-28)


### Features

* add auth middlewares, implement gateway config using builders ([5e0e38c](https://github.com/majksa-dev/api-gateway/commit/5e0e38c872ade1aecff5ca24be44160f9cebc13c))


### Bug Fixes

* **deps:** bump serde_json from 1.0.117 to 1.0.118 ([4ff69ab](https://github.com/majksa-dev/api-gateway/commit/4ff69ab6e48860944904ee21e22bc570e74c7a71))
* **deps:** upgrade gateway from 0.5.3 to 0.5.4 ([9a8ba4a](https://github.com/majksa-dev/api-gateway/commit/9a8ba4acbda069d3bc5ee8fe65763fa13d721838))
* **jwt:** setup auth config correctly ([5560f18](https://github.com/majksa-dev/api-gateway/commit/5560f1841cec9938195bda7d5990e03850973379))
* run cargo build before tests ([3a01cd7](https://github.com/majksa-dev/api-gateway/commit/3a01cd71c2415094fbf75236024480dd6c63bd6b))

## [0.2.0](https://github.com/majksa-dev/api-gateway/compare/v0.1.5...v0.2.0) (2024-06-23)


### Features

* add caching middleware ([8d34046](https://github.com/majksa-dev/api-gateway/commit/8d3404613618d1abfaf7ded6b04352b5627575cb))
* implement custom server ([457076b](https://github.com/majksa-dev/api-gateway/commit/457076b8d92d4a2e3df535d927abb335ea2fad1b))


### Bug Fixes

* **deps:** bump async-trait from 0.1.78 to 0.1.80 ([cf8dd78](https://github.com/majksa-dev/api-gateway/commit/cf8dd78f0abbe40ecfc46603411ffb8e296121e5))
* **deps:** bump chrono from 0.4.35 to 0.4.38 ([40c639f](https://github.com/majksa-dev/api-gateway/commit/40c639fcc09316ad5c8d37197b4098979a30c6e4))
* **deps:** bump redis from 0.25.2 to 0.25.4 ([0afa3a8](https://github.com/majksa-dev/api-gateway/commit/0afa3a8f1c6a505899ceaca61706c0c9d02ade8f))
* **deps:** bump regex from 1.10.3 to 1.10.5 ([5989852](https://github.com/majksa-dev/api-gateway/commit/5989852aa7bc1ad2f76406cbf4af59e68dd03126))
* **deps:** bump serde_json from 1.0.114 to 1.0.117 ([f013305](https://github.com/majksa-dev/api-gateway/commit/f013305b3c8c5968cfddb1c61172c79f8dba9324))

## [0.1.5](https://github.com/majksa-dev/api-gateway/compare/v0.1.4...v0.1.5) (2024-03-19)


### Bug Fixes

* limit platforms ([6fe2528](https://github.com/majksa-dev/api-gateway/commit/6fe2528e497fe604379c8aefaa4f9ccfdba7852a))

## [0.1.4](https://github.com/majksa-dev/api-gateway/compare/v0.1.3...v0.1.4) (2024-03-19)


### Bug Fixes

* project name ([fb4a6f5](https://github.com/majksa-dev/api-gateway/commit/fb4a6f521468043c01b405bcad36c2784aacd1d1))

## [0.1.3](https://github.com/majksa-dev/api-gateway/compare/v0.1.2...v0.1.3) (2024-03-19)


### Bug Fixes

* docker build ([1996921](https://github.com/majksa-dev/api-gateway/commit/199692104f2d4b1017d7ca48c988586c50fa0683))

## [0.1.2](https://github.com/majksa-dev/api-gateway/compare/v0.1.1...v0.1.2) (2024-03-19)


### Bug Fixes

* building the app ([afec1ae](https://github.com/majksa-dev/api-gateway/commit/afec1aeeb067325d9021434d04d434c114bb833a))

## [0.1.1](https://github.com/majksa-dev/api-gateway/compare/v0.1.0...v0.1.1) (2024-03-19)


### Bug Fixes

* allow writing packages ([90fe9d3](https://github.com/majksa-dev/api-gateway/commit/90fe9d3f62b1ca824d92963ba3927c75c31945a1))

## 0.1.0 (2024-03-19)


### Features

* add pingora server ([5de2710](https://github.com/majksa-dev/api-gateway/commit/5de2710b32d46009882348f4d89c2ab68bd31d55))
* add quota config ([176f2c3](https://github.com/majksa-dev/api-gateway/commit/176f2c3e5def5b8132719eb05da152bb5d39c53f))
* add time utils ([bfb83c2](https://github.com/majksa-dev/api-gateway/commit/bfb83c27d178194a7ec381a645445bf638b2e1b2))
* default configuration values ([7c0d524](https://github.com/majksa-dev/api-gateway/commit/7c0d52408a75c1257923523c83514538701ac6e3))
* implement gateway with rate limiting and cors ([b938ad4](https://github.com/majksa-dev/api-gateway/commit/b938ad4cae7b09081421c155bc5dfced26c0449f))
