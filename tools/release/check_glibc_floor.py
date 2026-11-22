#!/usr/bin/env python3
"""
Gate: every shipped Linux ELF runs on glibc GLIBC_FLOOR or newer.

A Rust binary takes the newest glibc symbol versions its build host offers; 0.21.153 linked on an
Ubuntu 26.04 runner needed GLIBC_2.43 and could not start on Ubuntu 24.04 (GitHub issue #409). The
publish job builds in an older container; this gate proves the result, per binary, before upload.

Usage:
  check_glibc_floor.py [ELF ...] [--deb FILE ...] [--appimage FILE ...]

Prints `<name> max GLIBC_<v> floor <floor> OK|ABOVE` per binary (every ELF under usr/bin/ of a bundle).
Exit 0 all OK, 1 any ABOVE, 2 a missing input, an input holding no ELF, or a failed tool.
"""

from __future__ import annotations

import argparse
import os
import re
import subprocess
import sys
import tempfile
from pathlib import Path

# The one place the floor lives: Ubuntu 22.04's glibc. The workflows name the container image, never this number.
GLIBC_FLOOR = "2.35"

_VERSION = re.compile(r"\bGLIBC_(\d+(?:\.\d+)+)\b")


class InputError(Exception):
    pass


def _key(version: str) -> tuple[int, ...]:
    return tuple(int(part) for part in version.split("."))


def max_glibc(objdump_text: str) -> str | None:
    """Highest GLIBC_x.y version named in `objdump -T` output, compared numerically; None if none."""
    versions = _VERSION.findall(objdump_text)
    return max(versions, key=_key) if versions else None


def verdict(name: str, version: str | None) -> tuple[str, bool]:
    ok = version is None or _key(version) <= _key(GLIBC_FLOOR)
    shown = f"GLIBC_{version}" if version else "GLIBC_none"
    return f"{name} max {shown} floor {GLIBC_FLOOR} {'OK' if ok else 'ABOVE'}", ok


def _is_elf(path: Path) -> bool:
    try:
        with path.open("rb") as f:
            return f.read(4) == b"\x7fELF"
    except OSError:
        return False


def _objdump(path: Path) -> str:
    result = subprocess.run(["objdump", "-T", str(path)], capture_output=True, text=True)
    if result.returncode != 0:
        raise InputError(f"objdump -T {path} failed: {result.stderr.strip()}")
    return result.stdout


def _elfs_under_usr_bin(root: Path, source: str) -> list[Path]:
    usr_bin = root / "usr" / "bin"
    elfs = sorted(p for p in usr_bin.rglob("*") if p.is_file() and not p.is_symlink() and _is_elf(p))
    if not elfs:
        raise InputError(f"no ELF under usr/bin/ in {source}")
    return elfs


def _unpack(kind: str, bundle: Path, tmp: Path) -> Path:
    if not bundle.is_file():
        raise InputError(f"missing {kind} input: {bundle}")
    if kind == "deb":
        cmd, cwd, root = ["dpkg-deb", "-x", str(bundle), str(tmp)], None, tmp
    else:
        local = tmp / "bundle.AppImage"
        local.write_bytes(bundle.read_bytes())
        local.chmod(0o755)
        cmd, cwd, root = [str(local), "--appimage-extract"], tmp, tmp / "squashfs-root"
    result = subprocess.run(cmd, cwd=cwd, capture_output=True, text=True)
    if result.returncode != 0:
        raise InputError(f"unpacking {bundle} failed: {result.stderr.strip()}")
    return root


def _check(paths: list[Path]) -> bool:
    all_ok = True
    for path in paths:
        line, ok = verdict(path.name, max_glibc(_objdump(path)))
        print(line)
        all_ok &= ok
    return all_ok


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("elf", nargs="*", type=Path)
    parser.add_argument("--deb", action="append", default=[], type=Path)
    parser.add_argument("--appimage", action="append", default=[], type=Path)
    args = parser.parse_args(argv)

    if not (args.elf or args.deb or args.appimage):
        print("error: no input (give ELF paths, --deb or --appimage)", file=sys.stderr)
        return 2

    all_ok = True
    try:
        for path in args.elf:
            if not path.exists():
                raise InputError(f"missing input: {path}")
            if not _is_elf(path):
                raise InputError(f"no ELF: {path}")
        if args.elf:
            all_ok &= _check(args.elf)
        for kind, bundles in (("deb", args.deb), ("appimage", args.appimage)):
            for bundle in bundles:
                with tempfile.TemporaryDirectory(prefix="glibc-floor-") as tmp:
                    root = _unpack(kind, bundle, Path(tmp))
                    print(f"== {bundle.name}")
                    all_ok &= _check(_elfs_under_usr_bin(root, str(bundle)))
    except InputError as err:
        print(f"error: {err}", file=sys.stderr)
        return 2
    return 0 if all_ok else 1


if __name__ == "__main__":
    sys.exit(main())
