## Context Read

- Change artifacts: `proposal.md`, all five `specs/*/spec.md` deltas, `grill.md`, and `design.md`.
- Project rules and context: `CONSTITUTION.md`, `CONTEXT.md`, `ARCHITECTURE.md`, `adr/README.md`.
- In-force ADRs emphasized by the design: ADR 0008, ADR 0009, ADR 0010.
- Evidence: `target/e2e-logs/live-functional/acceptance-summary.md`, `app-state-text.json`, `safe_apps.py`, and `safe_apps.events.log`.
- Relevant implementation/test surfaces: `src/input.rs`, `src/accessibility.rs`, `src/list_windows.rs`, `src/target_window.rs`, `src/app_state.rs`, `src/mcp.rs`, `scripts/e2e/codex-x11-e2e.py`, and the corresponding tests under `tests/`.

## Design Summary

- Keyboard design keeps exact focus verification before all routes, adds key alias normalization, treats semantic `xdotool` stderr as failure, uses Unicode keysyms first for non-ASCII, and gates clipboard fallback with restoration diagnostics.
- AT-SPI design replaces substring class matching with token-boundary matching, adds one-target `xprop` enrichment, and expands candidate diagnostics without lowering confidence thresholds.
- Overlay design introduces a standalone X11 provider using `x11rb` or helper-owned override-redirect border windows, with project overlay windows filtered from target listings.
- App-state/e2e design fixes `diagnostics.layers` extraction, removes base64 screenshot payloads from summaries, and adds exact Cyrillic, GTK AT-SPI, and overlay lifecycle checks to live mode.

## Question Loop

### Question 1: Does the design conflict with ADR 0008 root-coordinate semantics?

- Recommended answer: No.
- Rationale: Overlay borders, pointer targets, screenshots, and bounds validation all remain in X11 root/global pixel coordinates. The design explicitly uses target bounds from the existing `x11-ewmh` listing and does not introduce client-local coordinate semantics.
- Resolution: Answered from ADR 0008 and design text. No artifact changes required.

### Question 2: Does the keyboard design weaken the ADR 0009 focus/input safety boundary?

- Recommended answer: No.
- Rationale: The design keeps target resolution and exact active-window verification before `xdotool key`, `xdotool type`, Unicode keysyms, and clipboard paste. It also explicitly rejects `xdotool --window` and `ydotool` as primary fixes.
- Resolution: Answered from ADR 0009 and spec/design. No artifact changes required.

### Question 3: Is clipboard fallback too risky for a safe targeted input tool?

- Recommended answer: Accept as an explicit fallback, not as a silent primary route.
- Rationale: Exact non-ASCII text may require a paste route if X11 keysyms fail on a user's layout/input method. The design mitigates risk by requiring verified focus, route diagnostics, previous clipboard restoration when possible, and failure/degraded warnings when restoration cannot be verified.
- Resolution: Answered from user scope and design. Per-change ADR must record the route ordering and clipboard trade-off.

### Question 4: Is a standalone overlay provider a hard dependency for target-window success?

- Recommended answer: No.
- Rationale: Existing specs and live evidence already separate target-state correctness from optional visual overlay. The provider should improve UX/readiness evidence, but target save/release semantics must remain usable when overlay display is unavailable.
- Resolution: Answered from existing spec and design. Per-change ADR must record provider choice and non-blocking semantics.

### Question 5: Does target-scoped xprop enrichment risk performance or privacy regressions?

- Recommended answer: Accept bounded target-scoped enrichment only.
- Rationale: One `xprop -id <target>` call during a user-requested target correlation is bounded and diagnosable. Unbounded normal list-windows fan-out remains forbidden. Window titles/classes are already part of listing evidence, and secret handling rules still forbid copying secrets into tracked artifacts.
- Resolution: Answered from design and constitution. No artifact changes required.

## Design Findings

- **Safety alignment:** Design preserves the in-force safety boundaries: verified focus before input; no bounds-only AT-SPI; no arbitrary subtree on ambiguity; overlay non-blocking; no global plugin masquerade.
- **Verification feasibility:** Each slice has a plausible RED/GREEN public interface: CLI/MCP JSON reports for keyboard, accessibility, target-window overlay, app-state, and e2e evidence files.
- **Dependency risk:** `xclip`/`xsel`, PyGObject/GTK, and `x11rb`/helper process availability must be represented as readiness/degraded diagnostics, not hidden assumptions.
- **State cleanup risk:** Overlay lifecycle needs stale-target and release-all cleanup tests to avoid abandoned project-owned windows.
- **Evidence risk:** Live summaries must sanitize screenshot `data_url` by default; otherwise tracked evidence can become excessively large and noisy.

## Document Updates Applied

None. The proposal, specs, grill, and design already include the design-review findings.

## Document Updates Required Before Next Gate

None.

## ADR Candidates

- Per-change ADR required:
  - Unicode input route ordering: verified focus -> X11 Unicode keysyms -> explicit recoverable clipboard fallback.
  - Standalone overlay provider: `x11rb` or helper override-redirect border windows, non-focus, project-owned identity, non-blocking failure.
- Durable ADR not required now because the design stays inside ADR 0008/0009/0010. Revisit durable ADR only if apply selects a hard-to-reverse dependency or changes final v1 baseline claims.

## Open Questions

None.
