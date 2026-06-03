## ADR Review

## Existing In-Force ADRs

- `adr/0001-adopt-codex-native-intent-driven-openspec-overlay.md` — Accepted; remains in force. This change follows the OpenSpec overlay lifecycle rather than ad-hoc implementation.
- `adr/0003-formalize-project-context-entrypoints.md` — Accepted; remains in force. `CONSTITUTION.md`, `CONTEXT.md`, `ARCHITECTURE.md`, and ADR history were read before lifecycle and architecture-sensitive work.
- `adr/0005-adopt-matt-grill-and-tdd-gates.md` — Accepted; remains in force. `grill.md`, `design-review.md`, and `test-plan.md` gates are required before apply.
- `adr/0006-adopt-claude-artifact-review.md` — Accepted; remains in force. Claude review is optional and was disabled by explicit user instruction for this session.
- `adr/0007-adopt-automatic-checkpoints-and-claude-session-controls.md` — Accepted; remains in force. Safe lifecycle checkpoints are automatic in session git `auto` mode; push/merge/archive/PR/release still require explicit approval.
- `adr/0008-adopt-x11-root-coordinate-model.md` — Accepted; remains in force. This change does not alter coordinate or runtime input behavior.
- `adr/0009-adopt-final-cinnamon-x11-v1-dod-baseline.md` — Accepted; remains in force. The adapter-prep work preserves `x11-ewmh`, namespaced `x11_*` tools, standalone identity, and upstream separation.
- `adr/0010-adopt-x11-provider-takeover-shim.md` — Accepted; remains in force. The future adapter must not globally masquerade as bundled `computer-use` and must not rewrite bundled plugin ownership.
- `adr/0011-adopt-rollback-first-install-manifest.md` — Accepted; remains in force. Installer-owned mutations remain rollback-first; offline packaging and inert scaffold generation do not change installer rollback semantics.

## Constitution / Architecture Rules Considered

- Required technologies: Rust 2021/Cargo for the standalone crate, Bash helper scripts under `scripts/`, Markdown OpenSpec artifacts, root Makefile verification commands.
- Secret handling: no `.secrets.local.env`, tokens, private endpoints, or credentials are needed or read; tracked docs may include variable names only.
- Documentation sources: project OpenSpec/ADRs/docs first; upstream `codex-desktop-linux` checkout used read-only as integration-target evidence.
- Verification rules: Rust changes must pass `make fmt`, `make check`, `make test`; OpenSpec must validate; doctor JSON behavior must remain machine-readable; release package script must verify artifacts.
- Architecture snapshot: standalone `codex-computer-use-x11` remains the runtime plugin; source overlay/upstream integration remains optional staging evidence, not a core runtime rewrite.

## Decisions Evaluated

- **Source-of-truth boundary**: keep this repository as the owner of plugin runtime, release bundle metadata, and release artifact checksums. Future upstream adapter is thin and disabled by default.
- **Bundle generation approach**: use a shared helper for plugin bundle files so installer and packaging metadata do not drift, while keeping installer marketplace/config/rollback behavior in `scripts/install-codex-plugin.sh`.
- **Scaffold patch approach**: include a conservative read-aloud-style plugin gate patch because upstream pattern evidence and user requirements indicate marketplace staging may not be sufficient.
- **Upstream mutation boundary**: do not modify the local upstream checkout, publish a release, push, PR, merge, or archive in this change without separate explicit approval.
- **Durable ADR candidate rejected**: no new top-level ADR is needed because these decisions apply existing ADR 0009/0010/0011 boundaries rather than changing runtime architecture or superseding prior durable decisions.

## New Durable ADRs Created

- None.

## Superseded ADRs

- None.

## Architecture Snapshot Updates

- None. `ARCHITECTURE.md` remains accurate: standalone plugin identity, optional source-overlay/upstream integration boundary, X11/EWMH baseline, and rollback-first installer rules remain unchanged.

## No ADR Needed

- No durable ADR is needed because this change prepares packaging/docs/scaffold handoff artifacts without changing the current runtime architecture, supported baseline, provider takeover rule, rollback-first installer contract, or in-force upstream separation. The per-change ADR review is sufficient to record the adapter handoff decisions.
