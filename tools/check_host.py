#!/usr/bin/env python3
"""Check whether the host can configure and compile Halo's LLVM target."""

from __future__ import annotations

import argparse
import importlib.metadata
import json
import platform
import shutil
import subprocess
import sys
from dataclasses import asdict, dataclass
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
REQUIRED_COMMANDS = ("cmake", "clang", "clang++", "lld-link")
REQUIRED_PACKAGES = ("libclang", "pefile", "pyxbe")


@dataclass
class Check:
    name: str
    ok: bool
    detail: str


def command_checks(which=shutil.which) -> list[Check]:
    return [Check(f"command:{name}", bool(path := which(name)), path or "not found in PATH")
            for name in REQUIRED_COMMANDS]


def package_checks(version=importlib.metadata.version) -> list[Check]:
    checks = []
    for name in REQUIRED_PACKAGES:
        try:
            checks.append(Check(f"python:{name}", True, version(name)))
        except importlib.metadata.PackageNotFoundError:
            checks.append(Check(f"python:{name}", False, "not installed"))
    return checks


def compile_probe(run=subprocess.run) -> Check:
    source = "void _start(void) {}\n"
    command = ["clang", "-x", "c", "-", "-target", "i386-pc-win32",
               "-march=pentium3", "-nostdlib", "-ffreestanding",
               "-fuse-ld=lld-link", "-Wl,-entry:_start", "-o", "/dev/null"]
    try:
        result = run(command, input=source, text=True, capture_output=True, timeout=30)
    except (FileNotFoundError, subprocess.TimeoutExpired) as error:
        return Check("compile:i386-pc-win32", False, str(error))
    detail = (result.stderr or result.stdout or "target compiled and linked").strip()
    return Check("compile:i386-pc-win32", result.returncode == 0, detail)


def inspect_host(run_compile: bool = True) -> dict:
    checks = command_checks() + package_checks()
    prerequisites_ok = all(check.ok for check in checks)
    if run_compile:
        checks.append(compile_probe() if prerequisites_ok else
                      Check("compile:i386-pc-win32", False, "skipped: prerequisites missing"))
    return {
        "host": {"system": platform.system(), "release": platform.release(),
                 "machine": platform.machine(), "python": platform.python_version()},
        "valid_build_host": all(check.ok for check in checks),
        "checks": [asdict(check) for check in checks],
        "note": "This checks the compiler/tooling path only; patching requires a legally obtained cachebeta.xbe.",
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--json", action="store_true", help="emit machine-readable output")
    parser.add_argument("--no-compile", action="store_true", help="skip the target compile/link probe")
    args = parser.parse_args()
    report = inspect_host(not args.no_compile)
    if args.json:
        print(json.dumps(report, indent=2))
    else:
        print(f"Host: {report['host']['system']} {report['host']['machine']} (Python {report['host']['python']})")
        for check in report["checks"]:
            print(f"[{'PASS' if check['ok'] else 'FAIL'}] {check['name']}: {check['detail']}")
        print("Verdict:", "VALID" if report["valid_build_host"] else "NOT READY")
        print(report["note"])
    return 0 if report["valid_build_host"] else 1


if __name__ == "__main__":
    sys.exit(main())
