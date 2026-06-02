## Context Read

- Change artifacts: `proposal.md`, `specs/codex-computer-use-settings-ui/spec.md`, `grill.md`, `design.md`
- Project context: `CONSTITUTION.md`, `CONTEXT.md`, `ARCHITECTURE.md`, `adr/README.md`, and in-force ADRs `0001`, `0003`, `0005`, `0006`, `0007`, `0008`, `0009`
- Target implementation context:
  - `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full/AGENTS.md`
  - `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full/scripts/patches/computer-use.js`
  - `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full/scripts/patches/core/all-linux/webview/computer-use-ui/patch.js`
  - `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full/scripts/patch-linux-window-ui.js`
  - `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full/scripts/patch-linux-window-ui.test.js`
  - `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full/tests/scripts_smoke.sh`
  - extracted current settings asset `/tmp/codex-asar-extract/webview/assets/computer-use-settings-Bj9s3CiH.js`

## Design Summary

- Implement a new target patcher function that injects a `codex-computer-use-x11` plugin lookup and `X11 Computer Use` row into the current minified Computer Use settings asset.
- Register the function as an opt-in Computer Use UI webview patch against `computer-use-settings-*.js`.
- Preserve existing `Any App` and `Google Chrome` behavior and the standalone plugin namespace.
- Verify by exported patcher tests, descriptor/smoke coverage, and recorded live/degraded UI evidence.

## Question Loop

### Q1: Is it acceptable to patch a hash-named minified settings asset instead of adding a source-level React component?

**Recommended answer:** Yes, for the current Codex Desktop Linux wrapper architecture.

**Rationale:** The target repository adapts upstream packaged Codex Desktop assets through minified ASAR/webview patches. Existing Computer Use Linux UI behavior already lives in `scripts/patches/computer-use.js` and `scripts/patches/core/all-linux/webview/computer-use-ui/patch.js`. A source-level React component is not available in this target checkout.

**Resolution:** Answered by target repo architecture. Proceed with minified asset patching and fail-soft drift warnings.

### Q2: Should the patch be enabled whenever plugins are enabled or only when the existing Linux Computer Use UI opt-in is enabled?

**Recommended answer:** Only when `context.enableComputerUseUi` is true.

**Rationale:** The row belongs to the visible `Settings -> Computer use` control page. Target repo documentation says visible Computer Use UI patches remain opt-in because they bypass upstream rollout checks; this row should follow the same posture and not create a new hidden route exposure path.

**Resolution:** Design keeps the same opt-in gate as existing Computer Use UI patches.

## Design Findings

- **Finding 1 — plugin availability is separate from row code:** The UI row can only render when `use-plugins` returns `codex-computer-use-x11` in `availablePlugins`. The existing standalone installer/local marketplace must still be run. This is acceptable and already captured as expected behavior.
- **Finding 2 — idempotence marker count must tolerate title and id literals:** Tests should not assume the string `codex-computer-use-x11` appears exactly once in the entire patched asset if the implementation needs both a lookup literal and an id/comment marker. They should assert no duplicate row-injection block or stable output after applying twice.
- **Finding 3 — target repo smoke may be cheaper than full app rebuild:** A direct webview patcher test against a fake/extracted asset is sufficient hard evidence; live app visual verification can be degraded if restart/rebuild is unavailable.
- **Finding 4 — no new durable ADR:** The design follows existing ADR 0009 separation and target patcher architecture. It is not a new hard-to-reverse architectural choice.

## Document Updates Applied

- No proposal/spec/design updates required. The design already records side-by-side row, opt-in patch registration, fail-soft behavior, and live/degraded verification.

## Document Updates Required Before Next Gate

None.

## ADR Candidates

None. No durable ADR is required for this implementation-only UI glue.

## Open Questions

None.
