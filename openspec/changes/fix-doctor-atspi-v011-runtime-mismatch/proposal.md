## Why

After the v0.1.1 release, `doctor --json` still reports AT-SPI tree extraction as degraded even when the same installed binary can return a successful `accessibility-tree --window-id ... --json` tree for the focused Codex window. This creates a runtime mismatch in the readiness surface: doctor treats environment hints or its bounded probe as unavailable while the canonical collector path proves tree extraction works.

## What Changes

- Align the bounded doctor AT-SPI probe success semantics with the `accessibility-tree` collector path.
- Stop short-circuiting doctor to degraded solely because `NO_AT_BRIDGE=1` is present when the actual collector returns valid candidates or a tree.
- Preserve degraded doctor diagnostics only when the collector is genuinely unavailable, invalid, ambiguous for the required semantics, or times out.
- Add regression tests for `NO_AT_BRIDGE=1`, `env -u NO_AT_BRIDGE`, valid collector output, invalid collector output, and timeout/unavailable collector cases.

## Capabilities

- Modified capability: `doctor-cli` — doctor accessibility facts, AT-SPI diagnostic state, and readiness degraded reasons.
- Modified capability: `x11-atspi-window-correlation` — shared collector/probe contract consumed by doctor and `accessibility-tree`.

## Impact

- Code: likely `src/doctor.rs`, `src/accessibility.rs`, and public CLI regression tests such as `tests/doctor_cli.rs`.
- CLI/API: corrective JSON values only; no field removal or breaking shape change. Existing fields such as `tree_available`, `diagnostic_state`, `match_outcome`, and candidate counts keep their names.
- Runtime: doctor remains read-only and non-invasive; no screenshots, input injection, target-checkout mutation, secrets, or external credentials.
- Architecture/ADR constraints: preserve ADR 0009's safe AT-SPI degradation on absence/ambiguity and ADR 0011's non-secret bridge-environment facts, but correct the old spec behavior so environment hints cannot override proven collector success.
- Verification: strict OpenSpec validation, `make fmt`, `make check`, `make test`, and a live-safe `doctor --json` / `accessibility-tree --window-id ... --json` comparison when X11 and a suitable focused window are available.
