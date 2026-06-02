## ADR Context Read

- `ARCHITECTURE.md`
- `adr/README.md`
- `adr/0008-adopt-x11-root-coordinate-model.md`
- `adr/0009-adopt-final-cinnamon-x11-v1-dod-baseline.md`
- `adr/0010-adopt-x11-provider-takeover-shim.md`
- Change artifacts: proposal, spec delta, grill, design, and design-review.

## Existing ADRs Considered

- **ADR 0008**: Not directly affected. This change does not alter X11 root coordinate, input, screenshot, or app-state models.
- **ADR 0009**: Supports the final Cinnamon/X11 v1 DoD baseline by making provider takeover rollback evidence reliable and clean.
- **ADR 0010**: Directly relevant. The change preserves the localized provider takeover shim, standalone plugin identity, and bundled rollback requirement. It strengthens rollback mechanics without expanding takeover scope.

## Grill / Design-Review Findings Considered

- Standalone plugin uninstall is correctly scoped to `$CODEX_HOME` but insufficient after provider takeover install.
- Live webview assets can retain visible `X11 Computer Use` UI after source overlay rollback when backups are not recorded.
- Normal rollback must be manifest-backed and must refuse unsafe blind deletion when marked live assets have no backup.
- Metadata must not be removed before restore succeeds.

## Decisions Evaluated

### Option A: Extend `uninstall-codex-plugin.sh` to remove provider takeover state

Rejected. The standalone plugin uninstaller has a clear narrow ownership boundary and should not mutate target checkouts or `/opt` live assets. Expanding it would surprise users who only installed the standalone plugin.

### Option B: Add a provider-takeover rollback wrapper plus harden lower-level manifest restore

Accepted. This matches the existing one-command provider takeover installer, keeps lower-level source/live restore in `codex-source-overlay.py`, and leaves standalone plugin rollback narrowly scoped.

### Option C: Automatically reverse-patch live assets by string deletion without backups

Rejected for normal rollback. It is unsafe against upstream/minified asset drift and can corrupt live UI bundles. Missing backups should produce a blocker with manual recovery guidance.

## Durable ADR Needed?

No new durable ADR is needed.

Rationale: The durable architecture choice already exists in ADR 0010: localized X11 provider takeover shim with rollback and standalone identity preservation. This change hardens implementation lifecycle and verification details within that architecture. Capturing the behavior in OpenSpec specs/design/tasks is sufficient.

## New Durable ADRs

None.

## Superseded ADRs

None.

## Architecture Snapshot Updates Required

No `ARCHITECTURE.md` update is required because there is no new architecture boundary or durable decision. The current snapshot already names install/uninstall scripts, source overlay, and provider takeover shim as delivery/verification mechanisms.

## Open Questions

None.
