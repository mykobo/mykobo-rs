# Release Process & Changelog Generation

This project uses [cargo-release](https://github.com/crate-ci/cargo-release) for automated releases and [git-cliff](https://github.com/orhun/git-cliff) for changelog generation based on conventional commits.

## Quick Start

```bash
# 1. Install git-cliff (one-time setup)
make install-git-cliff

# 2. Preview what would be in the next release
make changelog-preview

# 3. Create a release
make release              # Patch release (0.0.27 -> 0.0.28)
make release-minor        # Minor release (0.0.27 -> 0.1.0)
make release-major        # Major release (0.0.27 -> 1.0.0)
```

## Conventional Commits

This project follows [Conventional Commits](https://www.conventionalcommits.org/) specification. Each commit message should be structured as:

```
<type>(<scope>): <description>

[optional body]

[optional footer(s)]
```

### Commit Types

| Type | Description | Changelog Section | Version Bump |
|------|-------------|-------------------|--------------|
| `feat` | New feature | ⛰️  Features | Minor (0.0.x → 0.1.0) |
| `fix` | Bug fix | 🐛 Bug Fixes | Patch (0.0.27 → 0.0.28) |
| `docs` | Documentation | 📚 Documentation | None |
| `perf` | Performance improvement | ⚡ Performance | Patch |
| `refactor` | Code refactoring | 🚜 Refactor | Patch |
| `style` | Code style changes | 🎨 Styling | None |
| `test` | Adding/updating tests | 🧪 Testing | None |
| `chore` | Maintenance tasks | ⚙️ Miscellaneous | None |
| `ci` | CI/CD changes | ⚙️ Miscellaneous | None |
| `revert` | Revert previous commit | ◀️ Revert | Patch |

### Breaking Changes

To indicate a breaking change, add `!` after the type or add `BREAKING CHANGE:` in the footer:

```bash
# Method 1: Using !
feat(api)!: remove deprecated Metadata struct

# Method 2: Using footer
feat(api): remove deprecated Metadata struct

BREAKING CHANGE: The Metadata struct has been removed. Use MetaData instead.
```

**Breaking changes trigger a MAJOR version bump** (0.0.27 → 1.0.0 or 0.1.0 → 1.0.0)

### Examples

```bash
# Feature - adds to "Features" section, minor version bump
feat(message_bus): add support for EventType enum

# Bug fix - adds to "Bug Fixes" section, patch version bump
fix(identity): correct token refresh logic

# Documentation - adds to "Documentation" section, no version bump
docs(readme): update installation instructions

# Refactor - adds to "Refactor" section, patch version bump
refactor(models): move models to service-specific directories

This moves request/response models from src/models/ to their
respective service directories for better organization.

# Breaking change - triggers major version bump
feat(message_bus)!: remove deprecated generate_meta_data function

BREAKING CHANGE: The generate_meta_data function has been removed.
Use MetaData::new() instead.

Migration guide:
- Old: generate_meta_data(...)
- New: MetaData::new(...).unwrap()
```

## Release Process

### Automatic Release (Recommended)

When you run `make release`, the following happens automatically:

1. **Version Bump**: Updates version in `Cargo.toml` based on commits
2. **Changelog Generation**: Runs git-cliff to update `CHANGELOG.md`
3. **Git Commit**: Creates commit with message `chore: Release mykobo-rs version X.Y.Z`
4. **Git Tag**: Creates annotated tag `vX.Y.Z`
5. **Optional Push**: Pushes to remote (if configured)

```bash
# Patch release (0.0.27 -> 0.0.28)
make release-patch

# Minor release (0.0.27 -> 0.1.0)
make release-minor

# Major release (0.0.27 -> 1.0.0)
make release-major
```

### Manual Changelog Update

If you want to update the changelog without creating a release:

```bash
# Preview unreleased changes
make changelog-preview

# Update CHANGELOG.md with unreleased changes
make changelog
```

## CHANGELOG Format

The generated `CHANGELOG.md` follows this format:

```markdown
# Changelog

All notable changes to this project will be documented in this file.

## [0.1.0] - 2025-01-14

### ⛰️  Features

- **message_bus**: add support for EventType enum
- **models**: move models to service-specific directories

### 🐛 Bug Fixes

- **identity**: correct token refresh logic
- **wallets**: fix wallet profile retrieval

### 🚜 Refactor

- **models**: reorganize request/response structures

### 📚 Documentation

- **readme**: add semantic release guide
- **api**: update API usage examples

## [0.0.28] - 2025-01-13

### 🐛 Bug Fixes

- **tests**: add serial attribute to prevent race conditions
```

## Configuration Files

### `cliff.toml`

Configures how git-cliff generates the changelog:
- Commit parsing rules
- Changelog sections and grouping
- Commit filters and preprocessors
- Template formatting

### `release.toml`

Configures cargo-release behavior:
- Version bump strategy
- Pre-release hooks (changelog generation)
- Commit messages
- Tag format
- Push settings

## Best Practices

### 1. Write Clear Commit Messages

```bash
# Good
feat(identity): add OAuth2 authentication support

# Bad
feat: updates
```

### 2. Use Scopes Consistently

Common scopes in this project:
- `identity` - Identity service client
- `wallets` - Wallet service client
- `message_bus` - Message bus functionality
- `models` - Data models
- `tests` - Test suite
- `docs` - Documentation

### 3. Group Related Changes

```bash
# Instead of multiple commits for one feature:
feat: add part 1
feat: add part 2
feat: add part 3

# Squash into one commit:
feat(feature-name): add complete feature implementation
```

### 4. Reference Issues

```bash
fix(identity): resolve token refresh race condition

Fixes #123
Closes #456
```

## Troubleshooting

### Changelog not generating

**Problem**: CHANGELOG.md is not updated during release

**Solution**:
1. Ensure git-cliff is installed: `make install-git-cliff`
2. Check `release.toml` has `pre-release-hook` configured
3. Verify commits follow conventional format

### Wrong version bump

**Problem**: Release created wrong version (e.g., major instead of minor)

**Cause**: Breaking change detected when not intended

**Solution**:
- Don't use `!` in commit type unless it's actually breaking
- Don't include `BREAKING CHANGE:` in commit footer unless necessary
- Review recent commits for unintended breaking change markers

### Commits not appearing in changelog

**Problem**: Some commits missing from CHANGELOG.md

**Possible causes**:
1. Commits don't follow conventional format
2. Commit type is configured to be skipped (like `chore`, `test`)
3. Commits match a skip pattern in `cliff.toml`

**Solution**:
- Use `make changelog-preview` to see what will be included
- Check commit format against examples above
- Review `cliff.toml` commit_parsers section

## CI/CD Integration

For automated releases in GitHub Actions:

```yaml
name: Release

on:
  workflow_dispatch:
    inputs:
      version:
        description: 'Version bump type'
        required: true
        type: choice
        options:
          - patch
          - minor
          - major

jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
        with:
          fetch-depth: 0  # Full history for changelog

      - uses: dtolnay/rust-toolchain@stable

      - name: Install git-cliff
        run: cargo install git-cliff

      - name: Create release
        run: |
          git config user.name "github-actions[bot]"
          git config user.email "github-actions[bot]@users.noreply.github.com"
          make release-${{ github.event.inputs.version }}
```

## Resources

- [Conventional Commits Specification](https://www.conventionalcommits.org/)
- [cargo-release Documentation](https://github.com/crate-ci/cargo-release)
- [git-cliff Documentation](https://git-cliff.org/)
- [Semantic Versioning](https://semver.org/)

## Summary

1. **Write commits** using conventional commit format
2. **Preview changelog** with `make changelog-preview`
3. **Create release** with `make release` (or `release-minor`/`release-major`)
4. **CHANGELOG.md** is automatically generated and committed
5. **Version** is automatically bumped in Cargo.toml
6. **Git tag** is automatically created
