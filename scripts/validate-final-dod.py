#!/usr/bin/env python3
"""Validate the tracked final Cinnamon/X11 Computer Use DoD report."""
from __future__ import annotations

import argparse
import json
from pathlib import Path
import re
import sys
from typing import Any

REPO_DIR = Path(__file__).resolve().parents[1]
DEFAULT_DOCUMENT = REPO_DIR / "docs/final-architecture-dod.md"

REQUIRED_DECISION_TOPICS = [
    "backend_identity",
    "window_model",
    "command_execution_seam",
    "shell_out_vs_native_x11",
    "diagnostics_readiness",
    "input_safety_invariant",
    "pointer_keyboard_backend_priority",
    "atspi_correlation",
    "screenshot_coordinate_model",
    "get_app_state_integration",
    "plugin_source_overlay_strategy",
    "licensing_upstream_policy",
    "cinnamon_extension_wayland_scope",
]

REQUIRED_CAPABILITY_ROWS = [
    "doctor_capabilities",
    "list_windows",
    "focused_window",
    "focus_window_verification",
    "safe_target_resolution",
    "get_app_state_x11_context",
    "keyboard_type_text",
    "keyboard_press_key",
    "pointer_click",
    "pointer_scroll",
    "pointer_drag",
    "stock_activate_window",
    "stock_mousemove_absence",
    "cinnamon_x11_input_backend",
    "screenshot_global_provider",
    "screenshot_window_crop_bounds",
    "atspi_tree",
    "atspi_action_value_set",
    "terminal_context_selectors",
    "standalone_codex_mcp_plugin",
    "source_overlay",
    "e2e_from_codex",
    "uninstall_rollback",
]

STATUS_VALUES = {"pass", "degraded"}
REQUIRED_FOR_VALUES = {"yes", "should", "yes if available"}
ADR_REFERENCE_DOCS = [
    REPO_DIR / "ARCHITECTURE.md",
    REPO_DIR / "adr" / "README.md",
]


class DodFailure(RuntimeError):
    pass


def extract_labeled_json(text: str, label: str) -> Any:
    pattern = re.compile(r"```(?:json\s+)?" + re.escape(label) + r"\s*\n(.*?)\n```", re.DOTALL)
    match = pattern.search(text)
    if not match:
        raise DodFailure(f"missing labeled JSON block: {label}")
    try:
        return json.loads(match.group(1))
    except json.JSONDecodeError as exc:
        raise DodFailure(f"invalid JSON in {label}: {exc}") from exc


def normalize_decision_topics(decisions: Any) -> set[str]:
    if not isinstance(decisions, list):
        raise DodFailure("final-dod-decisions must be a JSON array")
    topics: set[str] = set()
    for item in decisions:
        if isinstance(item, str):
            topics.add(item)
        elif isinstance(item, dict) and isinstance(item.get("id"), str):
            topics.add(item["id"])
        else:
            raise DodFailure(f"invalid decision topic entry: {item!r}")
    return topics


def normalize_capability_rows(rows: Any) -> dict[str, dict[str, Any]]:
    if not isinstance(rows, list):
        raise DodFailure("final-dod-capability-matrix must be a JSON array")
    by_id: dict[str, dict[str, Any]] = {}
    for row in rows:
        if not isinstance(row, dict) or not isinstance(row.get("id"), str):
            raise DodFailure(f"invalid capability row: {row!r}")
        row_id = row["id"]
        if row_id in by_id:
            raise DodFailure(f"duplicate capability row: {row_id}")
        by_id[row_id] = row
    return by_id


def validate_decisions(decisions: Any, errors: list[str]) -> None:
    topics = normalize_decision_topics(decisions)
    for topic in REQUIRED_DECISION_TOPICS:
        if topic not in topics:
            errors.append(f"missing decision topic: {topic}")


def non_empty_string(value: Any) -> bool:
    return isinstance(value, str) and bool(value.strip())


def validate_rows(rows: Any, errors: list[str]) -> None:
    by_id = normalize_capability_rows(rows)
    for row_id in REQUIRED_CAPABILITY_ROWS:
        if row_id not in by_id:
            errors.append(f"missing capability row: {row_id}")
            continue
        row = by_id[row_id]
        required_for_v1 = row.get("required_for_v1")
        if required_for_v1 not in REQUIRED_FOR_VALUES:
            errors.append(f"{row_id}: required_for_v1 must be one of {sorted(REQUIRED_FOR_VALUES)}")
        status = row.get("status")
        if status not in STATUS_VALUES:
            errors.append(f"{row_id}: status must be pass or degraded")
        evidence = row.get("evidence")
        if not isinstance(evidence, list) or not any(non_empty_string(item) for item in evidence):
            errors.append(f"{row_id}: evidence must contain at least one non-empty entry")
        degraded_behavior = row.get("degraded_behavior")
        if status == "degraded" and not non_empty_string(degraded_behavior):
            errors.append(f"{row_id}: degraded_behavior is required when status is degraded")
        capability = row.get("capability")
        if not non_empty_string(capability):
            errors.append(f"{row_id}: capability must be non-empty")


def validate_text_sections(text: str, errors: list[str]) -> None:
    required_snippets = [
        "Research refresh",
        "2026-05-31",
        "License refresh",
        "runtime command invocation",
        "Final answer",
        "yes for Cinnamon/X11",
        "Cinnamon Wayland",
        "unsafe targeted input without verification",
        "scripts/validate-final-dod.py",
        "CODEX_DESKTOP_LINUX_FULL_PATH",
    ]
    for snippet in required_snippets:
        if snippet not in text:
            errors.append(f"missing required text: {snippet}")


def collect_top_level_adr_references(text: str) -> set[str]:
    return {
        match.group(1)
        for match in re.finditer(r"`?(adr/[0-9]{4}[-A-Za-z0-9_.]+\.md)`?", text)
    }


def validate_adr_references(errors: list[str]) -> None:
    for doc in ADR_REFERENCE_DOCS:
        if not doc.is_file():
            errors.append(f"missing ADR reference document: {doc.relative_to(REPO_DIR)}")
            continue
        text = doc.read_text(encoding="utf-8")
        for ref in sorted(collect_top_level_adr_references(text)):
            if not (REPO_DIR / ref).is_file():
                errors.append(f"missing referenced ADR file: {ref}")


def validate_document(path: Path) -> list[str]:
    if not path.is_file():
        raise DodFailure(f"final DoD document not found: {path}")
    text = path.read_text(encoding="utf-8")
    errors: list[str] = []
    validate_text_sections(text, errors)
    decisions = extract_labeled_json(text, "final-dod-decisions")
    rows = extract_labeled_json(text, "final-dod-capability-matrix")
    validate_decisions(decisions, errors)
    validate_rows(rows, errors)
    validate_adr_references(errors)
    return errors


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--document", type=Path, default=DEFAULT_DOCUMENT)
    args = parser.parse_args(argv)

    try:
        errors = validate_document(args.document)
    except DodFailure as exc:
        print(str(exc), file=sys.stderr)
        return 1

    if errors:
        for error in errors:
            print(error, file=sys.stderr)
        return 1
    print(f"Final X11 Computer Use DoD complete: {args.document}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
