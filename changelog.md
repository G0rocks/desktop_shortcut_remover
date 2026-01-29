# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Tray icon with possibility to turn off program from system tray

### Changed

- 

### Removed

- 

## [1.1.0] - 2026-01-29

### Added

- Event based shortcut deletions and in case they don't work, the 10:00 o'clock scheduled delete is still in place :) 

## [1.0.0] - 2025-12-07
The first release of the desktop shortcut remover.
Made to deal with [this problem](https://github.com/microsoft/winget-cli/issues/1545#issuecomment-3620746650).
Currently you must run the program and then it will delete all shortcuts from the desktop (public and user) at 10:00 o'clock.
To turn the program off, you have to open the task manager and end the task. But hey, it works! ^_^

### Added

- main.rs
- .gitignore
- cargo.lock
- cargo.toml
- changelog.md

## List of releases
[unreleased]: https://github.com/G0rocks/marine_vessel_simulator/compare/1.0.0...main
[1.0.0]: https://github.com/G0rocks/marine_vessel_simulator/releases/tag/1.0.0