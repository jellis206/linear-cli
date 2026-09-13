# CI Operator Guide

CircleCI is the canonical CI and release path. See
`.circleci/README.md` and `.circleci/config.yml` for the checked-in pipeline.

## Branch and pull-request CI

The CircleCI `ci` workflow runs locked tests with `secure-storage`, formatting,
clippy with warnings denied, and a default-feature build on Linux. It runs for
non-release refs.

## Tagged releases

Push a tag matching `vX.Y.Z`. CircleCI builds these five archives:

- `x86_64-unknown-linux-gnu`
- `aarch64-unknown-linux-gnu`
- `x86_64-pc-windows-msvc`
- `x86_64-apple-darwin`
- `aarch64-apple-darwin`

The release verifier fails closed unless the tag matches `Cargo.toml`, all
five archives are present, each archive contains the expected binary, and each
binary reports the tagged version. The GitHub release upload and crates.io
publish jobs run only after this gate.

## Credentials and plan prerequisites

Create the restricted CircleCI context `linear-cli-release` with:

- `GH_TOKEN`: permission to create or update releases in `nesszer/linear-cli`.
- `CARGO_REGISTRY_TOKEN`: permission to publish `linear-cli` on crates.io.

The macOS and Windows executors also need to be enabled for the CircleCI
organization/plan. The pipeline cannot prove that external project wiring or
executor entitlements exist from this repository alone.

## Legacy fallback

The GitHub Actions CI file is manual-dispatch only during the migration. The
old GitHub Actions release workflow is disabled so it cannot publish a partial
or incorrectly tagged asset set.
