#!/usr/bin/env python3
"""Unified pre-commit verification script for the firmware repository.

Executes in order:
  1. Parallel codebase validations (`tools/validation/validate_all.py`: cargo fmt, clippy, AST, oxlint JS)
  2. Rust unit & integration tests (`cargo nextest run`, falling back to `cargo test`)
  3. Parallel JavaScript ES module unit tests (`node --test`)
  4. Parallel Python host test suites (`pytest -n <workers>`)
  5. Embedded firmware & host tool debug builds (`build_firmware.sh --debug-only`, `build_host_tools.sh --debug`)
  6. Automatic target hardware discovery & async on-device `smoke` + `integration` suites (`tools/host.py test`)
"""

import argparse
import glob
import os
from pathlib import Path
import shutil
import subprocess
import sys
import time
from typing import Dict, List, Optional, Sequence

TARGET_NOFILE_LIMIT: int = 65536
MAX_PARALLEL_WORKERS: int = 8


def raise_nofile_limit() -> None:
    """Raise open file descriptor soft limit toward `TARGET_NOFILE_LIMIT` when supported."""
    if os.name != "posix":
        return
    import resource

    try:
        soft, hard = resource.getrlimit(resource.RLIMIT_NOFILE)
        desired = min(TARGET_NOFILE_LIMIT, hard) if hard != resource.RLIM_INFINITY else TARGET_NOFILE_LIMIT
        if soft < desired:
            resource.setrlimit(resource.RLIMIT_NOFILE, (desired, hard))
    except (OSError, ValueError):
        pass


def build_verification_env() -> Dict[str, str]:
    """Build single-threaded BLAS/OMP environment dictionary inheriting the active conda environment."""
    env = dict(os.environ)
    env["OPENBLAS_NUM_THREADS"] = "1"
    env["MKL_NUM_THREADS"] = "1"
    env["OMP_NUM_THREADS"] = "1"
    return env


def run_step(cmd: Sequence[str], *, cwd: Path, env: Dict[str, str]) -> None:
    """Run a verification command synchronously and exit immediately on non-zero status."""
    proc = subprocess.run(list(cmd), cwd=str(cwd), env=env, check=False)
    if proc.returncode != 0:
        sys.exit(proc.returncode)


def parse_args(argv: Optional[Sequence[str]] = None) -> argparse.Namespace:
    """Parse CLI options for `./tools/verify.py`."""
    parser = argparse.ArgumentParser(
        prog="tools/verify.py",
        description="Unified pre-commit verification suite for the firmware repository.",
    )
    parser.add_argument(
        "--skip-device-tests",
        "--host-only",
        dest="skip_device_tests",
        action="store_true",
        help="Skip automatic target hardware discovery and on-device pytest suites.",
    )
    parser.add_argument(
        "--bringup",
        "--device-all",
        dest="bringup",
        action="store_true",
        help="Include full bringup scenario tier alongside smoke and integration.",
    )
    parser.add_argument(
        "--project",
        default="all",
        help="Filter on-device tests by @device_test(project=...) (e.g. cat_detector, carrier_board).",
    )
    parser.add_argument(
        "--scenario",
        default="default",
        help="Pytest scenario marker expression for on-device suites (default: smoke or integration).",
    )
    parser.add_argument(
        "--board",
        "--device",
        dest="board",
        default="auto",
        help="Board glob pattern filter for on-device suites (e.g. auto, rp2040*, carrier_board*).",
    )
    parser.add_argument(
        "--interactive",
        action="store_true",
        help="Enable interactive operator prompts for manual bringup steps.",
    )
    return parser.parse_args(argv)


def main(argv: Optional[Sequence[str]] = None) -> int:
    """Execute full firmware pre-commit verification pipeline."""
    args = parse_args(argv)
    start_time = time.monotonic()
    repo_root = Path(__file__).resolve().parent.parent

    raise_nofile_limit()
    env = build_verification_env()
    python_exe = sys.executable

    print("Running parallel codebase validations...")
    run_step([python_exe, "tools/validation/validate_all.py"], cwd=repo_root, env=env)

    print("Running test suite...")
    if shutil.which("cargo-nextest", path=env.get("PATH")):
        run_step(["cargo", "nextest", "run"], cwd=repo_root, env=env)
    else:
        run_step(["cargo", "test"], cwd=repo_root, env=env)

    cpu_count = os.cpu_count() or 4
    workers = min(cpu_count, MAX_PARALLEL_WORKERS)

    js_tests = sorted(glob.glob(str(repo_root / "tools" / "tests" / "js" / "*.test.mjs")))
    if js_tests:
        print("   Running parallel JavaScript unit tests...")
        run_step(["node", "--test", f"--test-concurrency={workers}", *js_tests], cwd=repo_root, env=env)

    print("   Running Python tests...")
    run_step([python_exe, "-m", "pytest", "-n", str(workers)], cwd=repo_root, env=env)

    print("Building target binaries and host tools (Debug)...")
    run_step(["./tools/build/build_firmware.sh", "--debug-only"], cwd=repo_root, env=env)
    run_step(["./tools/build/build_host_tools.sh", "--debug"], cwd=repo_root, env=env)

    if not args.skip_device_tests:
        scenario_expr = "smoke or integration or bringup" if args.bringup else args.scenario
        print("Checking for connected target hardware and running on-device suites...")
        device_cmd: List[str] = [
            python_exe,
            "tools/host.py",
            "test",
            "--project",
            args.project,
            "--scenario",
            scenario_expr,
            "--board",
            args.board,
        ]
        if args.interactive:
            device_cmd.append("--interactive")
        run_step(device_cmd, cwd=repo_root, env=env)
    else:
        print("ℹ️  Skipping on-device test discovery (--skip-device-tests).")

    elapsed = int(time.monotonic() - start_time)
    print(f"Verification PASSED in {elapsed}s")
    return 0


if __name__ == "__main__":
    sys.exit(main())
