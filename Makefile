prepare:
	cargo fmt && cargo clippy --fix

# Install git-cliff for changelog generation
install-git-cliff:
	cargo install git-cliff

# Generate/update CHANGELOG.md for unreleased changes
changelog:
	git cliff --unreleased --tag unreleased --prepend CHANGELOG.md

# Preview the changelog that would be generated
changelog-preview:
	git cliff --unreleased

# Create a patch release (0.0.27 -> 0.0.28)
# Automatically generates CHANGELOG.md via pre-release-hook
release-patch:
	cargo release patch --execute

# Create a minor release (0.0.27 -> 0.1.0)
release-minor:
	cargo release minor --execute

# Create a major release (0.0.27 -> 1.0.0)
release-major:
	cargo release major --execute

# Default release (patch)
release: release-patch

test:
	@cargo nextest run --nocapture
