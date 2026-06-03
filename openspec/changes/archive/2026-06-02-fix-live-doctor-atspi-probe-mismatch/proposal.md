## Why

Independent live verification found that `doctor --json` can still report AT-SPI tree extraction as unavailable while the same built binary successfully returns a high-confidence `accessibility-tree --window-id ... --json` match with a non-empty tree for the focused Codex window. This keeps the doctor readiness surface from faithfully reflecting the project’s canonical accessibility collector facts.

## What Changes

- Make the doctor AT-SPI probe use the same live collector semantics as the window-scoped accessibility-tree path when determining tree extraction availability.
- Preserve safe degradation when AT-SPI is unavailable, ambiguous, bridge-disabled, or too slow, but stop reporting `collector_unavailable` when the collector actually returns usable candidates.
- Add regression coverage for the live-probe mismatch shape: a collector output that is valid for accessibility-tree must produce doctor `tree_available=true`, candidate count, and `match_outcome=tree_available`.
- Add verification evidence that `doctor --json` and `accessibility-tree --window-id ... --json` agree on AT-SPI availability in a controlled or live-safe path.

## Capabilities

- Modified capability: `doctor-cli` — doctor accessibility facts and readiness diagnostics.
- Modified capability: `x11-atspi-window-correlation` — shared collector/probe contract used by both doctor and window-scoped accessibility-tree paths.

## Impact

- Code: `src/doctor.rs`, `src/accessibility.rs`, and doctor/accessibility tests under `tests/` or module tests.
- CLI/API: additive or corrective JSON values only; no field removals or breaking shape changes.
- Runtime: doctor remains read-only and must not require secrets, screenshots, input injection, target checkout mutation, or external credentials.
- Architecture/ADR constraints: preserve the Cinnamon/X11 baseline from ADR 0009, the AT-SPI confidence/degraded behavior from existing specs, and the non-invasive doctor contract in `CONSTITUTION.md` and `ARCHITECTURE.md`.
- Verification: run `openspec validate`, `make fmt`, `make check`, `make test`, and a live-safe `doctor --json`/`accessibility-tree --json` comparison when a suitable X11 target window is available.
