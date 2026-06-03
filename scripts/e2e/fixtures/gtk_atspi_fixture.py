#!/usr/bin/env python3
"""Deterministic run-scoped GTK AT-SPI fixture for X11 e2e harness tests."""
from __future__ import annotations

import argparse
import json
import os
import signal
import time
from pathlib import Path

running = True


def stop(_signum, _frame):
    global running
    running = False


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--role", required=True)
    parser.add_argument("--title", required=True)
    parser.add_argument("--wm-class", required=True)
    parser.add_argument("--ready-file", type=Path, required=True)
    parser.add_argument("--metadata-file", type=Path, required=True)
    args = parser.parse_args()

    metadata = {
        "role": args.role,
        "toolkit": "gtk",
        "title": args.title,
        "wm_class": args.wm_class,
        "expected_accessible_control": "Apply",
        "env": {
            "GTK_MODULES": os.environ.get("GTK_MODULES", "gail:atk-bridge"),
            "NO_AT_BRIDGE": os.environ.get("NO_AT_BRIDGE"),
            "NO_AT_BRIDGE_PRESENT": "NO_AT_BRIDGE" in os.environ,
        },
        "safe_for_atspi": True,
    }
    args.metadata_file.parent.mkdir(parents=True, exist_ok=True)
    args.metadata_file.write_text(json.dumps(metadata, sort_keys=True) + "\n", encoding="utf-8")
    args.ready_file.write_text(json.dumps({"ready": True, **metadata}, sort_keys=True) + "\n", encoding="utf-8")

    signal.signal(signal.SIGTERM, stop)
    signal.signal(signal.SIGINT, stop)
    while running:
        time.sleep(0.1)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
