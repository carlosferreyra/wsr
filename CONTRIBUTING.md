# Contributing to wsr

Thank you for your interest in contributing!

## How to contribute

1. **Fork** the repository and clone your fork locally.
2. Create a branch for your change:
   ```bash
   git checkout -b feat/your-feature
   ```
3. Make your changes and commit following **Conventional Commits**:
   ```
   feat: add workflow parser
   fix: handle missing config file
   docs: update usage examples
   ```
4. Push to your fork and open a **Pull Request** against `main`.

## Commit conventions

This project uses [Conventional Commits](https://www.conventionalcommits.org). Your commit messages directly determine the changelog, so please be descriptive.

Common prefixes:
- `feat:` — new feature
- `fix:` — bug fix
- `docs:` — documentation only
- `refactor:` — code change that neither fixes a bug nor adds a feature
- `chore:` — tooling, dependencies, config
- `ci:` — CI/CD changes

## Development setup

```bash
git clone https://github.com/<your-fork>/wsr.git
cd wsr
cargo build
cargo test
```

## Preview the changelog before releasing

```bash
git cliff
```

## Questions?

Open an issue or reach out at eduferreyraok@gmail.com.
