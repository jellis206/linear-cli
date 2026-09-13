# CI and Release

CircleCI is the canonical CI and release system for linear-cli. The checked-in
configuration is `.circleci/config.yml`; it owns branch validation and tagged
release artifacts. GitHub Actions is retained only as an explicitly dispatched
fallback for CI while repository protection rules are migrated.

## Language

**CI**: The CircleCI `ci` workflow runs the locked secure-storage test suite,
formatting check, clippy with warnings denied, and a default-feature build.

**Release**: A semver tag matching `vX.Y.Z` starts five parallel target builds.
The release gate requires that the tag version equals `Cargo.toml`, that the
exact five archives exist, and that each binary reports the tagged version.
Only then are the GitHub release assets uploaded and the crate published.

**Target set**: `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu`,
`x86_64-pc-windows-msvc`, `x86_64-apple-darwin`, and `aarch64-apple-darwin`.

**Release context**: The CircleCI context `linear-cli-release` must provide
`GH_TOKEN` for GitHub release uploads and `CARGO_REGISTRY_TOKEN` for crates.io.
Those credentials are used only by downstream release jobs after verification.

**Manual fallback**: When CircleCI is unavailable, follow
`docs/manual-release.md` and preserve the same five-asset/version gate.
