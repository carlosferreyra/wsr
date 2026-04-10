## [0.0.2] - 2026-04-10

### 🚀 Features

- Scaffold module structure
- *(cli)* Add wsr init and wsr run subcommands with clap

### 🐛 Bug Fixes

- *(ci)* Remove aarch64-pc-windows-msvc from dist targets

### 📚 Documentation

- Add v0.1 implementation checklist
- Add docs/ directory mirroring src module structure
- Reformat docs index table alignment
## [0.0.1] - 2026-04-10

### 📚 Documentation

- Add CONTRIBUTING, SECURITY, CODE_OF_CONDUCT and author metadata
- Reformat CODE_OF_CONDUCT to 100 char line width

### ⚙️ Miscellaneous Tasks

- Cargo new wsr with README, ROADMAP, and ARCHITECTURE
- Initialize cargo-dist with GitHub Actions release workflow
- Add MIT license
- Add package metadata (description, license) to Cargo.toml
- Add git-cliff and cargo-release config with CHANGELOG
- Fix pre-release hook to stage CHANGELOG.md before release commit
