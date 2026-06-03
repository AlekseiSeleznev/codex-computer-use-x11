<!-- Planning artifact only. Do not implement these tasks until `/opsx:apply` or an explicit implementation request starts from the completed, checkpointed planning state. Preserve CONSTITUTION.md, ARCHITECTURE.md, ADR 0008/0009/0010, and the TDD slices in test-plan.md. Do not expose, stage, or commit secret values. -->

## 1. Keyboard aliases/stderr

- [x] 1.1 RED: Add a public CLI/fake-backend test proving `press-key --key Enter` normalizes to `Return` and `press-key --key Backspace` normalizes to `BackSpace`, then run the targeted test and record the expected failure.
- [x] 1.2 GREEN: Implement key alias normalization and JSON diagnostics for requested/normalized keys with no `xdotool --window` route.
- [x] 1.3 RED: Add a fake `xdotool` test where stderr contains `No such key name` or `Ignoring it` with exit 0, then run the targeted test and record the expected failure.
- [x] 1.4 GREEN: Treat semantic `xdotool` stderr refusal phrases as `InputBackendFailed` with `success=false` and `input_sent=false`.
- [x] 1.5 REFACTOR: Keep normalization and semantic-stderr detection reusable across CLI and MCP paths; run the slice test plus relevant targeted-input tests.

## 2. Unicode keysyms + fallback decision

- [x] 2.1 RED: Add a test proving non-ASCII `type-text` after verified focus must choose a Unicode keysym route and emit `U041F/U0440/...` for Cyrillic, then record the expected failure against the current literal `xdotool type` route.
- [x] 2.2 GREEN: Implement active-context Unicode keysym typing after exact focus verification and report route/args without using `xdotool --window`.
- [x] 2.3 RED: Add a fallback test where the Unicode keysym route cannot prove exact fidelity and clipboard fallback must be explicit/restorable, then record the expected failure.
- [x] 2.4 GREEN: Implement `clipboard-paste` fallback through `xclip` or `xsel` with previous clipboard restore diagnostics and failure/degraded warning on restore failure.
- [x] 2.5 REFACTOR: Ensure `ydotool` remains non-primary for Unicode, focus verification gates all routes, and MCP `x11_type_text` exposes the same JSON route diagnostics.

## 3. AT-SPI token matching/enrichment

- [x] 3.1 RED: Add matcher tests proving `Tk`/`tk` does not class-match `gtk3`, `ibus-ui-gtk3`, or `xdg-desktop-portal-gtk`; record expected failure.
- [x] 3.2 GREEN: Replace substring class/app matching with token-boundary/exact-token matching for short tokens while preserving legitimate exact app-token matches.
- [x] 3.3 RED: Add a correlation test requiring one target-scoped `xprop -id <target>` enrichment and parsed `_NET_WM_PID`, `WM_CLIENT_MACHINE`, names, `WM_CLASS`, and `_NET_WM_WINDOW_TYPE`; record expected failure.
- [x] 3.4 GREEN: Implement bounded target-scoped xprop enrichment for accessibility correlation only; keep normal list-windows unbounded per-window enrichment disabled.
- [x] 3.5 RED: Add no-match/ambiguous tests requiring candidate score reasons and `missing_signals`, including a bounds-only candidate that must not return a subtree; record expected failure.
- [x] 3.6 GREEN: Emit candidate reasons and missing-signal diagnostics while preserving no arbitrary subtree on ambiguity/low confidence.
- [x] 3.7 REFACTOR: Keep scoring readable and threshold constants documented; run accessibility CLI/MCP tests.

## 4. GTK fixture

- [x] 4.1 RED: Extend e2e harness tests/fixtures to require a GTK AT-SPI-positive safe fixture row, then run fake harness validation and record expected failure.
- [x] 4.2 GREEN: Add GTK fixture support or documented GTK-safe app selection with stable title and accessible control assertions.
- [x] 4.3 GREEN: Keep Tkinter fixtures for keyboard/pointer and document/report Tk AT-SPI limitations separately from GTK AT-SPI pass/degraded evidence.
- [x] 4.4 REFACTOR: Ensure GTK dependency absence is explicit degraded evidence, not a silent pass; update docs if fixture invocation changes.

## 5. Overlay provider

- [x] 5.1 RED: Add target-window tests requiring `overlay.shown=true` for a successful provider, no focus steal, and warning-only behavior on provider failure; record expected failure.
- [x] 5.2 GREEN: Introduce an overlay provider boundary with a fake provider for tests and a real v1 provider using `x11rb` or helper-owned non-focus override-redirect border windows.
- [x] 5.3 RED: Add listing/target-resolution tests proving title/class `codex-computer-use-x11-overlay` windows are excluded from ordinary targets; record expected failure.
- [x] 5.4 GREEN: Mark project overlay windows with title/class `codex-computer-use-x11-overlay` and filter them from `x11_list_windows`, target resolution, and app-state target selectors.
- [x] 5.5 RED: Add release/stale cleanup tests requiring per-target and release-all overlay hide diagnostics; record expected failure.
- [x] 5.6 GREEN: Hide overlays on `release-window`, `release-window --all`, and stale target cleanup while keeping hide failures as warnings.
- [x] 5.7 REFACTOR: Keep overlay lifecycle isolated from target-state correctness; run target-window and listing tests.

## 6. Evidence/app-state cleanup

- [x] 6.1 RED: Add app-state/evidence summary test proving layer extraction reads `diagnostics.layers` and treats missing `diagnostics.layers` as degraded/failure evidence; record expected failure if any summary path reads top-level `.layers`.
- [x] 6.2 GREEN: Fix summary extraction to use `diagnostics.layers` and emit concrete diagnostics on missing layer data.
- [x] 6.3 RED: Add no-screenshot-data summary test proving base64 `screenshot.data_url` is omitted while MIME/source/dimensions/status remain; record expected failure.
- [x] 6.4 GREEN: Implement summary/evidence sanitization mode that preserves screenshot metadata and removes large data URLs by default in live summaries.
- [x] 6.5 GREEN: Update readiness/recommended-next-step docs/messages so incomplete RemoteDesktop portal is degraded/report-only for the working X11/EWMH path and not the main blocker when X11 focus/input/screenshot works.
- [x] 6.6 REFACTOR: Keep raw app-state JSON compatibility unless an explicit flag requests sanitized output; run app-state and harness-script tests.

## 7. Live e2e harness

- [x] 7.1 RED: Add live/fake harness validation requiring exact Cyrillic text value evidence, not only key events; record expected degraded/failure against current live behavior.
- [x] 7.2 GREEN: Extend safe fixtures and harness checks to read actual typed value and mark keyboard Unicode pass/degraded with route evidence.
- [x] 7.3 RED: Add harness validation requiring GTK AT-SPI matched subtree evidence and Tk limitation reporting; record expected failure/degraded until fixture support exists.
- [x] 7.4 GREEN: Wire GTK AT-SPI fixture check into live capability matrix with concrete pass/degraded reason and evidence path.
- [x] 7.5 RED: Add harness validation requiring overlay shown/release/listing exclusion evidence; record expected failure until overlay provider exists.
- [x] 7.6 GREEN: Wire overlay lifecycle checks into live mode and capability matrix.
- [x] 7.7 GREEN: Update capability matrix validation so every v1 group has `pass` or `degraded` with concrete evidence paths/tool names/reasons; missing rows fail.
- [x] 7.8 REFACTOR: Keep live harness safe-window-only, sanitized, non-destructive, and separate from manual UI STOP-gate evidence.

## 8. Verification and readiness gates

- [x] 8.1 Run `openspec validate --all --strict --json` and record result.
- [x] 8.2 Run `make fmt`, `make check`, and `make test` and record results or exact blockers.
- [x] 8.3 Run affected fake e2e matrix validation and record evidence path.
- [x] 8.4 Run live hardening e2e on a live X11 desktop when available; if unavailable, record explicit limitation/degraded evidence rather than fabricating pass.
- [x] 8.5 Confirm `git status --short` and ensure no secret/local files are staged.
- [x] 8.6 Update the test-plan Evidence Log during apply with RED/GREEN command outputs or evidence references before marking tasks complete.
