## Context Read

- `CONSTITUTION.md` — Rust/Cargo stack, OpenSpec validation, secret handling, verification rules, and automatic safe checkpoint discipline.
- `CONTEXT.md` — glossary terms: `x11-ewmh`, `Active window`, `Focus verification`, `FocusNotVerified`, `Standalone plugin`, `Source overlay`, `TDD slice`.
- `ARCHITECTURE.md`, `adr/README.md`, in-force ADRs 0001, 0003, 0005, 0006, 0007 — lifecycle, mandatory grill/TDD gates, automatic checkpoints, optional Claude review disabled by session state.
- `backlog/00-research-reuse-map.md` and `backlog/07-targeted-input-safety.md` — milestone order, safety invariant, research requirements, and targeted keyboard input acceptance criteria.
- Existing specs: `openspec/specs/doctor-cli/spec.md`, `openspec/specs/x11-window-listing/spec.md`, `openspec/specs/x11-active-window-focus/spec.md`, `openspec/specs/standalone-codex-mcp-plugin/spec.md`, `openspec/specs/x11-integration-contract/spec.md`.
- Current change artifacts: `proposal.md` and spec deltas under `specs/`.
- Standalone code: `src/cli.rs`, `src/doctor.rs`, `src/focus.rs`, `src/list_windows.rs`, `src/mcp.rs`, tests under `tests/`.
- Target checkout research: `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full/computer-use-linux/src/windowing/{types.rs,registry.rs,target.rs}`, `server.rs`, `diagnostics.rs`, `remote_desktop.rs`, and `screenshot.rs` at commit `1a6f343ee7ce597019a4c573259c2a9838376874`.
- External/source research: `gh repo view` metadata for relevant Linux Computer Use projects and local `xdotool` manpage `SENDEVENT NOTES` / Unicode warning.

## Plan Summary

- Extend the standalone path first; do not patch the moving Codex Desktop Linux target checkout in this change.
- Add `type-text` and `press-key` CLI JSON commands plus MCP `x11_type_text` and `x11_press_key` wrappers.
- Require a unique current window target before input; support `window_id`, title, `wm_class`, and pid selectors where the current listing can resolve exactly one window.
- Reuse the existing focus activation/verification safety boundary and call active-context `xdotool type/key --clearmodifiers` only after exact focus verification.
- Treat missing target, ambiguous target, stale target, failed focus verification, missing backend, and Unicode/layout limitations as structured safe failures/degraded evidence.

## Question Loop

### Q1: Should the next task be numeric file `06` or milestone `07`?

- Recommended answer: use `backlog/07-targeted-input-safety.md` now.
- Rationale: `backlog/00-research-reuse-map.md` explicitly reorders source overlay file `06` later, after keyboard, pointer, AT-SPI, screenshot, and `get_app_state` core behavior stabilize.
- Resolution: answered from repository context; no user question needed. The change is `add-x11-targeted-input-safety`.

### Q2: Is `xdotool --window` a safe targeted-input boundary?

- Recommended answer: no.
- Rationale: the local `xdotool` manpage states direct `--window` key/mouse events use XSendEvent and many programs reject or ignore those events; active-window XTEST input is a different path. Therefore safety must be focus/active-window verification, not direct-to-window events.
- Resolution: answered by local docs/research. Specs require active-context input after verified focus and forbid treating `--window` as the safety boundary.

### Q3: Should this change add a source overlay keyboard backend or standalone plugin keyboard tools?

- Recommended answer: standalone plugin keyboard tools only.
- Rationale: target `server.rs` already has `focus_target_for_input()` for stock `type_text`/`press_key`; backlog/00 says source overlay should run later and not hold a long-lived patch against moving upstream before core behavior stabilizes. The standalone plugin is the immediate feedback loop from backlog/05.
- Resolution: answered by target/source code and backlog context. Source overlay writes are out of scope.

### Q4: Should global/unverified keyboard injection be allowed as a fallback?

- Recommended answer: no for these safe targeted commands.
- Rationale: `ydotool`, `xdotool`, and `/dev/uinput` input paths are global desktop injectors, not OS-isolated per-window channels. A command whose purpose is targeted safety must refuse when it cannot verify focus.
- Resolution: answered by architecture/backlog safety invariant. Specs require `MissingTarget` and `FocusNotVerified` refusals with `input_sent: false`.

### Q5: Can full Unicode text support be promised for xdotool?

- Recommended answer: not until proven; record evidence or degraded limitation.
- Rationale: `xdotool` docs warn unusual symbols under non-US keybindings may send the wrong character. Backlog requires Cyrillic and non-BMP/emoji evidence.
- Resolution: specs and test-plan must record Cyrillic/non-BMP behavior without bypassing focus verification.

## Resolved Terms

- `targeted keyboard input`: keyboard input whose intended recipient is a specific current X11/EWMH window and which is permitted only after exact active-window verification.
- `active-context input`: keyboard input sent to the current active window without `xdotool --window`; safe only because a preceding focus check proved the active window identity.
- Existing glossary terms (`Active window`, `Focus verification`, `FocusNotVerified`, `Standalone plugin`, `Source overlay`) already cover the domain language. No `CONTEXT.md` update is needed.

## Document Updates Applied

- Proposal records the research refresh, scoped standalone implementation, and target/source findings.
- Spec delta `x11-targeted-input-safety` captures CLI/MCP behavior, unique target resolution, refusal paths, active-context xdotool semantics, and Unicode evidence.
- Spec delta `standalone-codex-mcp-plugin` updates deterministic tool listing from four to six project-owned `x11_*` tools.

## Document Updates Required Before Next Gate

- None.

## ADR Candidates

- No new durable ADR candidate. The core invariant "verify focus before targeted input" is already part of backlog/00, `x11-active-window-focus`, and current architecture language; this change applies it to standalone keyboard tools without changing project architecture.

## Open Questions

None
