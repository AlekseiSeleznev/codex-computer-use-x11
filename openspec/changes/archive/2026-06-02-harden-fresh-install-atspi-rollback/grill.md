## Context Read

- `CONSTITUTION.md` — Rust/Cargo and Makefile verification constraints, local target path guidance, secret handling, no installed OpenSpec package patching, and safe checkpoint discipline.
- `CONTEXT.md` — existing project terminology for standalone plugin, source overlay, overlay drift, AT-SPI window correlation, accessibility tree, app state, e2e harness, controlled fixture, capability matrix evidence, and release checklist.
- `ARCHITECTURE.md` — current standalone/source-overlay architecture, Cinnamon/X11 v1 scope, mandatory lifecycle gates, and ADR 0010 takeover rule.
- `adr/README.md` and `adr/0010-adopt-x11-provider-takeover-shim.md` — in-force ADR set and provider takeover constraints.
- `openspec/changes/harden-fresh-install-atspi-rollback/proposal.md` and all delta specs under `openspec/changes/harden-fresh-install-atspi-rollback/specs/`.
- Relevant code/scripts: `src/doctor.rs`, `scripts/install-codex-plugin.sh`, `scripts/uninstall-codex-plugin.sh`, `scripts/install-x11-provider-takeover.sh`, `scripts/uninstall-x11-provider-takeover.sh`, `scripts/codex-source-overlay.py`, `scripts/e2e/codex-x11-e2e.py`, and existing source-overlay/provider takeover tests.

## Plan Summary

- The change fixes a concrete false-negative: `src/doctor.rs:gather_system_facts()` already checks AT-SPI bus availability and the report shape already exposes `match_outcome`, `candidate_count`, and `controlled_fixture_pass`, but tree availability is currently hardcoded false.
- Fresh install must become a complete Cinnamon/X11 activation flow for the selected mode: owned standalone plugin, optional X11 provider takeover, source overlay, optional live asset patch, and safe AT-SPI setup.
- Rollback must be the safety boundary: every mutation needs before/after metadata and changed-vs-already-present classification, and uninstall must restore only manifest-owned changes.
- Provider takeover remains constrained by ADR 0010: localize compatibility aliases, preserve `codex-computer-use-x11` identity and `x11_*` tools, and retain bundled fallback/rollback.
- Verification must cover fixture-backed doctor behavior, installer/uninstaller state restoration, fake no-GUI fresh install → doctor ok → uninstall, plus a live-safe checklist when the local Cinnamon/X11 target is available.

## Question Loop

No user questions were required at the pre-design grill gate. Repository context and the user request resolved the material boundaries:

1. **Should fresh install enable Orca/screen-reader state to make AT-SPI work?**
   - **Recommended answer**: No.
   - **Rationale**: The request explicitly forbids enabling screen reader/Orca without a separate decision, and existing AT-SPI fixture evidence uses GTK bridge/accessibility environment rather than Orca autostart.
   - **Resolution**: Specs require toolkit accessibility and bridge environment remediation only; no Orca enablement.

2. **Should provider takeover globally masquerade the standalone plugin as bundled `computer-use`?**
   - **Recommended answer**: No.
   - **Rationale**: ADR 0010 rejects global masquerade and requires localized settings/provider shim plus rollback to bundled mode.
   - **Resolution**: Specs preserve standalone plugin identity and localized compatibility aliases.

3. **Should rollback overwrite drifted live assets blindly?**
   - **Recommended answer**: No.
   - **Rationale**: Existing glossary defines overlay drift as a safety status, and the user requested explicit drift/blocker reporting rather than blind rollback.
   - **Resolution**: Specs require comparing current state to manifest after-state and blocking drifted restoration.

4. **Should doctor duplicate AT-SPI collector logic or consume a shared probe result?**
   - **Recommended answer**: Consume a shared lightweight probe result.
   - **Rationale**: `x11_accessibility_tree`/`x11_get_app_state` already have working collector/correlation behavior, while doctor has the report fields but not the collector facts. A shared probe avoids divergent readiness logic.
   - **Resolution**: Specs add a reusable AT-SPI collector probe consumed by doctor.

## Resolved Terms

- Added `Rollback-first install` to `CONTEXT.md` as the install contract where rollback metadata exists before mutation and drift blocks blind restoration.
- Added `Backup manifest` to `CONTEXT.md` as the non-secret durable record for before-state, after-state, changed-vs-already-present classification, file metadata, checksums, and partial-install rollback progress.

## Document Updates Applied

- Created proposal and six delta specs for `doctor-cli`, `x11-atspi-window-correlation`, `standalone-codex-mcp-plugin`, `codex-source-overlay-extension`, `codex-computer-use-provider-takeover`, and `codex-x11-e2e-test-harness`.
- Updated `CONTEXT.md` with the two resolved glossary terms above.

## Document Updates Required Before Next Gate

None.

## ADR Candidates

- A change-local ADR review is required by the lifecycle because the change spans installer safety, provider takeover, live/root-owned assets, and rollback semantics.
- Durable ADR candidate: **rollback-first install/manifest contract**. This may merit a new durable ADR if design confirms the manifest model is a hard-to-reverse architecture boundary across standalone plugin, source overlay, live asset, gsettings, and activation environment state. ADR 0010 remains in force and should be considered rather than superseded unless design chooses to change takeover boundaries.

## Open Questions

None.
