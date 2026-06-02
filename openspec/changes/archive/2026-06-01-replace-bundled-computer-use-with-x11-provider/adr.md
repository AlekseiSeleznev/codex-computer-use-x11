## ADR Review

## Existing In-Force ADRs

- `adr/0001-adopt-codex-native-intent-driven-openspec-overlay.md` — accepted; remains in force. This change uses OpenSpec artifacts as the lifecycle source of truth.
- `adr/0003-formalize-project-context-entrypoints.md` — accepted; remains in force. `CONSTITUTION.md`, `ARCHITECTURE.md`, and local-secret boundaries were read and respected.
- `adr/0005-adopt-matt-grill-and-tdd-gates.md` — accepted; remains in force. `grill.md`, `design-review.md`, and `test-plan.md` are required before implementation.
- `adr/0006-adopt-claude-artifact-review.md` — accepted; remains in force. Claude review is disabled for this session by user request and session state.
- `adr/0007-adopt-automatic-checkpoints-and-claude-session-controls.md` — accepted; remains in force. Safe local lifecycle checkpoints may be automatic; push/archive remain explicit.
- `adr/0008-adopt-x11-root-coordinate-model.md` — accepted; remains in force. Not directly changed because this work is settings/provider selection, not coordinate semantics.
- `adr/0009-adopt-final-cinnamon-x11-v1-dod-baseline.md` — accepted; remains in force. The standalone `codex-computer-use-x11` plugin identity and `x11_*` namespace remain preserved.
- `adr/0010-adopt-x11-provider-takeover-shim.md` — accepted by this change. It extends ADR 0009's identity-separation rule to the Computer Use settings provider layer.

## Constitution / Architecture Rules Considered

- `CONSTITUTION.md` allows target checkout work through `CODEX_DESKTOP_LINUX_FULL_PATH` or `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full` when OpenSpec tasks explicitly target source-overlay compatibility.
- Secret-handling rules apply; this change needs no external credentials and diagnostics must not write secret values.
- Verification must include OpenSpec validation, target patcher/installer checks, rollback evidence, and git-status checks for both repositories.
- `ARCHITECTURE.md` requires root ADR history to remain append-only and the architecture snapshot to be updated when a durable decision changes current architecture.
- Target repo rules favor fail-soft ASAR/webview patches and patch reports, while generated/root-owned live assets require backup and restart guidance.

## Decisions Evaluated

- **Localized provider takeover shim vs global plugin-id masquerade:** choose localized shim. It solves the active-provider UI need while keeping `codex-computer-use-x11@codex-computer-use-x11`, `x11_*` tools, and bundled `openai-bundled/computer-use` ownership intact.
- **Baseline diagnostics before takeover vs static patch markers only:** choose diagnostics. The prior asset marker did not prove runtime row visibility, so row decisions must be observable from payload and gate facts.
- **Explicit takeover configuration vs implicit replacement:** choose explicit `--provider x11 --mode takeover` so rollback and status can reason about bundled mode versus takeover mode.
- **Source overlay manager extension vs one-off live patch script:** choose the existing `scripts/codex-source-overlay.py` path with report/backup/rollback extensions.
- **Durable ADR vs per-change ADR only:** create a durable ADR because the takeover/shim decision is hard to reverse, surprising without context, and a real trade-off against global masquerade.

## New Durable ADRs Created

- `adr/0010-adopt-x11-provider-takeover-shim.md` — Accepted; captures the decision to use a localized X11 provider takeover shim for Computer Use settings instead of globally masquerading the standalone plugin as bundled `computer-use`.

## Superseded ADRs

- None. ADR 0010 extends ADR 0009 but does not supersede it.

## Architecture Snapshot Updates

- `ARCHITECTURE.md` updated to list ADR 0010 as in force and summarize the settings provider takeover rule.
- `adr/README.md` updated to include ADR 0010 in the current in-force durable ADR list.

## No ADR Needed

- N/A. A durable ADR was created.
