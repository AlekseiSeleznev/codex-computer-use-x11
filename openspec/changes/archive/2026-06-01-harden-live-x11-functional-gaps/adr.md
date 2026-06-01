## ADR Review

This per-change ADR review records architectural decisions for `harden-live-x11-functional-gaps`. It does not create or supersede durable top-level ADRs because the selected design stays within ADR 0008, ADR 0009, and ADR 0010.

## Existing In-Force ADRs

- `adr/0001-adopt-codex-native-intent-driven-openspec-overlay.md` — Accepted; remains in force for OpenSpec-as-source-of-truth workflow.
- `adr/0003-formalize-project-context-entrypoints.md` — Accepted; remains in force for `CONSTITUTION.md`, `ARCHITECTURE.md`, ADR, and local-secret boundaries.
- `adr/0005-adopt-matt-grill-and-tdd-gates.md` — Accepted; remains in force for mandatory grill/design-review and TDD apply discipline.
- `adr/0006-adopt-claude-artifact-review.md` — Accepted; remains in force, but session Claude review is currently disabled.
- `adr/0007-adopt-automatic-checkpoints-and-claude-session-controls.md` — Accepted; remains in force for safe scoped lifecycle checkpoint commits.
- `adr/0008-adopt-x11-root-coordinate-model.md` — Accepted; remains in force. Overlay bounds and screenshot/window evidence stay in X11 root/global coordinates.
- `adr/0009-adopt-final-cinnamon-x11-v1-dod-baseline.md` — Accepted; remains in force. This change hardens degraded live evidence without weakening verified-focus input or AT-SPI confidence safety.
- `adr/0010-adopt-x11-provider-takeover-shim.md` — Accepted; remains in force. This change does not globally masquerade plugin identity or rename standalone tools.
- Superseded ADRs `0002` and `0004` remain historical only and are not revived.

## Constitution / Architecture Rules Considered

- Rust 2021/root Cargo/Makefile remain the implementation and verification baseline.
- OpenSpec lifecycle order remains mandatory; no implementation before proposal/specs/grill/design/design-review/adr/test-plan/tasks are complete.
- Secret values stay out of Git-tracked artifacts and summaries; this change needs no external-system credentials.
- Runtime command dependencies must be documented and degraded when unavailable.
- `x11-ewmh` remains canonical backend identity.
- X11 root/global coordinates remain canonical for target bounds, overlays, pointer points, and screenshot/app-state context.
- Safe targeted input requires target resolution and exact focus verification before backend injection.
- AT-SPI correlation must return no subtree on ambiguity or low confidence.
- Provider takeover remains localized; no global plugin id masquerade beyond ADR 0010.

## Decisions Evaluated

- **Decision: Unicode input route ordering.**
  - Accepted for this change: verified target focus -> active-context X11 Unicode keysyms (`Uxxxx`) for non-ASCII text -> explicit `clipboard-paste` fallback only when exact fidelity cannot be achieved by keysyms.
  - Rationale: The live run showed layout-dependent Cyrillic through `xdotool type`. Unicode keysyms are a narrower X11-native fix that preserves the active-context safety boundary. Clipboard paste is more likely to provide exact text but mutates clipboard state, so it is fallback-only with restore diagnostics.
  - Rejected: direct `xdotool --window` as a safety boundary because ADR 0009 rejects direct-window SendEvent/XSendEvent as proof of targeted safety.
  - Rejected: `ydotool` as primary Unicode fix because it is uinput/scancode/layout-bound, requires `ydotoold`, and does not directly prove exact Unicode fidelity.
  - Consequence: Tests must prove alias normalization, stderr semantic failure, Unicode keysym args, and exact Cyrillic value in live mode; fallback tests must prove restoration or explicit restoration warning.

- **Decision: AT-SPI precision over permissive matching.**
  - Accepted for this change: token-boundary/exact-token class matching, target-scoped xprop enrichment, missing-signal diagnostics, GTK-positive fixture.
  - Rationale: Live Tk fixtures gave `NoAccessibilityMatch`, and current substring class matching can create false positives such as `tk` matching `gtk3`. Better diagnostics and a positive GTK fixture are safer than relaxing threshold.
  - Rejected: bounds-only matching for Tk windows.
  - Consequence: Tk/Tkinter remains useful keyboard/pointer fixture evidence; AT-SPI pass evidence must come from a GTK fixture or another accessible app with real semantic signals.

- **Decision: Standalone overlay provider boundary.**
  - Accepted for this change: implement an overlay provider using `x11rb` or a helper process that creates non-focus `override-redirect` border windows around target X11 root-coordinate bounds. Overlay windows must have title/class `codex-computer-use-x11-overlay`, must be excluded from target listing, and must be hidden on release.
  - Rationale: The current standalone target context lifecycle works but reports no overlay provider. A real X11-owned overlay improves UX and acceptance evidence without making visual display a hard target-state dependency.
  - Rejected: making overlay failure block target save.
  - Rejected: letting overlay windows appear as ordinary app targets.
  - Consequence: Apply must include overlay lifecycle and stale/release cleanup tests. If `x11rb` dependency proves too large, helper process design remains acceptable within this per-change decision.

- **Decision: App-state/evidence cleanup.**
  - Accepted for this change: summarize app-state layers from `diagnostics.layers`, add no-screenshot-data evidence summaries, and classify RemoteDesktop portal gaps as optional/report-only for the X11 path when core X11 functions work.
  - Rationale: Live evidence showed screenshot capture worked, AT-SPI degraded, and portal incomplete did not block X11/EWMH focus/input/screenshot. Summaries should reflect that accurately and avoid embedding huge base64 data.
  - Rejected: treating portal absence as the top recommended blocker for the supported Cinnamon/X11 path.

## New Durable ADRs Created

- None.

## Superseded ADRs

- None.

## Architecture Snapshot Updates

- None required during planning. `ARCHITECTURE.md` already covers the relevant in-force constraints through ADR 0008/0009/0010.
- If apply selects a hard-to-reverse overlay dependency or changes the final Cinnamon/X11 baseline claim beyond current degraded/pass semantics, create a future durable ADR and update `ARCHITECTURE.md` in that future change.

## No ADR Needed

- No new durable top-level ADR is needed now because the decisions are scoped to hardening an already accepted Cinnamon/X11 v1 baseline and do not supersede ADR 0008, ADR 0009, or ADR 0010.
- The route and provider trade-offs are still recorded here because every intent-driven change requires a per-change ADR review, and the user explicitly requested decisions on Unicode routing and overlay provider.
