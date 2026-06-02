## Context

The review found three low-risk drift issues after the project already passed build, clippy, fmt, and tests: two timestamped OpenSpec config backup files are tracked, the generated Codex plugin manifest uses a misspelled GitHub owner in `homepage`, and the manifest copy still describes only the first six MCP tools. The current architecture keeps the standalone plugin as a project-owned `x11_*` MCP surface, with OpenSpec as lifecycle source of truth and local checks as verification. No external systems or secrets are needed.

Relevant constraints:

- `CONSTITUTION.md` requires root Makefile checks for Rust changes and OpenSpec validation for changed artifacts.
- ADR 0007 permits automatic safe lifecycle checkpoints in session auto mode, but push/archive/destructive operations still require approval.
- ADR 0009 keeps standalone MCP tools namespaced as `x11_*` and distinct from stock Computer Use tools.
- `README.md` and `src/mcp.rs` already agree on the fourteen exposed standalone tool names.

No Mermaid diagram is needed because this change does not alter runtime boundaries, integration topology, process ownership, or data flow. It only changes repository hygiene and generated metadata text.

## Goals / Non-Goals

**Goals:**

- Remove tracked timestamped OpenSpec backup files and prevent future `*.bak.*` backups from appearing in `git status`.
- Correct `scripts/install-codex-plugin.sh` so generated plugin manifests point at `https://github.com/AlekseiSeleznev/codex-computer-use-x11`.
- Refresh generated manifest `longDescription` and `defaultPrompt` text so plugin metadata reflects the complete current `x11_*` tool surface.
- Add focused regression coverage for generated plugin manifest metadata.

**Non-Goals:**

- Do not change MCP tool definitions, CLI command behavior, or README tool enumeration.
- Do not address `get-app-state` screenshot crop/fullscreen semantics in this change.
- Do not harden `xdotool` payload `--` handling in this change.
- Do not install, uninstall, push, archive, or access external systems.

## Decisions

1. **Use `.gitignore` for broad timestamped backups.**
   - Add `*.bak.*` as a repository-level ignore rule.
   - Remove the already tracked `openspec/config.yaml.bak.*` files using Git index/file removal while keeping canonical `openspec/config.yaml` intact.
   - Alternative rejected: ignore only `openspec/*.bak.*`; broader `*.bak.*` is simpler and covers the same accidental editor/tool artifact class without affecting canonical source names.

2. **Keep manifest metadata generated in the installer.**
   - Update the Python manifest literal inside `scripts/install-codex-plugin.sh` rather than adding a separate template file.
   - Alternative rejected: introduce a generated manifest source file; this is unnecessary for a small metadata-only drift fix and would add another file to keep synchronized.

3. **Describe tool groups rather than exhaustively repeating all fourteen names in marketing copy.**
   - `longDescription` should mention the current tool groups: readiness diagnostics, window listing/focus, keyboard and pointer actions, AT-SPI accessibility tree, app state, and target-window context.
   - `defaultPrompt` should mention representative current tools, including inspection and action/context flows, while staying in the `x11_*` namespace.
   - Alternative rejected: a long exact-name sentence listing every tool, because README and `tools/list` already provide the authoritative full enumeration and a marketing sentence would be brittle.

4. **Use focused tests around generated manifest JSON.**
   - Extend `tests/plugin_installer.rs` where it already installs into a temporary Codex home and parses `.codex-plugin/plugin.json`.
   - Assert the corrected homepage, absence of the stale owner, and the presence of representative post-six tool names/tool groups in `longDescription` and `defaultPrompt`.
   - Use command-based verification (`git ls-files`, `git check-ignore`) for backup hygiene because it is repository metadata rather than Rust runtime behavior.

## Risks / Trade-offs

- Broad `*.bak.*` ignores may hide intentionally named backup fixtures in future work. This is acceptable because tracked fixtures can still be force-added deliberately, and timestamped backup artifacts should not be ordinary source files.
- Grouped manifest copy is less exhaustive than `tools/list`; tests should therefore check representative current capabilities instead of pretending the long description is canonical API documentation.
- Removing tracked backup files changes repository history going forward but does not affect runtime behavior.

## Migration Plan

1. Add `*.bak.*` to `.gitignore`.
2. Remove `openspec/config.yaml.bak.20260530150421` and `openspec/config.yaml.bak.20260530150551` from tracked files.
3. Update `scripts/install-codex-plugin.sh` manifest metadata.
4. Extend installer tests to validate generated manifest metadata.
5. Verify with OpenSpec validation, focused test/check commands, and the constitution-required `make fmt`, `make check`, and `make test`.

Rollback is a normal Git revert of the implementation commit; no deployed state or external system mutation is involved.

## Open Questions

None.
