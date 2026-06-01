## Context Read

- `openspec/changes/fix-plugin-manifest-hygiene/proposal.md`
- `openspec/changes/fix-plugin-manifest-hygiene/specs/project-bootstrap/spec.md`
- `openspec/changes/fix-plugin-manifest-hygiene/specs/standalone-codex-mcp-plugin/spec.md`
- `CONSTITUTION.md`
- `CONTEXT.md`
- `ARCHITECTURE.md`
- `adr/README.md`
- `adr/0001-adopt-codex-native-intent-driven-openspec-overlay.md`
- `adr/0003-formalize-project-context-entrypoints.md`
- `adr/0005-adopt-matt-grill-and-tdd-gates.md`
- `adr/0006-adopt-claude-artifact-review.md`
- `adr/0007-adopt-automatic-checkpoints-and-claude-session-controls.md`
- `adr/0008-adopt-x11-root-coordinate-model.md`
- `adr/0009-adopt-final-cinnamon-x11-v1-dod-baseline.md`
- `.gitignore`
- `README.md` standalone plugin section
- `scripts/install-codex-plugin.sh`
- `src/mcp.rs`
- `tests/plugin_installer.rs`
- `tests/mcp_server.rs`

## Plan Summary

- The change is intentionally scoped to safe review findings 1.1-1.3: tracked backup cleanup, manifest homepage correction, and manifest description/default-prompt drift.
- The current MCP server exposes fourteen `x11_*` tools in `src/mcp.rs`, and README already lists those fourteen names.
- The generated plugin manifest currently points at `AlekseiSelin/codex-computer-use-x11`, while the configured Git remote is `AlekseiSeleznev/codex-computer-use-x11`.
- The repository currently ignores local secrets/session state but does not ignore timestamped `*.bak.*` artifacts.
- No external systems or local secrets are needed; verification can stay local.

## Question Loop

1. **Question:** Should the scope include review findings 2.1 and 2.2 about `get-app-state` screenshot semantics and `xdotool --` payload hardening?
   - **Recommended answer:** No; keep this change limited to safe isolated repository hygiene and generated plugin metadata.
   - **Rationale:** Findings 2.1 and 2.2 affect behavior/design and need separate decisions/tests. The user-provided review text explicitly separated them from the safe 1.1-1.3 changes.
   - **Resolution:** Repository context and the proposal resolve this as out of scope for this change.

2. **Question:** Should the manifest long description enumerate every `x11_*` tool by name or describe the complete set by tool groups?
   - **Recommended answer:** Use concise tool groups plus representative prompts, while tests assert the presence of important current tool names and no stale six-tool wording.
   - **Rationale:** Grouped copy is easier to maintain and still fixes the drift that implied only six tools exist. README and `tools/list` remain the authoritative full enumerations.
   - **Resolution:** Specs require tool groups and representative prompts rather than a long exhaustive marketing sentence.

3. **Question:** Is a glossary update needed for terms such as backup artifact or plugin metadata?
   - **Recommended answer:** No.
   - **Rationale:** These are ordinary repository maintenance terms, not project-domain vocabulary requiring `CONTEXT.md` entries.
   - **Resolution:** No `CONTEXT.md` update.

## Resolved Terms

- No new project glossary terms were introduced.
- Existing terms used: standalone plugin, x11-ewmh, target window, app state.

## Document Updates Applied

- Proposal scopes the change to tracked backup cleanup, manifest homepage correction, and generated plugin manifest metadata drift.
- Delta specs add repository backup artifact hygiene and generated plugin manifest metadata accuracy requirements.

## Document Updates Required Before Next Gate

None.

## ADR Candidates

None. The change does not introduce a hard-to-reverse architecture decision; it preserves existing standalone plugin and OpenSpec lifecycle architecture.

## Open Questions

None.
