## Context Read

- `CONSTITUTION.md` — Rust/Cargo root crate, OpenSpec validation, secret handling, target checkout guidance, and required `make fmt` / `make check` / `make test` verification.
- `CONTEXT.md` — existing terms: `x11-ewmh`, standalone plugin, source overlay, active window, focus verification, and `FocusNotVerified`; updated with `AT-SPI window correlation` and `Accessibility tree` glossary entries.
- `ARCHITECTURE.md` and `adr/README.md` — lifecycle gates, automatic checkpoints, optional Claude review controls, no-secrets boundary, and source-overlay/standalone boundaries. Referenced ADR bodies are absent in this checkout; README/snapshot are the available in-force context.
- `backlog/00-research-reuse-map.md` and `backlog/08-atspi-window-correlation.md` — milestone order, fresh-research requirements, confidence/ambiguity requirements, PID reliability cautions, terminal/browser edge cases, and acceptance checks.
- Change artifacts: `proposal.md`, `specs/x11-atspi-window-correlation/spec.md`, and `specs/standalone-codex-mcp-plugin/spec.md`.
- Existing canonical specs: `openspec/specs/standalone-codex-mcp-plugin/spec.md`, `openspec/specs/x11-window-listing/spec.md`, `openspec/specs/x11-active-window-focus/spec.md`, `openspec/specs/x11-targeted-input-safety/spec.md`, and `openspec/specs/x11-pointer-actions/spec.md`.
- Standalone code: `src/cli.rs`, `src/list_windows.rs`, `src/focus.rs`, `src/input.rs`, `src/pointer.rs`, `src/mcp.rs`, `src/doctor.rs`, and integration tests under `tests/`.
- Target checkout `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full`: branch `main`, clean; reviewed `computer-use-linux/src/atspi_tree.rs`, `server.rs`, `terminal.rs`, `diagnostics.rs`, `windowing/types.rs`, and `windowing/target.rs`.
- Current docs/research: docs.rs `atspi`, Ubuntu AT-SPI DBus Accessible/Component reference, GitHub metadata for `Touchpoint-Labs/Touchpoint`, `BeckhamLabsLLC/linux-desktop-mcp`, `tak-uukti/linux-computer-use`, `wimi321/linux-computer-use-skill`, `MONTBRAIN/vadgr-computer-use`, and `joe223/sootie`.
- Local live probes: Cinnamon/X11 session; `toolkit-accessibility=true`; `org.a11y.Bus` present; `gdbus`, `busctl`, and `python3` installed.

## Plan Summary

- Add a standalone `accessibility-tree --window-id <id> --json` command that resolves a current X11 window and returns AT-SPI semantic nodes only after a confident correlation.
- Add `x11_accessibility_tree` to the standalone MCP tool surface in project-owned order after pointer tools.
- Correlate by scoring multiple signals: reliable sidecar PID, title/name similarity, wm_class/app-name similarity, bounds overlap, and focus state.
- Return explicit `ambiguous` or `degraded` states instead of guessing when AT-SPI is unavailable, PID is unreliable, candidates tie, or score is below threshold.
- Keep target checkout and external repos read-only/reference-only; no external code copy or source overlay mutation in this stage.

## Question Loop

### Q1: Should this change use the Rust `atspi`/`zbus` stack now or a command-testable collector boundary?

- Recommended answer: use a command-testable collector boundary now, with a minimal live collector that can degrade cleanly, and keep the pure matcher as the core behavior.
- Rationale: the standalone crate currently depends only on `serde`/`serde_json`; adding async AT-SPI dependencies would enlarge the implementation before the confidence model is proven. The target repo already uses Rust `atspi` for eventual source-overlay integration, while this standalone stage benefits from fake `PATH` tests and explicit degraded output.
- Resolution: no user question needed. Design will keep correlation/matching pure and make live AT-SPI collection a bounded external boundary that can report `AtspiUnavailable`.

### Q2: Is PID allowed to decide the match by itself?

- Recommended answer: only when the X11 sidecar marks PID reliability as reliable and no other candidate creates ambiguity; unreliable or unknown PID must be discounted and corroborated by title/class/bounds/focus.
- Rationale: backlog and current `WindowMetadata.pid_reliability` already warn that PID can be absent or unreliable. Browser and terminal cases explicitly violate naive PID matching.
- Resolution: no user question needed. Specs already require reliable PID and non-PID evidence; design must carry `PidReliability` into the matcher.

### Q3: Does `accessibility-tree` imply input safety or only semantic read context?

- Recommended answer: semantic read context only.
- Rationale: `Focus verification` remains the input safety boundary. AT-SPI nodes can expose actions, but this change only returns a correlated tree and must not imply keyboard/pointer injection is isolated.
- Resolution: updated `CONTEXT.md` to define `Accessibility tree` as semantic context, not proof of safe input targeting.

### Q4: Should ambiguous AT-SPI candidates still return a best-effort tree with low confidence?

- Recommended answer: no; ambiguity returns no subtree and includes candidate diagnostics.
- Rationale: a wrong semantic tree can mislead subsequent actions. Existing project safety posture prefers degraded/ambiguous results over false precision.
- Resolution: specs already require `AmbiguousAccessibilityMatch`; design must keep `tree` empty when status is `ambiguous`.

### Q5: Does the proposal require a durable ADR?

- Recommended answer: no durable ADR for this change.
- Rationale: the approach applies existing project-wide decisions: standalone-before-source-overlay, no code copy, and safe degraded reporting. The matcher thresholds and collector boundary are local and reversible.
- Resolution: per-change `adr.md` will record no new durable ADR unless design-review uncovers a hard-to-reverse architecture decision.

## Resolved Terms

- `AT-SPI window correlation` — added to `CONTEXT.md` as the multi-signal match between an `x11-ewmh` window and one AT-SPI subtree.
- `Accessibility tree` — added to `CONTEXT.md` as semantic context for a selected window; not an input-safety guarantee.

## Document Updates Applied

- Updated `CONTEXT.md` with glossary entries for `AT-SPI window correlation` and `Accessibility tree`.
- No proposal or spec changes were required after grilling; the specs already encode ambiguity/degraded behavior and PID reliability constraints.

## Document Updates Required Before Next Gate

None.

## ADR Candidates

None. No hard-to-reverse, surprising, project-wide architecture decision is introduced by this standalone correlation stage.

## Open Questions

None
