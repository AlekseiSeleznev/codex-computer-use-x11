## Context Read

- Change artifacts: `proposal.md`, `specs/standalone-codex-mcp-plugin/spec.md`, `grill.md`, `design.md`.
- Root project context: `CONSTITUTION.md`, `CONTEXT.md`, `ARCHITECTURE.md`, `adr/README.md`.
- Implementation context: `src/cli.rs`, `src/doctor.rs`, `src/list_windows.rs`, `src/focus.rs`, `src/lib.rs`, `src/main.rs`, existing CLI tests, `README.md`, `Cargo.toml`, `Makefile`.
- Target/local plugin context already inspected during proposal/grill/design: target bundled plugin manifests, launcher sync code, bundled marketplace writing code, installed cache/marketplace shape, and non-secret user config plugin sections.
- External protocol context: official MCP tool error guidance says protocol errors are for malformed/unknown requests, while tool/business-logic errors use tool results with `isError: true`.

## Design Summary

- `codex-computer-use-x11 mcp` will be a minimal internal stdio JSON-RPC server with static `x11_*` tool registry and thin wrappers over existing JSON report builders.
- Installer scripts will create an owned `codex-computer-use-x11` cache/marketplace/config namespace under `$CODEX_HOME`, never touching `/opt`, `openai-bundled`, bundled `computer-use`, or the target checkout.
- Installer/uninstaller behavior is public-script tested through temp `CODEX_HOME`, dry-run, idempotent install, and owned-only uninstall.
- Direct MCP stdio smoke remains the primary feedback path when host Codex plugin hot-loading cannot be proven in the current process.

## Question Loop

### Q1: Should `x11_focus_window` return protocol errors for invalid or unverified focus requests?

- **Recommended answer:** Use JSON-RPC protocol errors only for malformed requests or unknown tools; use MCP tool results with `isError: true` for missing/invalid `window_id`, `WindowNotFound`, or `FocusNotVerified` execution results, while preserving structured JSON content whenever the underlying focus report exists.
- **Rationale:** This matches MCP error guidance and lets the model recover from tool/business-logic errors without confusing them with server/protocol failures.
- **Resolution:** Existing design already uses tool results for argument/business failures and protocol errors for method/request failures. No user question required.

### Q2: Does the design risk printing secrets from `~/.codex/config.toml`?

- **Recommended answer:** No, if scripts never print full config and tests verify only owned sections. Implementation should read/write the config file but log only owned section names and paths.
- **Rationale:** The user config can contain unrelated MCP server settings or environment names; full config output is unnecessary and violates project secret hygiene.
- **Resolution:** Design already says do not print full config; tasks/test-plan must include a preservation/no-full-config-output check. No user question required.

### Q3: Should dry-run validate or build the binary?

- **Recommended answer:** No. Dry-run should resolve intended paths and print planned actions without building or writing.
- **Rationale:** A strict dry-run with no writes is easier to test and satisfies the acceptance check. Non-dry install can build or copy an env-provided binary.
- **Resolution:** Design already states `--dry-run` does not build or write. No user question required.

### Q4: Is a symlinked `latest` acceptable for the standalone plugin cache?

- **Recommended answer:** Yes, because the current bundled cache uses `latest` symlinks and this installer runs on Linux. Uninstall must handle absent or broken symlinks safely.
- **Rationale:** Matching observed local cache shape improves compatibility; tests can assert symlink resolution and rollback safety.
- **Resolution:** Keep `latest` symlink behavior. No user question required.

### Q5: Does the persistent marketplace root conflict with the launcher's bundled marketplace cleanup?

- **Recommended answer:** No. Use `$CODEX_HOME/plugins/marketplaces/codex-computer-use-x11`, not `$CODEX_HOME/.tmp/bundled-marketplaces/openai-bundled`.
- **Rationale:** The launcher cleanup explicitly targets bundled `openai-bundled` temp roots. The owned persistent path is separated and uninstallable.
- **Resolution:** Design already uses the persistent owned path. No user question required.

### Q6: Should this installer be a durable project architecture decision?

- **Recommended answer:** No durable ADR is required now. The installer format is an implementation detail of the accepted standalone-plugin delivery path and remains reversible.
- **Rationale:** It is not hard to reverse; future plugin format changes can update this capability/spec without superseding project architecture.
- **Resolution:** Record no durable ADR in `adr.md` unless later implementation discovers a hard-to-reverse constraint. No user question required.

## Design Findings

- **Protocol boundary is acceptable:** Minimal MCP is sufficient for the required initialize/tools path; use JSON-RPC errors only for protocol/request failures and `isError` tool results for tool/business failures.
- **Config safety needs explicit tests:** The test plan should prove unrelated config sections survive install/uninstall and scripts avoid dumping full config.
- **Dry-run definition is strict:** Dry-run must not build the release binary or create directories, not merely avoid config changes.
- **Hot-load uncertainty is handled:** Direct MCP smoke plus exact refresh/restart guidance satisfies the spec when current Codex cannot dynamically discover the new plugin.
- **No target-checkout writes:** Design remains compatible with constitution/backlog; target repo is read-only research only.

## Document Updates Applied

None. The existing spec/design already cover the resolved findings.

## Document Updates Required Before Next Gate

None.

## ADR Candidates

No durable ADR candidate. The namespace/cache/config approach is reversible, local, and a concrete implementation of the already accepted standalone plugin path rather than a project-wide architecture change.

## Open Questions

None.
