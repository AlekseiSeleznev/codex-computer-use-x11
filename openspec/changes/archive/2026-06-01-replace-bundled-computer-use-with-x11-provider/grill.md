## Context Read

- Change artifacts:
  - `openspec/changes/replace-bundled-computer-use-with-x11-provider/proposal.md`
  - `openspec/changes/replace-bundled-computer-use-with-x11-provider/specs/codex-computer-use-provider-takeover/spec.md`
  - `openspec/changes/replace-bundled-computer-use-with-x11-provider/specs/codex-computer-use-settings-ui/spec.md`
  - `openspec/changes/replace-bundled-computer-use-with-x11-provider/specs/codex-source-overlay-extension/spec.md`
  - `openspec/changes/replace-bundled-computer-use-with-x11-provider/specs/standalone-codex-mcp-plugin/spec.md`
- Project context:
  - `CONSTITUTION.md`
  - `CONTEXT.md`
  - `ARCHITECTURE.md`
  - `adr/README.md`
  - In-force ADRs `0001`, `0003`, `0005`, `0006`, `0007`, `0008`, `0009`
- Prior active attempt context:
  - `openspec/changes/show-x11-plugin-in-computer-use-settings/proposal.md`
  - `openspec/changes/show-x11-plugin-in-computer-use-settings/specs/codex-computer-use-settings-ui/spec.md`
  - `openspec/changes/show-x11-plugin-in-computer-use-settings/design.md`
  - `openspec/changes/show-x11-plugin-in-computer-use-settings/test-plan.md`
- Target checkout context under `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full`:
  - `AGENTS.md`
  - `README.md` Linux Computer Use section
  - `scripts/patches/computer-use.js`
  - `scripts/patches/core/all-linux/webview/computer-use-ui/patch.js`
  - `scripts/patch-linux-window-ui.test.js`
- Live asset facts inspected read-only:
  - `/opt/codex-desktop/content/webview/assets/computer-use-settings-aHZZtKP_.js` exists and contains the prior `codexLinuxX11Plugin` marker, `installedPlugins`, `availablePlugins`, `computerUseAvailability`, and `settings.computerUse.anyApp` literals.

## Plan Summary

- The side-by-side row approach is insufficient because live UI feedback still shows only Google Chrome, even after the asset contains the X11 marker and the X11 lookup uses installed-first fallback.
- The next plan must first make the bundled baseline explainable: record what the settings page really receives and why the bundled `Any App` row is shown, hidden, gated, or absent.
- X11 takeover mode intentionally changes the row decision for the Computer Use provider surface: hide or disable the bundled `computer-use` row and show `codex-computer-use-x11` as the active provider row.
- The takeover must not globally masquerade the standalone plugin as `computer-use`; any alias must stay inside the settings/provider resolver payload.
- The installer path must be reversible from this repository over the target checkout and, when explicitly applied to live assets, must back up first, report all patches, and tell the user to restart Codex Desktop.

## Question Loop

### Q1: Is takeover mode allowed to bypass or destroy the bundled Computer Use feature gates and install flow?

**Recommended answer:** No. Takeover mode should bypass the bundled row only at the settings/provider row decision, while preserving the bundled plugin's marketplace/cache identity and existing install flow for rollback and non-takeover mode.

**Rationale:** The target README documents Linux Computer Use UI patches as opt-in and distinguishes default bundled MCP registration from visible UI controls. ADR 0009 also keeps the standalone X11 plugin separate from stock target tools. Destroying bundled gates would make rollback unreliable and would turn a UI provider-selection change into a global plugin identity change.

**Resolution:** Answered from `README.md`, ADR 0009, and the proposal constraints. Specs require a localized compatibility shim and rollback to bundled mode.

### Q2: Should X11 takeover be implemented by renaming `codex-computer-use-x11` to `computer-use` in the plugin catalog?

**Recommended answer:** No. If a compatibility alias is required, synthesize it only inside the settings/provider resolver layer.

**Rationale:** The standalone plugin spec already owns `codex-computer-use-x11@codex-computer-use-x11` and `x11_*` tools. Renaming the plugin globally would break marketplace ownership, could overwrite `openai-bundled/computer-use`, and would contradict the explicit user constraint to avoid a coarse plugin-id masquerade when a safer provider shim is possible.

**Resolution:** Answered from existing specs and user constraints. The new specs prohibit global renames and bundled marketplace rewrites.

### Q3: Should takeover mode silently fall back to the bundled row when the X11 plugin is missing?

**Recommended answer:** No for takeover mode. It should show a clear unavailable/diagnostic state for X11 and keep the bundled provider hidden or disabled so the operator can see that takeover is misconfigured.

**Rationale:** The current problem is invisibility: the UI can look normal while the desired provider is absent. Silent fallback would recreate the same debugging failure. Rollback is the explicit path back to bundled behavior.

**Resolution:** Captured in `codex-computer-use-provider-takeover` scenarios: missing X11 provider reports unavailable instead of silently falling back to only Chrome or bundled Any App.

### Q4: Is live asset mutation acceptable as part of the installer flow?

**Recommended answer:** Yes, but only as an explicit installer/apply task with backup, patch report, and restart hint. Planning artifacts must not mutate live assets.

**Rationale:** Prior work showed that patching `/opt/codex-desktop/content/webview/assets/computer-use-settings-*.js` can be useful for immediate local verification, but the running Electron/webview process may keep stale loaded state. Because `/opt` is outside the source tree and root-owned, every live write must be reversible and visible in a report.

**Resolution:** Specs require timestamped live-asset backups, machine-readable patch reports, rollback/restore behavior, and restart guidance. No live asset mutation occurs during this planning gate.

### Q5: Is the prior completed side-by-side change a blocker for this new change?

**Recommended answer:** No, but it is evidence and context. This change supersedes the side-by-side row assumption at the planning level by adding takeover diagnostics and provider-resolution behavior; archive/push of either change remains out of scope until explicitly approved.

**Rationale:** `show-x11-plugin-in-computer-use-settings` is complete but not archived. The user explicitly asked for a new change at `replace-bundled-computer-use-with-x11-provider`. The new change can consume the prior findings without archiving or pushing.

**Resolution:** Proceed with the new change and record the side-by-side attempt as context, not as the active design target.

## Resolved Terms

- **Provider takeover**: a mode where the settings/provider resolver intentionally selects an alternate provider as the active Computer Use provider surface and hides or disables the bundled provider row for that surface until rollback.
- **Provider shim**: a localized compatibility object or alias used by the settings/provider layer so existing row controls can render the selected provider without changing the global plugin catalog identity.
- **Bundled mode**: the non-takeover state where the Codex Desktop settings page uses the target repo's ordinary bundled `computer-use` `Any App` row decision.

No `CONTEXT.md` update was applied in this gate because these are change-local planning terms for the target settings/provider layer; durable glossary updates can be added during design if the terms become project-wide.

## Document Updates Applied

- The specs already require baseline diagnostics before replacement, installed-first X11 provider resolution, no silent fallback in takeover mode, no global plugin-id masquerade, live-asset backup/reporting, and rollback to bundled mode.
- No proposal or spec edits were required after the grill because the material risks were already represented in the generated requirements.

## Document Updates Required Before Next Gate

None.

## ADR Candidates

- **Provider takeover/shim instead of global plugin-id masquerade** is a durable ADR candidate. It is hard to reverse once installer and target settings behavior rely on it, surprising without context because the simpler hack is to rename the plugin to `computer-use`, and it is a real trade-off between compatibility and identity safety. The per-change `adr.md` should record the decision and decide whether a top-level durable ADR is warranted.

## Open Questions

None.
