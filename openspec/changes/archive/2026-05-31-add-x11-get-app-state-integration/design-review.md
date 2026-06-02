## Context Read

- Change artifacts: `proposal.md`, all delta specs, `grill.md`, `design.md`.
- Project rules/context: `CONSTITUTION.md`, `CONTEXT.md`, `ARCHITECTURE.md`, `adr/README.md`, `adr/0008-adopt-x11-root-coordinate-model.md`.
- Code under review: `src/cli.rs`, `src/mcp.rs`, `src/doctor.rs`, `src/list_windows.rs`, `src/input.rs`, `src/accessibility.rs`, `src/coordinates.rs`, `tests/mcp_server.rs`, existing fake-command CLI tests.
- Target reference: `computer-use-linux/src/server.rs`, `screenshot.rs`, `atspi_tree.rs`, `diagnostics.rs`, and `windowing/target.rs` in the local target checkout, read-only.

## Design Summary

- The design adds `src/app_state.rs` as a composition layer over existing standalone reports.
- CLI and MCP surfaces remain standalone/project-owned: `get-app-state` and `x11_get_app_state`.
- App-state returns target-compatible concepts and treats target, screenshot, and AT-SPI failures as layer-degraded fields.
- Screenshot data is target-compatible (`data_url`) by default, with explicit opt-out to control response size.
- Doctor live probes are expanded for DBus/AT-SPI facts without modifying files or requiring secrets.

## Question Loop

### Q1: Is adding a `base64` dependency acceptable for this crate?

- **Recommended answer:** Yes, use the widely used Rust `base64` crate for app-state screenshot data URL encoding.
- **Rationale:** Target `screenshot.rs` already uses `base64`; implementing our own encoder would be riskier and less maintainable. The constitution allows Rust/Cargo dependencies when design evidence justifies them.
- **Resolution from repository context:** Adopt recommended answer. No user question needed.

### Q2: Should app-state use one window listing snapshot or call `accessibility_tree_report_from_system()` and accept its second listing snapshot?

- **Recommended answer:** For this change, reuse `accessibility_tree_report_from_system()` even though it takes a second listing snapshot; tests should cover stable fixture behavior, and future refactor can expose a listing-injected accessibility report if race evidence appears.
- **Rationale:** The existing accessibility report builder owns correlation diagnostics and collector behavior. Refactoring it now would broaden the change and risk breaking already-verified AT-SPI behavior.
- **Resolution from repository context:** Adopt recommended answer; note the possible race as a non-blocking trade-off.

### Q3: Should a missing target selector be an error?

- **Recommended answer:** No. Missing target selector means global app-state: screenshot and diagnostics can still be useful; `window_context` and `window_error` stay null.
- **Rationale:** The target stock `get_app_state` accepts calls without window target selectors. The spec already records this.
- **Resolution from repository context:** Adopt recommended answer.

## Design Findings

- **Dependency addition:** `base64` must be added to `Cargo.toml`/`Cargo.lock` and verified by `make check` / `make test`.
- **Screenshot cleanup:** The screenshot temp file must be removed on success and best-effort removed on provider/read/PNG-parse failure to avoid `/tmp` leaks.
- **Layer success semantics:** CLI exit should be 0 for a serializable app-state report even when `window_error`, `screenshot_error`, or `accessibility_error` is set. Unsupported usage and malformed CLI arguments remain non-zero.
- **MCP error semantics:** `x11_get_app_state` should return `isError=false` for layer-degraded reports and `isError=true` only for malformed MCP arguments or server-level failures.
- **Doctor probe safety:** DBus/AT-SPI probes are read-only and may run only when `busctl`/`gdbus` are available. They must not print or store session bus addresses or secret values.
- **Target checkout:** No implementation task may write to `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full`; verification must check target git status remains clean.

## Document Updates Applied

None. The current design already incorporates the review findings; cleanup and exit semantics are implementation/test-plan details.

## Document Updates Required Before Next Gate

None.

## ADR Candidates

No new durable ADR is needed. The decisions are scoped implementation tactics that follow existing architecture and ADR 0008 rather than changing the architecture snapshot.

## Open Questions

None.
