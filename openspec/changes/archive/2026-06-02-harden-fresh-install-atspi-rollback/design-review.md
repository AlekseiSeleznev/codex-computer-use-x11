## Context Read

- Change artifacts: `proposal.md`, all six delta specs, `grill.md`, and `design.md` for `harden-fresh-install-atspi-rollback`.
- Project context: `CONSTITUTION.md`, `CONTEXT.md`, `ARCHITECTURE.md`, `adr/README.md`, and `adr/0010-adopt-x11-provider-takeover-shim.md`.
- Relevant implementation/test context: `src/doctor.rs`, `src/accessibility.rs`, `src/app_state.rs`, `scripts/install-codex-plugin.sh`, `scripts/uninstall-codex-plugin.sh`, `scripts/install-x11-provider-takeover.sh`, `scripts/uninstall-x11-provider-takeover.sh`, `scripts/codex-source-overlay.py`, `tests/doctor_cli.rs`, `tests/accessibility_tree_cli.rs`, `tests/plugin_installer.rs`, `tests/source_overlay_scripts.rs`, and `tests/e2e_harness_scripts.rs`.

## Design Summary

- Doctor will replace hardcoded AT-SPI tree-unavailable facts with a bounded shared collector probe that returns tree availability, candidate count, match outcome, and controlled fixture pass.
- Installation is composed by surface: standalone plugin, accessibility setup, provider takeover/source overlay, and optional live assets stay independently testable but can be orchestrated by fresh install.
- A backup manifest is the rollback authority across plugin paths, gsettings, activation environment, source files, and live assets.
- Rollback restores only completed installer-owned changes, refuses blind overwrites on drift, and supports dry-run/report-json.
- ADR 0010 boundaries remain intact: localized takeover shim, standalone identity preserved, bundled mode restorable.

## Question Loop

No user questions were required during design review. Repository context resolved the material review points:

1. **Does manifest-backed rollback conflict with existing provider takeover code?**
   - **Recommended answer**: No; extend it compatibly.
   - **Rationale**: `scripts/codex-source-overlay.py` already records provider manifest and backup metadata, and existing tests expect keys such as `source_backups`, `live_asset_backups`, and installed sha/size metadata.
   - **Resolution**: Design keeps legacy fields while adding richer schema/version/ownership/mode/changed-state metadata.

2. **Can a fresh install safely modify user accessibility environment?**
   - **Recommended answer**: Yes, but only for explicit baseline variables and only with manifest-backed before-state.
   - **Rationale**: The request explicitly targets `toolkit-accessibility`, `NO_AT_BRIDGE`, `GTK_MODULES`, and `QT_ACCESSIBILITY`; no secret or broad environment mutation is needed.
   - **Resolution**: Design limits environment scope and requires drift/blocker reporting on post-install changes.

3. **Should takeover uninstall remove the standalone plugin by default?**
   - **Recommended answer**: No, unless the standalone install manifest/operation selected that surface.
   - **Rationale**: ADR 0010 separates provider takeover rollback from standalone plugin identity; bundled mode can be restored while the standalone plugin remains installed.
   - **Resolution**: Design makes standalone uninstall governed by standalone plugin manifest policy, not by takeover rollback alone.

## Design Findings

- **Positive finding — doctor report shape is ready**: `src/doctor.rs` already includes fields for match outcome, candidate count, and controlled fixture pass. The production change can be small if it focuses on gathering probe facts.
- **Safety finding — preserve provider manifest compatibility**: existing provider takeover tests already assert report/manifest metadata. Apply must add fields without removing legacy keys unless tests/docs are updated in the same slice.
- **Safety finding — environment values must stay scoped**: only the explicitly relevant non-secret activation environment keys should be recorded. Broad environment dumps would conflict with constitution secret handling.
- **Rollback finding — drift checks must compare after-state first**: source/live rollback should restore before-state only when current state matches installer after-state; otherwise report a blocker.
- **Verification finding — fake e2e must avoid sudo and GUI**: live asset and gsettings behavior need fake commands/paths in tests; live checklist is supplemental evidence, not CI pass criteria.

## Document Updates Applied

None. The design already contains the required constraints after review.

## Document Updates Required Before Next Gate

None.

## ADR Candidates

- **Durable ADR recommended**: `Adopt rollback-first manifest contract for install and uninstall`. It is hard to reverse because it defines installer/uninstaller safety across multiple surfaces, surprising enough to need durable rationale because it intentionally blocks drifted rollback instead of force-restoring, and it chooses a real trade-off between automatic cleanup and user/admin state preservation.
- **ADR 0010 remains in force** and does not need supersession for this change unless implementation later proposes global masquerade or bundled ownership changes, which the current design forbids.

## Open Questions

None.
