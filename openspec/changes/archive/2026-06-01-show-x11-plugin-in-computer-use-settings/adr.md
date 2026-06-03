## ADR Review

## Existing In-Force ADRs

- `adr/0001-adopt-codex-native-intent-driven-openspec-overlay.md` — accepted; remains in force. This change uses OpenSpec as source of truth and keeps implementation decisions in tracked artifacts.
- `adr/0003-formalize-project-context-entrypoints.md` — accepted; remains in force. Root context and local-secret boundaries were considered.
- `adr/0005-adopt-matt-grill-and-tdd-gates.md` — accepted; remains in force. `grill.md`, `design-review.md`, and vertical TDD evidence are required.
- `adr/0006-adopt-claude-artifact-review.md` — accepted; remains in force. Claude review is disabled for this session by user request and session state.
- `adr/0007-adopt-automatic-checkpoints-and-claude-session-controls.md` — accepted; remains in force. Safe checkpoints may be automatic; archive/push remain explicit.
- `adr/0008-adopt-x11-root-coordinate-model.md` — accepted; remains in force. Not directly affected because this is UI/plugin-row integration, not coordinate behavior.
- `adr/0009-adopt-final-cinnamon-x11-v1-dod-baseline.md` — accepted; remains in force. The standalone `x11_*` plugin namespace and separation from stock Computer Use tools are preserved.

## Constitution / Architecture Rules Considered

- `CONSTITUTION.md` allows local integration target work through `CODEX_DESKTOP_LINUX_FULL_PATH` or the documented default target path when an OpenSpec task explicitly targets overlay/source compatibility.
- Secret handling rules apply; no `.secrets.local.env` values or external credentials are needed.
- Verification must include OpenSpec validation and relevant target checks; Rust `make fmt/check/test` is not expected unless standalone Rust code changes.
- `ARCHITECTURE.md` separates Codex/OpenSpec lifecycle artifacts from project context and keeps durable decisions in top-level ADRs.
- `CONTEXT.md` terminology confirms standalone plugin/source overlay vocabulary; no durable glossary update is required.

## Decisions Evaluated

- **Side-by-side settings row vs replacing bundled row:** choose side-by-side `X11 Computer Use` row because replacing `computer-use` would contradict standalone ownership and risk bundled update conflicts.
- **Patch target webview asset vs mutate plugin manifest:** choose target webview patch because manifest metadata already installs correctly; the missing behavior is hardcoded UI lookup.
- **Enable under Computer Use UI opt-in vs always-on:** choose existing `enableComputerUseUi` gate because the row belongs to the visible Computer Use settings UI.
- **Create durable ADR vs per-change ADR review only:** no durable ADR because choices follow existing ADR 0009/source-overlay rules and are not a new hard-to-reverse architecture direction.

## New Durable ADRs Created

- None.

## Superseded ADRs

- None.

## Architecture Snapshot Updates

- None. `ARCHITECTURE.md` does not need an update because the current architecture and ADR 0009 already cover standalone/source-overlay separation.

## No ADR Needed

- No new durable ADR is needed. This change implements existing architecture in Codex Desktop Linux UI glue; it does not introduce a new backend boundary, safety model, coordinate model, lifecycle rule, or durable policy.
