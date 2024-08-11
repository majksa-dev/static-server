# Changelog

## [0.2.8](https://github.com/majksa-dev/static-server/compare/v0.2.7...v0.2.8) (2024-08-11)


### Bug Fixes

* **platforms:** build only for amd ([6d4c1bd](https://github.com/majksa-dev/static-server/commit/6d4c1bd3e3dbd87a1d539fb7e4ce64247f0443ba))

## [0.2.7](https://github.com/majksa-dev/static-server/compare/v0.2.6...v0.2.7) (2024-08-11)


### Bug Fixes

* **deps:** bump versions ([b5e2265](https://github.com/majksa-dev/static-server/commit/b5e2265fd5b9f0d8d3c350dea5becaa634dc6fcb))

## [0.2.6](https://github.com/majksa-dev/static-server/compare/v0.2.5...v0.2.6) (2024-08-10)


### Bug Fixes

* **deps:** bump updates ([2b94f26](https://github.com/majksa-dev/static-server/commit/2b94f263a806b1f01fc5b4a3d35da634eddd7a36))
* **dockerfile:** healthcheck ([fb424d5](https://github.com/majksa-dev/static-server/commit/fb424d5765bc656e1e8fed1ecc6fa1346dc01da2))

## [0.2.5](https://github.com/majksa-dev/static-server/compare/v0.2.4...v0.2.5) (2024-08-08)


### Bug Fixes

* **deps:** bump the dependencies group across 1 directory with 3 updates ([30a93a1](https://github.com/majksa-dev/static-server/commit/30a93a1d61109de952eb5e85d54f6a644e2dabcb))
* **deps:** use all features for derive_more ([4e6bc25](https://github.com/majksa-dev/static-server/commit/4e6bc25e446c5eb702e253d50e697e61b32774e5))
* **router:** handle document index and fallback page ([6817fb1](https://github.com/majksa-dev/static-server/commit/6817fb176eb5877d1dab3e6af6e7a182a9271d3b))

## [0.2.4](https://github.com/majksa-dev/static-server/compare/v0.2.3...v0.2.4) (2024-07-29)


### Bug Fixes

* dependabot groups ([f139a0e](https://github.com/majksa-dev/static-server/commit/f139a0e1889a589377ea023b5d634cf86a944fb5))
* **deps:** bump the dependencies group with 7 updates ([7560498](https://github.com/majksa-dev/static-server/commit/7560498f724c6b144de9cf2048661ed6cca5c976))
* response body copy to ([06d3c16](https://github.com/majksa-dev/static-server/commit/06d3c16a5cea689f276f88f07a8bf540e7b931b2))

## [0.2.3](https://github.com/majksa-dev/static-server/compare/v0.2.2...v0.2.3) (2024-06-27)


### Bug Fixes

* **deps:** update gateway ([40e2df7](https://github.com/majksa-dev/static-server/commit/40e2df70b4c4f908f3f36c0e7b3fa8cbef7d27e9))
* **deps:** upgrade dependencies ([f7f049d](https://github.com/majksa-dev/static-server/commit/f7f049dc2dee3013bba0c17f50968a3cae91d966))

## [0.2.2](https://github.com/majksa-dev/static-server/compare/v0.2.1...v0.2.2) (2024-06-22)


### Bug Fixes

* release please version ([ae4e1af](https://github.com/majksa-dev/static-server/commit/ae4e1af8b844b5f3a3bf68e9e7017f15d451855c))
* update Cargo.lock ([cb4da29](https://github.com/majksa-dev/static-server/commit/cb4da292c04d0f43828594347771a3243101d8b8))

## [0.2.1](https://github.com/majksa-dev/static-server/compare/v0.2.0...v0.2.1) (2024-06-22)


### Bug Fixes

* implement zero copy using sendfile and new gateway changes ([22211bb](https://github.com/majksa-dev/static-server/commit/22211bb245681c2fb64c547c764e8e949b0e4658))
* response body copy_to should return () ([a2a4c04](https://github.com/majksa-dev/static-server/commit/a2a4c04ef0a2ffa404d5c1518f08155f3aa4e29b))

## [0.2.0](https://github.com/majksa-dev/static-server/compare/v0.1.1...v0.2.0) (2024-06-11)


### Features

* implement custom rust server library ([2842765](https://github.com/majksa-dev/static-server/commit/284276514a80a2876804738b18e1c6da0d7363c1))

## [0.1.1](https://github.com/majksa-dev/static-server/compare/v0.1.0...v0.1.1) (2024-05-16)


### Bug Fixes

* build docker only for "linux/amd64" and "linux/arm64" ([2d00049](https://github.com/majksa-dev/static-server/commit/2d000496487193a7c41850e1d42816c0ae9f03a1))

## 0.1.0 (2024-05-16)


### Features

* expose application inside docker ([164c8ba](https://github.com/majksa-dev/static-server/commit/164c8baf869af289b3c1705978bb7528e434802b))
* fast multi threaded static server with etag, health check ([d835044](https://github.com/majksa-dev/static-server/commit/d835044cff129b37d6ee13119e60f1a47564afa2))


### Bug Fixes

* **deps:** bump chrono from 0.4.35 to 0.4.38 ([3fde9fd](https://github.com/majksa-dev/static-server/commit/3fde9fde179c4130dd2c5cb4cd0e32f2f385a5eb))
* **deps:** bump serde from 1.0.201 to 1.0.202 ([b16e25f](https://github.com/majksa-dev/static-server/commit/b16e25f6fccf1591d0c57152155639871a217af8))
* **deps:** bump serde_json from 1.0.114 to 1.0.117 ([8c13635](https://github.com/majksa-dev/static-server/commit/8c1363560bdd569f3499198b71588f0935504ae9))
* inefficient clone ([5d22744](https://github.com/majksa-dev/static-server/commit/5d22744d3e1404fb2919b49e0fee12ab31f2b3c9))
* use specific versions ([b839852](https://github.com/majksa-dev/static-server/commit/b839852537fc1ab72649436622565de44f8dc44a))
