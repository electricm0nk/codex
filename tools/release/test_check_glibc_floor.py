#!/usr/bin/env python3
"""
Tests for tools/release/check_glibc_floor.py.

0.21.153's Linux build linked on an Ubuntu 26.04 runner and needed GLIBC_2.43, so it could not start on
Ubuntu 24.04 (GitHub issue #409). The gate fails a release whose shipped ELFs need a newer glibc than
the floor; these tests prove it fails, passes, and never passes silently.

Run: python3 -m pytest tools/release/test_check_glibc_floor.py -q
"""

from __future__ import annotations

import importlib.util
import re
import subprocess
import sys
import tempfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
GATE = HERE / "check_glibc_floor.py"
REPO = HERE.parent.parent
PUBLISH = REPO / ".github/workflows/publish-tester-release.yml"
CHECK = REPO / ".github/workflows/linux-build-check.yml"

_spec = importlib.util.spec_from_file_location("check_glibc_floor", GATE)
gate = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(gate)

# `objdump -T usr/bin/codex-desktop` lines from Codex_0.21.153_amd64.deb (alpha-v0.21.153-d6890a6c):
# every symbol above 2.34 plus a sample of the older ones.
OBJDUMP_0_21_153 = """
usr/bin/codex-desktop:     file format elf64-x86-64

DYNAMIC SYMBOL TABLE:
0000000000000000      DF *UND*\t0000000000000000 (GLIBC_2.34) __libc_start_main
0000000000000000  w   DF *UND*\t0000000000000000 (GLIBC_2.2.5) __cxa_finalize
0000000000000000      DF *UND*\t0000000000000000 (GLIBC_2.14) memcpy
0000000000000000      DF *UND*\t0000000000000000 (GLIBC_2.43) atan2f
0000000000000000      DF *UND*\t0000000000000000 (GLIBC_2.43) log10f
0000000000000000      DF *UND*\t0000000000000000 (GLIBC_2.43) acosf
0000000000000000  w   DF *UND*\t0000000000000000 (GLIBC_2.39) pidfd_spawnp
0000000000000000  w   DF *UND*\t0000000000000000 (GLIBC_2.39) pidfd_getpid
0000000000000000      DO *UND*\t0000000000000000 (GLIBC_PRIVATE) _rtld_global_ro
"""

OBJDUMP_MAX_2_34 = """
DYNAMIC SYMBOL TABLE:
0000000000000000      DF *UND*\t0000000000000000 (GLIBC_2.34) __libc_start_main
0000000000000000      DF *UND*\t0000000000000000 (GLIBC_2.3.4) __memcpy_chk
0000000000000000      DF *UND*\t0000000000000000 (GLIBC_2.2.5) free
"""


def _run(*args: str) -> subprocess.CompletedProcess:
    return subprocess.run([sys.executable, str(GATE), *args], capture_output=True, text=True)


def test_0_21_153_symbols_are_above_the_floor():
    assert gate.max_glibc(OBJDUMP_0_21_153) == "2.43"
    line, ok = gate.verdict("codex-desktop", gate.max_glibc(OBJDUMP_0_21_153))
    assert line == "codex-desktop max GLIBC_2.43 floor 2.35 ABOVE"
    assert ok is False


def test_max_2_34_is_ok():
    # 2.3.4 must not sort above 2.34: versions compare numerically, part by part.
    assert gate.max_glibc(OBJDUMP_MAX_2_34) == "2.34"
    line, ok = gate.verdict("pcg_import", "2.34")
    assert line == "pcg_import max GLIBC_2.34 floor 2.35 OK"
    assert ok is True


def test_bin_true_on_this_host_prints_its_max():
    result = _run("/bin/true")
    out = result.stdout.strip().splitlines()
    assert len(out) == 1, result.stdout + result.stderr
    assert re.fullmatch(r"true max GLIBC_\d+(\.\d+)+ floor 2\.35 (OK|ABOVE)", out[0]), out[0]
    # The exit code follows the verdict: 0 on OK, 1 on ABOVE (a host newer than the floor is ABOVE).
    assert result.returncode == (0 if out[0].endswith(" OK") else 1)


def test_missing_path_exits_2():
    result = _run("/nonexistent/codex-desktop")
    assert result.returncode == 2
    assert "missing" in result.stderr


def test_missing_bundle_exits_2():
    assert _run("--deb", "/nonexistent/Codex.deb").returncode == 2
    assert _run("--appimage", "/nonexistent/Codex.AppImage").returncode == 2


def test_no_input_and_non_elf_exit_2():
    assert _run().returncode == 2
    with tempfile.NamedTemporaryFile("w", suffix=".sh", delete=False) as f:
        f.write("#!/bin/sh\n")
    result = _run(f.name)
    assert result.returncode == 2
    assert "no ELF" in result.stderr


def test_floor_lives_in_one_place():
    assert gate.GLIBC_FLOOR == "2.35"
    for workflow in (PUBLISH, CHECK):
        assert "2.35" not in workflow.read_text(), f"{workflow.name} names the floor; it must name the image only"


def _job(text: str, job: str) -> str:
    match = re.search(rf"^  {re.escape(job)}:\n(.*?)(?=^  [A-Za-z0-9_-]+:\n|\Z)", text, re.M | re.S)
    assert match, f"job {job} not found"
    return match.group(1)


def _image(job_text: str) -> str:
    images = re.findall(r"^\s+image:\s*(\S+)\s*$", job_text, re.M)
    assert len(images) == 1, images
    return images[0]


def _apt_packages(job_text: str) -> list[str]:
    match = re.search(r"apt-get install -y((?:[^\n]*\\\n)*[^\n]*)", job_text)
    assert match, "no apt-get install -y step"
    return sorted(match.group(1).replace("\\\n", " ").split())


def test_proof_build_cannot_drift_from_the_shipping_build():
    publish = _job(PUBLISH.read_text(), "publish-tester-release")
    check = _job(CHECK.read_text(), "linux-build-check")
    assert _image(publish) == "ubuntu:22.04"
    assert _image(check) == _image(publish)
    assert _apt_packages(publish) == _apt_packages(check)
    for pkg in ("libwebkit2gtk-4.1-dev", "libappindicator3-dev", "librsvg2-dev", "patchelf"):
        assert pkg in _apt_packages(publish)
    assert "sudo " not in publish.split("Cache Rust build artifacts")[0]
    assert "check_glibc_floor.py" in publish and "check_glibc_floor.py" in check
