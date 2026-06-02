## Context Read

- `CONSTITUTION.md` — Rust/Cargo stack, OpenSpec validation, secret handling, target checkout guidance, verification rules, and automatic safe checkpoint discipline.
- `CONTEXT.md` — glossary terms: `x11-ewmh`, `Standalone plugin`, `Source overlay`, `Active window`, `Focus verification`, `FocusNotVerified`, and `TDD slice`.
- `ARCHITECTURE.md`, `adr/README.md`, and in-force ADR references — lifecycle, mandatory grill/TDD gates, automatic checkpoints, optional Claude review controls, and no-secrets boundaries. The referenced durable ADR files are not present in this checkout, so the snapshot/README are the available source of in-force rationale.
- `backlog/00-research-reuse-map.md` and `backlog/07b-pointer-click-scroll-drag.md` — milestone order, standalone-before-source-overlay posture, pointer safety invariants, research requirements, and acceptance checks.
- Current change artifacts: `proposal.md`, `specs/x11-pointer-actions/spec.md`, and `specs/standalone-codex-mcp-plugin/spec.md`.
- Existing specs: `openspec/specs/x11-window-listing/spec.md`, `openspec/specs/x11-active-window-focus/spec.md`, `openspec/specs/x11-targeted-input-safety/spec.md`, `openspec/specs/standalone-codex-mcp-plugin/spec.md`, `openspec/specs/doctor-cli/spec.md`, and `openspec/specs/x11-integration-contract/spec.md`.
- Standalone code: `src/cli.rs`, `src/input.rs`, `src/focus.rs`, `src/list_windows.rs`, `src/mcp.rs`, `src/doctor.rs`, and tests under `tests/`.
- Target checkout research at `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full` commit `1a6f343ee7ce597019a4c573259c2a9838376874`: `computer-use-linux/src/server.rs`, `remote_desktop.rs`, `abs_pointer.rs`, `windowing/target.rs`, `diagnostics.rs`, `atspi_tree.rs`, and `screenshot.rs`.
- Live local probes: `xdotool`, `ydotool`, `wmctrl`, `xprop`, and `xmessage` are installed; `/dev/uinput` is read/write; `/tmp/.ydotool_socket` is connectable; Screenshot portal exposes `Screenshot`; RemoteDesktop portal strict introspection has no concrete methods/properties.
- External/source research: `gh repo view` metadata for `agent-sh/computer-use-linux`, `tak-uukti/linux-computer-use`, `BeckhamLabsLLC/linux-desktop-mcp`, `MONTBRAIN/vadgr-computer-use`, `joe223/sootie`, and `jordansissel/xdotool`; no code-copy source selected.

## Plan Summary

- Add standalone pointer JSON commands and MCP wrappers for `click`, `scroll`, and `drag` under the project-owned X11/EWMH surface.
- Targeted actions reuse the existing unique target resolution and focus verification pattern, then add bounds checks before active/global-context pointer injection.
- Coordinates are global/root X11 pixels for this stage; frame/client ambiguity is recorded and screenshot/crop-specific coordinate model work remains in backlog/09.
- Standalone pointer backend uses `xdotool` only after safety gates; source-overlay use of `abs_pointer`/portal/ydotool remains a later integration decision.
- Explicit `--global` mode is allowed only when marked `global_unverified` and never presented as window-isolated targeting.

## Question Loop

### Q1: Should this stage implement backlog file `06` source overlay before pointer actions?

- Recommended answer: no; proceed with `07b` pointer actions now.
- Rationale: `backlog/00-research-reuse-map.md` and prior archived changes explicitly defer source overlay until core standalone behavior such as targeted keyboard, pointer, AT-SPI, screenshot/coordinates, and `get_app_state` stabilize. The target repo is moving and already has stock pointer tools, so a long-lived overlay is higher risk than standalone validation now.
- Resolution: answered from backlog and target/source research. No user question required. Source overlay writes are out of scope.

### Q2: Does pointer work conflict with backlog/09 because screenshot/coordinate model is later?

- Recommended answer: no, if this change limits itself to current global/root X11 pixels from `WindowInfo.bounds` and records frame/client uncertainty.
- Rationale: `x11-window-listing` already guarantees signed `bounds.x/y` and positive dimensions from `wmctrl -lpGx`; this is enough to validate whether a requested global point lies within a window's reported bounds. Backlog/09 remains responsible for screenshot/crop/client-vs-frame precision.
- Resolution: specs require global/root X11 coordinates and bounds validation only; no screenshot crop or client-area promise is added.

### Q3: Should the standalone pointer backend prefer target-style `abs_pointer` over `xdotool`?

- Recommended answer: no for this standalone plugin stage; use `xdotool` as the isolated X11 backend, while recording that source overlay should evaluate `abs_pointer` first later.
- Rationale: the standalone crate already uses command-based fake `PATH` tests for X11 tools and can validate `xdotool` without mutating target code. The target repo's stock server prefers `abs_pointer`, but importing that implementation now would expand scope and couple the standalone plugin to target internals.
- Resolution: design will use active-context `xdotool` commands for standalone pointer actions. Source-overlay backend ordering remains deferred.

### Q4: Is no-target pointer input allowed?

- Recommended answer: only with an explicit `--global` / `global: true` marker and degraded reporting.
- Rationale: pointer injectors are global desktop injectors; unlike safe targeted commands, global pointer movement/clicks can be a useful development primitive but must not masquerade as window-isolated targeting. Backlog/07b explicitly asks for global/unverified reporting.
- Resolution: specs require `MissingTarget` unless `--global` is present, and global reports must set `targeted=false` and `verification_mode=global_unverified`.

### Q5: What safety limits should design use for scroll and drag?

- Recommended answer: finite conservative defaults: clamp click count and scroll amount to small bounded ranges, and refuse very large drags in this stage.
- Rationale: backlog/07b requires no infinite movement and safety limits. Existing target `server.rs` clamps click counts and uses finite command sequences. Standalone implementation can choose fixed limits now and expose them in diagnostics without needing a durable architecture decision.
- Resolution: design must specify finite constants and tests must prove clamp/refusal behavior. No user question required.

### Q6: Is a durable ADR required for pointer actions?

- Recommended answer: no durable ADR for this change.
- Rationale: the hard architectural decisions already exist: backend id is `x11-ewmh`, standalone plugin precedes source overlay, and focus verification is the safety boundary for global input injectors. This change applies those decisions to pointer actions and remains reversible.
- Resolution: record no new durable ADR; the required per-change `adr.md` will document the review.

## Resolved Terms

- `pointer action`: a click, scroll, or drag operation expressed in global/root X11 pixel coordinates for this standalone stage.
- `targeted pointer action`: a pointer action whose coordinates are validated against one resolved current X11/EWMH window and whose target is focus-verified before injection.
- `global_unverified`: report value for explicit no-target pointer actions that are intentionally not window-isolated.

No `CONTEXT.md` update was required: these terms are capability/report-surface terms for this change and do not alter project-wide glossary language beyond existing `Active window`, `Focus verification`, `Standalone plugin`, and `x11-ewmh` concepts.

## Document Updates Applied

- None after grilling. The proposal and specs already encode the resolved scope: standalone only, global/root X11 coordinates, xdotool backend, explicit global mode, and source overlay deferred.

## Document Updates Required Before Next Gate

None.

## ADR Candidates

- None. No hard-to-reverse or surprising architecture decision is introduced by applying the existing verify-before-inject invariant to standalone pointer actions.

## Open Questions

None
