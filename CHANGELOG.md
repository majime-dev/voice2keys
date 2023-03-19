# Changelog

## [2.0.0](https://github.com/majime-dev/voice2keys/compare/v1.0.1...v2.0.0) (2023-03-19)


### ⚠ BREAKING CHANGES

* This requires changes to your config.
    * Existing configs no longer work, since the value of the `[commands]` map is now a sequence instead of a string.
    * The old `key1+key2` syntax to denote a sequence is no longer supported, and has been repurposed for describing a combination of keys instead (like you would expect).
* This requries changes to your config.
    * Remove the "key_" prefix from the setting names in the "[timing]" section.

### Features

* add support for key combinations ([cfde376](https://github.com/majime-dev/voice2keys/commit/cfde3764e2923d3e72db2c603d8e929fc958bf41))
* **input:** add aliases for arrow keys ([96e578b](https://github.com/majime-dev/voice2keys/commit/96e578be65d9d0d903b9a7a0e27fc8e5b853177d))


### Bug Fixes

* give the user a chance to read the error message before the process exits ([197968d](https://github.com/majime-dev/voice2keys/commit/197968d79d98008bb32f65f9c3c97109bfec3f66))


### Code Refactoring

* strip the "key" prefix from timing ([4ef5e7a](https://github.com/majime-dev/voice2keys/commit/4ef5e7a176203176315ce57ecc94d9d236f993c3))

## [1.0.1](https://github.com/majime-dev/voice2keys/compare/v1.0.0...v1.0.1) (2023-03-10)


### Bug Fixes

* support lower-case aliases for letter keys in config ([f129b85](https://github.com/majime-dev/voice2keys/commit/f129b85b1991b9fe89eff4f8542c9703ea2351de))

## [1.0.0](https://github.com/majime-dev/voice2keys/compare/v0.1.0...v1.0.0) (2023-03-06)


### ⚠ BREAKING CHANGES

* replace enigo with winput

### Features

* add support for no delays between key presses ([54c35b1](https://github.com/majime-dev/voice2keys/commit/54c35b1f04e6db2d5815f2e3a98dbee8c1d1baf0))


### Bug Fixes

* pass on tag names to package-binary ([7cac4f3](https://github.com/majime-dev/voice2keys/commit/7cac4f3b2ccfc40c24ebf7782ce21bd5db0c6e0e))
* replace enigo with winput ([064d475](https://github.com/majime-dev/voice2keys/commit/064d475401e68da392b303897428973f69a89546))

## 0.1.0 (2023-03-06)


### Features

* add a convenience CLI wrapper ([f4b56cd](https://github.com/majime-dev/voice2keys/commit/f4b56cd05333214babac2e1c46c4e956c3790cb5))
* add the initial implementation for the voice2keys library ([ae52d22](https://github.com/majime-dev/voice2keys/commit/ae52d2293fc626491b33c28cd805720e36719057))


### Bug Fixes

* allow using human-friendly time descriptors in the config ([150130c](https://github.com/majime-dev/voice2keys/commit/150130c5f4430134873921a3187d7a9235abdd2f))
* fetch with depth 0 for releases ([6717034](https://github.com/majime-dev/voice2keys/commit/671703498a27a21add773e1124610d01447f1f66))
* parse the key sequence, enigo doesn't have easily parsable enums (at least by serde) ([627cd1a](https://github.com/majime-dev/voice2keys/commit/627cd1a43daf57d2ba3ce379d774c510b77fd3d5))
* use a vendored version of enigo until a fixed version is available on crates.io ([701f5ad](https://github.com/majime-dev/voice2keys/commit/701f5ade29a46c07824a8f7b6acb1af15b3d7366))
* work around the actions triggering logic ([c6b57e7](https://github.com/majime-dev/voice2keys/commit/c6b57e78cdff9e29e19f958b297f8ee69645e204))

## 0.1.0 (2023-03-06)


### Bug Fixes

* allow using human-friendly time descriptors in the config ([150130c](https://github.com/majime-dev/voice2keys/commit/150130c5f4430134873921a3187d7a9235abdd2f))
* fetch with depth 0 for releases ([6717034](https://github.com/majime-dev/voice2keys/commit/671703498a27a21add773e1124610d01447f1f66))
* parse the key sequence, enigo doesn't have easily parsable enums (at least by serde) ([627cd1a](https://github.com/majime-dev/voice2keys/commit/627cd1a43daf57d2ba3ce379d774c510b77fd3d5))
* use a vendored version of enigo until a fixed version is available on crates.io ([701f5ad](https://github.com/majime-dev/voice2keys/commit/701f5ade29a46c07824a8f7b6acb1af15b3d7366))
* work around the actions triggering logic ([c6b57e7](https://github.com/majime-dev/voice2keys/commit/c6b57e78cdff9e29e19f958b297f8ee69645e204))


### Features

* add a convenience CLI wrapper ([f4b56cd](https://github.com/majime-dev/voice2keys/commit/f4b56cd05333214babac2e1c46c4e956c3790cb5))
* add the initial implementation for the voice2keys library ([ae52d22](https://github.com/majime-dev/voice2keys/commit/ae52d2293fc626491b33c28cd805720e36719057))
