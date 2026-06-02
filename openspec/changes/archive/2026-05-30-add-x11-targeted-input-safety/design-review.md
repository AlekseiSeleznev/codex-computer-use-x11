## Context Read

- `proposal.md`, `specs/x11-targeted-input-safety/spec.md`, `specs/standalone-codex-mcp-plugin/spec.md`, `grill.md`, and `design.md` for this change.
- Root `CONSTITUTION.md`, `CONTEXT.md`, `ARCHITECTURE.md`, `adr/README.md`, and in-force ADRs 0001, 0003, 0005, 0006, 0007.
- Existing standalone code: `src/cli.rs`, `src/focus.rs`, `src/list_windows.rs`, `src/mcp.rs` and current integration tests.
- Target checkout code: `computer-use-linux/src/server.rs::focus_target_for_input()`, `PressKeyParams`, `TypeTextParams`, `action_result_with_focus()`, `windowing/target.rs`, and diagnostics readiness names.
- Local `xdotool` manpage for XSendEvent/direct-window behavior and Unicode/layout warning.

## Design Summary

- The design adds standalone safe keyboard commands and MCP tools instead of applying a source overlay now.
- Target resolution is unique-or-fail for `window_id`, title substring, `wm_class`, and pid.
- Focus verification is the only safety boundary; any failure keeps `input_sent: false`.
- `xdotool type/key --clearmodifiers` runs only in active-context after exact focus verification and never uses `--window` as a safety claim.
- Tests must cover fake command behavior first, then live smoke/degraded evidence.

## Question Loop

### Q1: Does adding `type-text`/`press-key` conflict with the previous standalone MCP tool namespace?

- Recommended answer: no, if the existing standalone spec is explicitly modified.
- Rationale: the previous canonical spec required exactly four `x11_*` tools. This change includes a `MODIFIED` delta that updates deterministic order to six project-owned tools and still forbids unprefixed stock names.
- Resolution: answered from specs. No user question needed.

### Q2: Is there any hard-to-reverse architecture decision requiring a durable ADR?

- Recommended answer: no durable ADR for this change.
- Rationale: the hard architectural invariant already exists in `x11-active-window-focus`, backlog/00, and architecture context: verify focus before targeted input. This change applies the invariant to standalone keyboard tools and does not change backend identity, source-overlay strategy, or target repo architecture.
- Resolution: record no new durable ADR; per-change `adr.md` is still required.

### Q3: Are target selectors too broad for safe input?

- Recommended answer: acceptable with unique-or-fail semantics.
- Rationale: title substring can be ambiguous, but the design refuses ambiguous targets and returns candidates. `wm_class` and pid are exact matches; PID reliability remains whatever the listing reports, so diagnostics should include listing context.
- Resolution: keep selectors but require no activation/input on ambiguity or stale target.

### Q4: Does active-context xdotool still have a race after focus verification?

- Recommended answer: yes, but it is the best available standalone X11 safety boundary and must be reported honestly.
- Rationale: another window could steal focus after verification and before injection. The implementation should verify immediately before input and state that this is active-context safety, not OS-level per-window isolation.
- Resolution: no spec/design update required; risk is already documented in design.

### Q5: Should no-target global input be supported as development input?

- Recommended answer: no for this capability.
- Rationale: the target repo distinguishes `can_send_development_input` from targeted readiness, but this standalone change is specifically safe targeted input. Adding global mode now would obscure acceptance checks.
- Resolution: specs require `MissingTarget` and no input command.

## Design Findings

- **No material conflict with constitution or architecture.** Rust/Cargo, OpenSpec, secret handling, and verification rules are preserved.
- **No target checkout mutation.** Source overlay remains a later backlog stage.
- **Report shape needs stable automation fields.** Implementation must avoid only free-text success/failure; `success`, `input_sent`, `error_code`, and candidates are required for tests/MCP.
- **MCP schemas must require text/key and at least describe target selectors.** JSON Schema cannot easily enforce "one of the selectors" in the minimal hand-written MCP server, so the runtime validator must enforce it and return `MissingTarget`.
- **Unicode evidence cannot be faked as live text success.** Fake tests can prove argument preservation; live smoke must record actual behavior or a degraded limitation.

## Document Updates Applied

- None after design review; proposal/specs/design already contain the required constraints.

## Document Updates Required Before Next Gate

- None.

## ADR Candidates

- None. No new durable ADR needed for this standalone application of existing verify-before-inject architecture.

## Open Questions

None
