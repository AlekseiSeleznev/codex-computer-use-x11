## ADR Review

This change composes existing standalone X11/EWMH capabilities into an app-state read surface and adds a namespaced MCP wrapper. It does not change the durable coordinate model, source-overlay posture, target checkout ownership, lifecycle architecture, or project context model.

## Existing In-Force ADRs

- `adr/0008-adopt-x11-root-coordinate-model.md` — Accepted and in force. This change follows it by keeping window bounds and any target/window context in X11 root/global coordinates and by treating source-overlay screenshot capture as future target-provider reuse.

`ARCHITECTURE.md` and `adr/README.md` also reference earlier in-force ADRs that are not present as top-level files in the current checkout. Their summarized rules remain represented in `ARCHITECTURE.md` and `CONSTITUTION.md`; no decision in this change contradicts those summarized rules.

## Constitution / Architecture Rules Considered

- Rust 2021/Cargo remain the implementation stack; adding the `base64` crate is a normal Cargo dependency for target-compatible screenshot data URL encoding.
- Root `Makefile` verification (`make fmt`, `make check`, `make test`) remains mandatory.
- OpenSpec lifecycle and checkpoint rules remain in force; all planning artifacts are checkpointed before apply.
- No secrets are needed; `.secrets.local.env` is not read.
- The target checkout path named by `CODEX_DESKTOP_LINUX_FULL_PATH` is inspected read-only and must remain unmodified.
- Standalone plugin tools remain project-owned `x11_*` names; future source overlay should improve stock target `get_app_state` instead of adding a competing stock tool.
- ADR 0008 root-coordinate and screenshot-provider boundary remains in force.

## Decisions Evaluated

- **Standalone app-state composition vs target checkout patch:** Chose standalone composition and documentation only. A target patch would be broader, higher drift risk, and contrary to the current read-only target boundary for this stage.
- **Screenshot data URL vs metadata only:** Chose target-compatible screenshot data URL by default with opt-out. This aligns with target `GetAppStateOutput`; response size is mitigated by `--no-screenshot` / `include_screenshot=false`.
- **Layer-degraded response vs whole-report failure:** Chose layer-degraded response. This follows target `window_error` / `screenshot_error` / `accessibility_error` separation and preserves useful data when one layer fails.
- **Durable ADR need:** Evaluated whether screenshot data URL and layer-degraded app-state need a new durable ADR. Rejected because these are scoped implementation/application of existing target compatibility and error-boundary patterns, not a hard-to-reverse architecture change.

## New Durable ADRs Created

- None.

## Superseded ADRs

- None.

## Architecture Snapshot Updates

- None. `ARCHITECTURE.md` already covers the lifecycle, source-overlay boundary, optional Claude review, and ADR 0008 coordinate/screenshot provider guidance. This change does not alter the current architecture snapshot.

## No ADR Needed

- No new durable ADR is needed because this change composes existing standalone capabilities, follows the target repo's already-established `get_app_state` response concepts, and preserves existing ADR 0008/source-overlay boundaries. The decisions are implementation-level and reversible within the standalone crate.
