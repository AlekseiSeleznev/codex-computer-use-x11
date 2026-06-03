## Context Read

- `openspec/changes/prepare-codex-desktop-linux-x11-ewmh-adapter/proposal.md`
- `openspec/changes/prepare-codex-desktop-linux-x11-ewmh-adapter/specs/**/*.md`
- `openspec/changes/prepare-codex-desktop-linux-x11-ewmh-adapter/grill.md`
- `openspec/changes/prepare-codex-desktop-linux-x11-ewmh-adapter/design.md`
- `CONSTITUTION.md`, `CONTEXT.md`, `ARCHITECTURE.md`, `adr/README.md`, ADRs 0008-0011
- Existing installer bundle-writing code in `scripts/install-codex-plugin.sh`
- Upstream `linux-features/read-aloud-mcp/stage.sh`, `patches.js`, `test.js`, and `scripts/lib/linux-features.js`

## Plan Summary

- The design keeps runtime plugin behavior unchanged and adds source-of-truth release/scaffold artifacts only.
- A shared bundle helper reduces installer/package drift while leaving installer marketplace/config/rollback behavior localized.
- The release package is built from explicit staged files and verified by checksum, extraction, manifest inspection, forbidden-path checks, and extracted doctor JSON.
- The scaffold is copyable and inert locally; upstream stage/patch logic is tested through fake install roots and representative main-bundle fixtures.

## Question Loop

### Question 1: Can scaffold tests be useful before the scaffold is copied into upstream?

- **Recommended answer**: Yes; make `test.js` runnable both after upstream copy and from this repository by locating the upstream checkout through `CODEX_DESKTOP_LINUX_REPO`, `CODEX_DESKTOP_LINUX_FULL_PATH`, or the documented local default.
- **Rationale**: The user asked for self-contained Node tests modeled after upstream `read-aloud-mcp`. Running them only after copy would delay feedback; running them from this repo must remain read-only against upstream.
- **Resolution**: Applied design update requiring the dual-location lookup and no upstream writes.

### Question 2: Does the shared bundle helper conflict with rollback-first install manifests?

- **Recommended answer**: No; only extract the plugin bundle file-writing portion. Keep installer-owned `CODEX_HOME`, marketplace symlink, config, accessibility, and rollback manifest logic in `scripts/install-codex-plugin.sh`.
- **Rationale**: ADR 0011 applies to installer-owned mutations, not to offline artifact assembly. Moving marketplace/config/manifest logic into packaging would risk coupling unrelated responsibilities.
- **Resolution**: Design already keeps marketplace/config/rollback localized to the installer.

### Question 3: Should the future upstream adapter stage into `openai-bundled/plugins` even though the feature is disabled by default?

- **Recommended answer**: Yes, only when the feature is explicitly enabled. This follows upstream `read-aloud-mcp` staging shape and makes the plugin available through the app's bundled marketplace without becoming a default repository plugin.
- **Rationale**: The maintainer requested `linux-features/` and disabled-by-default opt-in integration; upstream read-aloud-mcp demonstrates feature-gated staging into `openai-bundled` resources.
- **Resolution**: Specs/design retain disabled-by-default feature gate and stage only under `$INSTALL_DIR/resources/plugins/openai-bundled/plugins/codex-computer-use-x11` when enabled.

## Resolved Terms

No new glossary terms beyond the pre-design grill updates.

## Document Updates Applied

- Updated `design.md` to require scaffold `test.js` to locate upstream via env/default when run from this repo, while preserving relative behavior after upstream copy.

## Document Updates Required Before Next Gate

None.

## ADR Candidates

- No new durable top-level ADR is required. The design is an application of existing ADRs:
  - ADR 0009: standalone identity, `x11-ewmh` baseline, source-overlay/upstream separation.
  - ADR 0010: no global masquerade as bundled `computer-use`.
  - ADR 0011: rollback-first install remains localized to installer-owned mutations.
- The release adapter handoff decision should be recorded in the per-change `adr.md` because it is important to this change, but it does not supersede or broaden existing durable architecture decisions.

## Open Questions

None.
