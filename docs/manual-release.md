# Manual Release Guide

This is the controlled fallback when CircleCI is unavailable. The same
fail-closed contract as `.circleci/config.yml` applies: never publish or
announce a release with a partial asset set.

## Release contract

1. Set `Cargo.toml` and `Cargo.lock` to the intended version, for example
   `0.3.28`.
2. Create and push the matching tag, for example `v0.3.28`, from the exact
   source to be released.
3. Build exactly these archives:

   - `linear-cli-x86_64-unknown-linux-gnu.tar.gz`
   - `linear-cli-aarch64-unknown-linux-gnu.tar.gz`
   - `linear-cli-x86_64-pc-windows-msvc.zip`
   - `linear-cli-x86_64-apple-darwin.tar.gz`
   - `linear-cli-aarch64-apple-darwin.tar.gz`

4. Verify native binaries with `linear --version`, and verify that their
   output equals `linear <version>`. For cross-target builds, verify the
   target format with `file` and preserve the version from the tagged
   `Cargo.toml` build.
5. Create/upload the GitHub release only after all five artifacts pass.
6. Publish the same version to crates.io with `cargo publish --locked`.

The Windows archive must contain `linear.exe` at its archive root. The
other archives must contain `linear` at their archive root.

## Build commands

Run these commands from the release tag. Linux builds need `libdbus-1-dev` and
`pkg-config`; the repository `Cross.toml` supplies the aarch64 Linux setup.

```bash
sudo apt-get update
sudo apt-get install -y libdbus-1-dev pkg-config

cargo build --locked --release --features secure-storage \
  --target x86_64-unknown-linux-gnu
tar -C target/x86_64-unknown-linux-gnu/release -czf \
  linear-cli-x86_64-unknown-linux-gnu.tar.gz linear-cli

cargo install cross --locked --version 0.2.5
cross build --locked --release --features secure-storage \
  --target aarch64-unknown-linux-gnu
tar -C target/aarch64-unknown-linux-gnu/release -czf \
  linear-cli-aarch64-unknown-linux-gnu.tar.gz linear-cli
```

Build the two Apple targets on a macOS host:

```bash
rustup target add x86_64-apple-darwin aarch64-apple-darwin
cargo build --locked --release --features secure-storage --target x86_64-apple-darwin
tar -C target/x86_64-apple-darwin/release -czf \
  linear-cli-x86_64-apple-darwin.tar.gz linear-cli
cargo build --locked --release --features secure-storage --target aarch64-apple-darwin
tar -C target/aarch64-apple-darwin/release -czf \
  linear-cli-aarch64-apple-darwin.tar.gz linear-cli
```

Build the Windows target on a Windows host with the MSVC toolchain:

```powershell
rustup target add x86_64-pc-windows-msvc
cargo build --locked --release --features secure-storage --target x86_64-pc-windows-msvc
Compress-Archive -Path target/x86_64-pc-windows-msvc/release/linear-cli.exe `
  -DestinationPath linear-cli-x86_64-pc-windows-msvc.zip -Force
```

## Verify and upload

Before uploading, check the archive names and versions manually, then create
or update the release with the exact tag:

```bash
sha256sum linear-cli-*
gh release create v0.3.28 --verify-tag --title v0.3.28 --generate-notes
gh release upload v0.3.28 linear-cli-*.tar.gz linear-cli-*.zip --clobber
cargo publish --locked
```

Finally confirm both public surfaces show the same version:

```bash
cargo search linear-cli --limit 1
gh release view v0.3.28
```

The crates.io version and the GitHub release tag must match before announcing
the release.
