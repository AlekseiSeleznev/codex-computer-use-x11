## Context Read

- Change artifacts:
  - `proposal.md`
  - `specs/codex-computer-use-provider-takeover/spec.md`
  - `specs/codex-computer-use-settings-ui/spec.md`
  - `specs/codex-source-overlay-extension/spec.md`
  - `specs/standalone-codex-mcp-plugin/spec.md`
  - `grill.md`
  - `design.md`
- Project context:
  - `CONSTITUTION.md`
  - `CONTEXT.md`
  - `ARCHITECTURE.md`
  - `adr/README.md`
  - In-force ADRs `0001`, `0003`, `0005`, `0006`, `0007`, `0008`, `0009`
- Target and implementation context:
  - `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full/AGENTS.md`
  - `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full/README.md` Linux Computer Use UI opt-in section
  - `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full/scripts/patches/computer-use.js`
  - `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full/scripts/patches/core/all-linux/webview/computer-use-ui/patch.js`
  - `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full/scripts/patch-linux-window-ui.test.js`
  - Project overlay manager `scripts/codex-source-overlay.py` and install/status/uninstall shell wrappers
- Prior attempt context:
  - `openspec/changes/show-x11-plugin-in-computer-use-settings/design.md`
  - `openspec/changes/show-x11-plugin-in-computer-use-settings/test-plan.md`

## Design Summary

- The design moves from static side-by-side row injection to a provider resolver that can explain bundled baseline behavior and choose X11 takeover mode deliberately.
- Diagnostics are required before replacement: sanitized plugin collections, `computerUseAvailability`, gate facts, asset marker version, and row decisions.
- X11 takeover mode is explicit: `--provider x11 --mode takeover` selects `codex-computer-use-x11` installed-first, hides or disables the bundled `computer-use` row for the Any App provider surface, and preserves Chrome row behavior.
- Any compatibility shim is local to the settings/provider row payload; global plugin identity and bundled marketplace/cache ownership remain unchanged.
- Installer work extends the existing reversible overlay manager with report, backup, live-asset patch, restart hint, status, and rollback behavior.

## Question Loop

### Q1: Can the provider resolver be tested without a live Codex Desktop Electron process?

**Recommended answer:** Yes. Treat the resolver as a public patcher/test boundary with fixture payloads and current/memoized settings asset shapes; live UI verification remains a final smoke/degraded evidence boundary.

**Rationale:** The target repo already tests minified patch behavior with `node --test scripts/patch-linux-window-ui.test.js`, and prior work proved live Electron can retain stale loaded assets until full restart. Making live UI the first proof would block TDD and reintroduce non-determinism.

**Resolution:** The test plan should put fixture resolver tests first and live UI verification last.

### Q2: Should the installer patch live `/opt` assets by default when applying target source overlay?

**Recommended answer:** No. Source checkout overlay and live asset patching should be separate flags, with live patching explicit and reported.

**Rationale:** `/opt` assets are root-owned generated install artifacts, not the durable source checkout. The constitution permits target checkout work for this change, but live root-owned mutation is higher risk and must be backed up/restorable. The user specifically requires live asset backup and restart hint, not silent live mutation.

**Resolution:** Design keeps live patching explicit through a `--patch-live-assets`-style option and requires backup/report/rollback.

### Q3: Does takeover contradict ADR 0009's stock-tool separation?

**Recommended answer:** No if and only if takeover is UI/provider selection, not a global plugin or tool rename.

**Rationale:** ADR 0009 says standalone MCP tools stay namespaced as `x11_*` and source-overlay stock tools should prefer target stock tools. This design changes which provider row the settings UI exposes; it does not rename tools or make the standalone plugin claim bundled ownership.

**Resolution:** ADR step must record provider takeover/shim as a deliberate identity-preserving decision.

## Design Findings

- **Finding 1 — diagnostics need an explicit sink:** The design names diagnostic fields but leaves the exact sink to implementation. This is acceptable for design because tests can define a report JSON sink first. Tasks should require a concrete report path or exported diagnostic function before production patching.
- **Finding 2 — prior side-by-side marker migration is mandatory in apply:** The live asset already contains `codexLinuxX11Plugin`. Takeover apply must not treat any existing X11 marker as proof that takeover is active. It needs a distinct marker version and migration/removal behavior for older side-by-side snippets.
- **Finding 3 — rollback must handle generated-source and live-asset drift separately:** Target source rollback can remove marker blocks, but live asset restore should verify backup metadata/checksum or owned takeover markers before writing. Unknown live asset drift should stop with an explicit report rather than overwriting.
- **Finding 4 — bundled baseline fixture comes before takeover fixture:** The first RED slice should prove the baseline `Any App` row can be diagnosed/rendered in fixture data. Otherwise takeover could hide a still-unexplained upstream row-shape bug.
- **Finding 5 — no secret/external-system issue:** Runtime plugin ids, marketplace names, feature booleans, and row decisions are not credentials. Diagnostics must still avoid auth tokens, private URLs, and raw user data.

## Document Updates Applied

No artifact updates were required during design review. The existing design already requires distinct takeover markers, migration/removal of prior side-by-side row markers, explicit live asset patching, backup/report/rollback, and fixture-first tests.

## Document Updates Required Before Next Gate

None.

## ADR Candidates

- **Provider takeover/shim instead of global masquerade plugin id** remains a durable ADR candidate. It is hard to reverse once installer and settings resolver behavior are built around it, surprising because global plugin-id spoofing would be the obvious shortcut, and a real trade-off between compatibility and safe ownership boundaries.

## Open Questions

None.
