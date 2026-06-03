## Context Read

- Change artifacts: `proposal.md`, delta spec for `codex-source-overlay-extension`, `grill.md`, and `design.md`.
- Project context: `CONSTITUTION.md`, `CONTEXT.md`, `ARCHITECTURE.md`, `adr/README.md`, and ADR 0010 for provider takeover boundaries.
- Current implementation: `scripts/install-x11-provider-takeover.sh`, `scripts/uninstall-codex-plugin.sh`, `scripts/codex-source-overlay.py`, provider takeover overlay patcher files, and existing source overlay/plugin installer tests.
- Prior manual rollback evidence from this session: source overlay state remained applied and live `computer-use-settings-aHZZtKP_.js` retained takeover strings after standalone plugin uninstall.

## Design Summary

- The design introduces a symmetric operator-facing `uninstall-x11-provider-takeover.sh` wrapper for the existing install wrapper.
- `codex-source-overlay.py` remains the lower-level owner of provider takeover source/live backup manifests and restore logic.
- Install becomes transaction-like for source/live files: backup before write, record installed metadata after write, restore current-transaction writes on failure.
- Uninstall restores from manifest backups, validates marker/checksum drift before overwriting, and no-ops only when all owned surfaces are already absent.
- Live asset cleanup is manifest-backed, not default regex deletion, because root-owned/generated live bundles can drift independently from target source files.

## Question Loop

No user-facing question required.

### Reviewed Question: Should rollback ever reverse-patch live assets without a backup?

Recommended answer: No for the normal uninstaller. It may report the exact manual recovery command or recommendation, but the automated rollback must require a manifest backup for marked live assets.

Rationale: The manual reverse patch in this session was safe only because we inspected the concrete minified bundle and exact marker strings. Automating blind deletion risks corrupting a newer upstream bundle or removing unrelated UI content. The spec already requires safe blockers for missing backups.

Resolution: Keep design as manifest-backed restore only; docs can mention clean rebuild/reinstall or manual recovery for legacy residue.

## Design Findings

- **Good:** The design closes the actual UX gap: uninstalling only standalone plugin state is not enough after provider takeover install.
- **Good:** The design preserves ADR 0010: the X11 provider remains standalone; compatibility stays in localized settings/provider patching; rollback restores bundled mode rather than rewriting bundled plugin identity.
- **Good:** The design separates lower-level deterministic source/live restore from wrapper-level plugin cleanup and aggregate reporting.
- **Risk:** Wrapper failure after plugin install but before overlay success could leave plugin state installed. The implementation tasks must explicitly require wrapper cleanup or an aggregate failure report that identifies plugin residue and attempts plugin uninstall when later phases fail.
- **Risk:** Existing older provider manifests have weaker metadata. Implementation should support old source backups but fail safe for unbacked live assets.
- **Risk:** Removing `.codex-computer-use-x11-overlay` too early would delete backup evidence before live restore succeeds. Implementation must remove metadata only after all restore/verification succeeds.
- **Verification feasibility:** Fake target and fake live asset dirs are sufficient for automated tests; no root `/opt` mutation is required for CI.

## Document Updates Applied

None. The existing proposal/spec/design already include the required wrapper, transaction, drift, missing-manifest, and no-blind-live-delete constraints.

## Document Updates Required Before Next Gate

None.

## ADR Candidates

No new durable ADR is required. The change hardens a local lifecycle mechanism already governed by ADR 0010 rather than making a new durable architecture decision. The manifest-backed rollback policy is important but specific to this provider takeover implementation and is captured in the OpenSpec spec/design.

## Open Questions

None.
