## Context Read

- `openspec/changes/show-x11-plugin-in-computer-use-settings/proposal.md`
- `openspec/changes/show-x11-plugin-in-computer-use-settings/specs/codex-computer-use-settings-ui/spec.md`
- `CONSTITUTION.md`, `CONTEXT.md`, `ARCHITECTURE.md`, `adr/README.md`, and in-force ADRs `0001`, `0003`, `0005`, `0006`, `0007`, `0008`, `0009`
- Target checkout read-only context under `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full`:
  - `AGENTS.md`
  - `README.md` Computer Use UI opt-in section
  - `scripts/patches/computer-use.js`
  - `scripts/patches/core/all-linux/webview/computer-use-ui/patch.js`
  - `scripts/patch-linux-window-ui.test.js`
  - `tests/scripts_smoke.sh`
  - extracted current asset `/tmp/codex-asar-extract/webview/assets/computer-use-settings-Bj9s3CiH.js`
- Current user-local plugin metadata:
  - `/home/as/.codex/plugins/marketplaces/codex-computer-use-x11/.agents/plugins/marketplace.json`
  - `/home/as/.codex/plugins/cache/codex-computer-use-x11/codex-computer-use-x11/latest/.codex-plugin/plugin.json`

## Plan Summary

- The visible Codex Desktop `Settings -> Computer use` page is a hardcoded control page, not a category-driven plugin list.
- The current minified settings asset looks up `computer-use` for the `Any App` row and Chrome ids for the `Google Chrome` row; it never looks up `codex-computer-use-x11`.
- The safe plan is to patch the target webview asset to add a separate side-by-side row for plugin id `codex-computer-use-x11`.
- The plan must preserve the standalone plugin's owned namespace and must not write to or spoof `openai-bundled/computer-use`.
- Verification should be mostly patcher-level and smoke-level because this is minified upstream UI glue, not standalone Rust runtime behavior.

## Question Loop

### Q1: Should the standalone plugin replace the bundled `Any App` row or appear as an additional row?

**Recommended answer:** Add an additional `X11 Computer Use` row.

**Rationale:** Replacing or masquerading as `computer-use` would contradict the existing standalone plugin architecture and ADR 0009's separation between stock target tools and project-owned `x11_*` tools. A side-by-side row keeps the bundled backend available and makes the local X11 backend explicitly testable.

**Resolution:** Answered from repository context and prior user intent. The specs require an additional row and no masquerading.

### Q2: Should the target launcher sync or install the standalone plugin as part of the bundled marketplace?

**Recommended answer:** No for this change; rely on the existing standalone plugin installer/local marketplace and only patch the settings page to render it when available.

**Rationale:** The installed plugin already exists in the owned `codex-computer-use-x11` marketplace/cache. Mutating `openai-bundled` would blur ownership and introduce update conflicts. The settings row can use the same available-plugin query that already includes local marketplaces.

**Resolution:** Answered from current user-local cache/config and source-overlay safety rules. The specs prohibit overwriting bundled paths.

### Q3: Should live visual UI verification be a hard archive blocker?

**Recommended answer:** No; automated patcher evidence is the hard requirement, and live UI verification is recorded when the current app can be rebuilt/restarted safely.

**Rationale:** The Codex app may cache webview assets and plugin schemas inside the running process. Requiring an immediate visual check as a hard gate would make verification depend on app lifecycle outside the code change. The spec requires the blocker to be recorded if live UI verification is unavailable.

**Resolution:** Captured in the verification requirement and test-plan expectations.

## Resolved Terms

- **Computer Use settings page**: the Codex Desktop settings route `/settings/computer-use`, implemented as a hardcoded control surface for specific Computer Use plugin ids rather than a generic plugin category listing.
- **X11 Computer Use row**: the new side-by-side settings control row backed by plugin id `codex-computer-use-x11`.

No `CONTEXT.md` update was required because these terms are change-local UI integration terms rather than durable domain vocabulary for the standalone backend.

## Document Updates Applied

- The spec explicitly requires side-by-side rendering, preservation of bundled rows, no masquerading as `openai-bundled/computer-use`, idempotent webview patching, and recorded live UI verification/degradation.

## Document Updates Required Before Next Gate

None.

## ADR Candidates

- A durable ADR is not required. The side-by-side identity decision follows existing ADR 0009 and the standalone plugin/source-overlay boundary already recorded in architecture/specs. This change is an implementation of that boundary in Codex Desktop UI glue, not a new durable architecture direction.

## Open Questions

None.
