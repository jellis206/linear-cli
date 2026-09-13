#!/usr/bin/env python3
"""Generate the exact five-target release manifest."""

import hashlib
import json
import pathlib
import sys


EXPECTED_ARCHIVES = sorted(
    [
        "linear-cli-x86_64-unknown-linux-gnu.tar.gz",
        "linear-cli-aarch64-unknown-linux-gnu.tar.gz",
        "linear-cli-x86_64-pc-windows-msvc.zip",
        "linear-cli-x86_64-apple-darwin.tar.gz",
        "linear-cli-aarch64-apple-darwin.tar.gz",
    ]
)


def target_for(name: str) -> str:
    prefix = "linear-cli-"
    if name.endswith(".tar.gz"):
        suffix = ".tar.gz"
    elif name.endswith(".zip"):
        suffix = ".zip"
    else:
        raise ValueError(f"unsupported archive: {name}")
    return name[len(prefix) : -len(suffix)]


def main() -> int:
    if len(sys.argv) != 2:
        raise SystemExit("usage: release-manifest.py VERSION")

    version = sys.argv[1]
    release_dir = pathlib.Path("release")
    archives = sorted(
        path.name
        for path in release_dir.iterdir()
        if path.is_file() and (path.name.endswith(".tar.gz") or path.name.endswith(".zip"))
    )
    if archives != EXPECTED_ARCHIVES:
        raise SystemExit(f"release archives do not match expected set: {archives!r}")

    manifest = {
        "version": version,
        "archives": [
            {
                "target": target_for(name),
                "file": name,
                "sha256": hashlib.sha256((release_dir / name).read_bytes()).hexdigest(),
            }
            for name in EXPECTED_ARCHIVES
        ],
    }
    (release_dir / "release-manifest.json").write_text(
        json.dumps(manifest, indent=2) + "\n", encoding="utf-8"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
