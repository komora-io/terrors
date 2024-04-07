# Changelog

All notable changes to this project will be documented in this file.

The format is based on
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## \[Unreleased]

## \[0.3.0] - 2024-04-06

### Added

- Added `OneOf::subset()`.
- Implemented `Clone` for `OneOf` where all variants are `Clone`.

## \[0.2.6] - 2024-04-02

### Added

- Added `OneOf::to_enum()` and `OneOf::as_enum()`.

## \[0.2.5] - 2024-04-01

- Implement `Debug`, `Display` and `Error` if all types in the `OneOf` implement them.

## \[0.2.4] - 2024-03-31

## \[0.2.3] - 2024-03-31

## \[0.2.2] - 2024-03-31

## \[0.2.1] - 2024-03-31

### Added

- Added support for tuples up to length 9.

## \[0.2.0] - 2024-03-31

### Added

- Added `OneOf::take()`.
- Implemented `Deref<Target = T>` for `OneOf<(T,)>`.

## \[0.1.6] - 2024-03-31

## \[0.1.4] - 2024-03-31

## \[0.1.3] - 2024-03-31

## \[0.1.2] - 2024-03-31

### Added

- Added `OneOf::broaden()`.

## \[0.1.1] - 2024-03-30

### Fixed

- Miscellaneous fixes and cleanups.

## \[0.1.0] - 2024-03-30

Initial release.
