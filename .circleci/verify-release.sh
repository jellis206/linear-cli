#!/usr/bin/env bash
set -euo pipefail

: "${CIRCLE_TAG:?This job must run from a release tag}"
if [[ ! "$CIRCLE_TAG" =~ ^v[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
  echo "Unsupported release tag: $CIRCLE_TAG" >&2
  exit 1
fi

version="${CIRCLE_TAG#v}"
cargo_version="$(awk -F'"' '/^version = "/ { print $2; exit }' Cargo.toml)"
test "$cargo_version" = "$version"

expected=(
  linear-cli-x86_64-unknown-linux-gnu.tar.gz
  linear-cli-aarch64-unknown-linux-gnu.tar.gz
  linear-cli-x86_64-pc-windows-msvc.zip
  linear-cli-x86_64-apple-darwin.tar.gz
  linear-cli-aarch64-apple-darwin.tar.gz
)
mkdir -p release
for archive in "${expected[@]}"; do
  found="$(find artifacts -type f -name "$archive" -print -quit)"
  test -n "$found"
  cp -- "$found" "release/$archive"

  proof="$(dirname -- "$found")/version.txt"
  test -f "$proof"
  test "$(tr -d '\r\n' < "$proof")" = "linear-cli $version"
done

actual_count="$(find artifacts -type f \( -name 'linear-cli-*.tar.gz' -o -name 'linear-cli-*.zip' \) | wc -l)"
test "$actual_count" -eq "${#expected[@]}"

command -v file >/dev/null
extract_dir="$(mktemp -d)"
trap 'rm -rf -- "$extract_dir"' EXIT
for archive in "${expected[@]}"; do
  target="${archive#linear-cli-}"
  target="${target%.tar.gz}"
  target="${target%.zip}"
  destination="$extract_dir/$target"
  mkdir -p "$destination"
  if [[ "$archive" == *.tar.gz ]]; then
    test "$(tar -tzf "release/$archive")" = "linear-cli"
    tar -xzf "release/$archive" -C "$destination"
    binary="$destination/linear-cli"
    test -x "$binary"
    file_description="$(file -b "$binary")"
    case "$archive" in
      linear-cli-x86_64-unknown-linux-gnu.tar.gz)
        grep -Eq 'ELF 64-bit.*x86-64' <<<"$file_description"
        reported="$($binary --version)"
        test "$reported" = "linear-cli $version"
        ;;
      linear-cli-aarch64-unknown-linux-gnu.tar.gz)
        grep -Eq 'ELF 64-bit.*(ARM aarch64|AArch64)' <<<"$file_description"
        ;;
      linear-cli-x86_64-apple-darwin.tar.gz)
        grep -Eq 'Mach-O 64-bit.*x86_64' <<<"$file_description"
        ;;
      linear-cli-aarch64-apple-darwin.tar.gz)
        grep -Eq 'Mach-O 64-bit.*(arm64|ARM64)' <<<"$file_description"
        ;;
    esac
  else
    test "$(unzip -Z1 "release/$archive" | tr -d '\r')" = "linear-cli.exe"
    unzip -q "release/$archive" -d "$destination"
    binary="$destination/linear-cli.exe"
    test -f "$binary"
    file_description="$(file -b "$binary")"
    grep -Eq 'PE32\+ executable.*x86-64' <<<"$file_description"
  fi
done

sha256sum release/linear-cli-* > release/SHA256SUMS
test "$(wc -l < release/SHA256SUMS)" -eq "${#expected[@]}"
python3 .circleci/release-manifest.py "$version"
test "$(find release -maxdepth 1 -type f | wc -l)" -eq 7
