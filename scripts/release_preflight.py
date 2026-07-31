#!/usr/bin/env python3
from __future__ import annotations

import re
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def read_text_versions(path: Path, pattern: str) -> list[str]:
    versions = re.findall(pattern, path.read_text(encoding="utf-8"), re.MULTILINE)
    if not versions:
        raise SystemExit(f"[error] Version not found in {path.relative_to(ROOT)}")
    return versions


def main() -> None:
    cargo_version = tomllib.loads((ROOT / "Cargo.toml").read_text(encoding="utf-8"))["package"]["version"]
    versions = {
        "Cargo.toml": [cargo_version],
        "SPEC.md": read_text_versions(ROOT / "SPEC.md", r"\*\*Crate version\s+([^\s]+)"),
    }
    lockfile = ROOT / "Cargo.lock"
    if lockfile.exists():
        lock = tomllib.loads(lockfile.read_text(encoding="utf-8"))
        lock_versions = [item["version"] for item in lock.get("package", []) if item.get("name") == "oasf"]
        if lock_versions:
            versions["Cargo.lock"] = lock_versions
    mismatches = {
        name: found
        for name, found_versions in versions.items()
        for found in found_versions
        if found != cargo_version
    }
    if mismatches:
        for name, version in mismatches.items():
            print(f"[error] {name} version {version} != Cargo.toml {cargo_version}")
        raise SystemExit(1)
    print(f"[ok] release metadata is consistent at version {cargo_version}")


if __name__ == "__main__":
    main()
