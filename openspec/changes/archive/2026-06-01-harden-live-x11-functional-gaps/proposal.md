## Why

The 2026-06-01 live functional verification proved the Cinnamon/X11 provider can drive focus, pointer, ASCII keyboard, screenshots, and target context, but it also exposed concrete degraded gaps in Unicode keyboard fidelity, AT-SPI correlation diagnostics/fixtures, standalone visual overlays, and readiness evidence summarization. This change plans those hardening fixes without implementing code until the full OpenSpec planning gate set is complete.

## What Changes

- Harden targeted keyboard semantics:
  - Normalize common key aliases such as `Enter` -> `Return` and `Backspace` -> `BackSpace` before invoking X11 key injection.
  - Treat `xdotool` stderr phrases such as `No such key name` and `Ignoring it` as backend failure even if `xdotool` exits with status 0.
  - Plan a verified-focus Unicode text route for non-ASCII text through X11 Unicode keysyms such as `U041F` / `U0440` before any fallback.
  - Plan an explicit clipboard-paste fallback only when the keysym route cannot provide exact fidelity, with previous clipboard restoration and `route=clipboard-paste` diagnostics.
  - Preserve the ADR 0009 rule that `xdotool --window` direct events are not a trusted targeted-safety boundary and that `ydotool` is not the primary Unicode fix.
- Harden AT-SPI correlation:
  - Fix short token matching so `Tk` / `tk` cannot substring-match unrelated names such as `gtk3`.
  - Add token-boundary/exact-token matching for short class/app tokens.
  - Add targeted per-window `xprop -id <target>` enrichment for the requested target only, including `_NET_WM_PID`, `WM_CLIENT_MACHINE`, `WM_NAME` / `_NET_WM_NAME`, `WM_CLASS`, and `_NET_WM_WINDOW_TYPE`.
  - Keep unbounded per-window `xprop` listing disabled for normal list-windows.
  - Add `missing_signals` and candidate score reasons to diagnostics while preserving ADR 0009's no-arbitrary-subtree-on-low-confidence rule.
  - Add an AT-SPI-positive GTK live fixture; keep Tkinter as keyboard/pointer fixture and document its AT-SPI limitation.
- Design and implement a standalone overlay provider boundary:
  - Preferred v1 route is `x11rb` or a helper process creating non-focus `override-redirect` border windows around target bounds.
  - Overlay windows use title/class `codex-computer-use-x11-overlay` and are excluded from target window listing.
  - `target-window --overlay` reports `overlay.shown=true` on success, while provider failure remains a warning that does not block target save.
  - `release-window` hides the associated overlay.
- Clean app-state/evidence/readiness behavior:
  - Fix summary extraction to read `diagnostics.layers` rather than a top-level `layers` field.
  - Add a no-screenshot-data / evidence-summary mode so live summaries avoid huge base64 screenshots while preserving MIME/source/size evidence.
  - Keep incomplete RemoteDesktop portal evidence degraded/report-only for the X11 path and separate optional portal diagnostics from real blockers when X11/EWMH focus/input/screenshot works.
- Expand the e2e harness:
  - Extend `scripts/e2e/codex-x11-e2e.py` live safe fixtures.
  - Verify exact Cyrillic text value, not only key events.
  - Verify a GTK AT-SPI-positive fixture.
  - Verify overlay shown, release/hide, and overlay listing exclusion.
  - Update capability matrix output to show pass/degraded rows with concrete evidence.
- Explicitly reject unsafe shortcuts:
  - No bounds-only AT-SPI match.
  - No direct `xdotool --window` as a safety boundary.
  - No unbounded per-window `xprop` spawning in normal list-windows.
  - No unrecoverable clipboard mutation.
  - No global plugin identity masquerade beyond the ADR 0010 localized takeover shim.

## Capabilities

Modified capabilities:

- `x11-targeted-input-safety` — keyboard alias normalization, stderr failure detection, Unicode keysym route, clipboard fallback diagnostics, and retained safety boundaries.
- `x11-atspi-window-correlation` — safer token matching, target-scoped X11 metadata enrichment, candidate diagnostics, and GTK-positive live fixture expectations.
- `x11-target-window-groups-overlays` — real standalone overlay provider contract, overlay window exclusion, and release/hide behavior.
- `x11-get-app-state-integration` — correct `diagnostics.layers` evidence extraction, screenshot-data suppression for summaries, and readiness wording that separates optional portal diagnostics from blockers.
- `codex-x11-e2e-test-harness` — live fixture coverage for exact Unicode text, GTK AT-SPI, overlay lifecycle, and capability matrix evidence.

## Impact

- Affected Rust code is expected in `src/input.rs`, `src/accessibility.rs`, `src/list_windows.rs`, `src/target_window.rs`, `src/app_state.rs`, `src/mcp.rs`, and related test modules under `tests/`.
- Affected harness/docs are expected in `scripts/e2e/codex-x11-e2e.py`, `docs/e2e-harness.md`, `docs/troubleshooting.md`, `docs/release-checklist.md`, and live evidence summaries under `target/e2e-logs/` when generated.
- New runtime command expectations may include `xclip` or `xsel` for the fallback route and either `x11rb` or a small helper process for overlay windows; these must be documented as runtime dependencies or optional degraded capabilities.
- The change must preserve Rust 2021, root `Makefile` verification, standalone `x11_*` tool naming, ADR 0008 X11 root-coordinate semantics, ADR 0009 safe verified-focus and no-arbitrary-AT-SPI-match rules, and ADR 0010 localized provider takeover boundaries.
- No implementation is included in this proposal. Apply must wait until `grill.md`, `design.md`, `design-review.md`, `adr.md`, `test-plan.md`, and `tasks.md` are complete and planning artifacts are checkpointed.
