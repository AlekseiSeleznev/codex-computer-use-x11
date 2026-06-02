# Design — fix-review-drift-and-doctor-readiness

## Goals

- Make the final v1 handoff internally consistent after full documentation/code review.
- Keep OpenSpec and root project context as the source of truth; do not introduce a new architecture scope.
- Preserve additive JSON compatibility for `doctor --json` while removing stale finalized-v1 placeholders.
- Keep diagnostics useful without serializing private environment-derived local paths.
- Keep the required verification surface (`make fmt`, `make check`, `make test`) unchanged while proving strict clippy cleanliness for this remediation.

## Non-goals

- Do not archive the change without explicit user approval.
- Do not push, merge, or modify external systems.
- Do not patch installed OpenSpec packages or mutate the Codex Desktop Linux target checkout.
- Do not add a new durable architecture decision unless implementation reveals a hard-to-reverse trade-off not already covered by in-force ADRs.
- Do not make clippy a default `Makefile` gate in this change.

## Current Findings

1. `ARCHITECTURE.md` and `adr/README.md` reference ADRs 0001-0007, but only ADRs 0008/0009 are currently tracked. The architecture snapshot therefore points to missing durable rationale.
2. `src/doctor.rs` still reports `can_focus_apps=false`, `can_focus_windows=false`, `capabilities.planned=["x11-ewmh-windowing"]`, even though v1 X11/EWMH windowing and verified focus behavior are implemented elsewhere in the repository.
3. `gather_system_facts()` probes ydotool socket candidates with real environment-derived paths and serializes them through `YdotoolCandidate.path`; this can expose machine-private paths in a shareable doctor report.
4. `docs/release-checklist.md` contains a validation command for an already-archived active change.
5. README/source-overlay language still includes bootstrap-era/read-only/future phrasing.
6. `.codex/skills/grill-with-docs/CONTEXT-FORMAT.md` uses Markdown links for illustrative files that intentionally do not exist in this repository.
7. Strict clippy exposes cleanup warnings that are safe to fix locally.

## Decisions

### D1 — Restore referenced ADR files instead of deleting references

Create tracked top-level ADR files for all referenced missing numbers 0001-0007. ADRs 0002 and 0004 will be marked superseded historical context; ADRs 0001/0003/0005/0006/0007 will match the in-force/supersession statements already present in `ARCHITECTURE.md` and `adr/README.md`.

Rationale: the snapshot and index already claim these decisions exist. Restoring files preserves durable rationale and makes validator/test coverage meaningful.

### D2 — Add ADR-reference validation

Extend final DoD validation/tests to scan `ARCHITECTURE.md` and `adr/README.md` for top-level `adr/NNNN-*.md` references and fail if any referenced file is missing.

Rationale: this prevents reintroducing the same documentation drift.

### D3 — Refresh doctor implemented/planned capability facts

Replace stale `planned=["x11-ewmh-windowing"]` with implemented finalized-v1 capability facts and an empty planned array unless future planned items are actually present. Required implemented entries include at least:

- `doctor-json`
- `doctor-capability-detection`
- `x11-ewmh-windowing`
- `x11-ewmh-window-listing`
- `x11-ewmh-focus-with-verification`
- finalized delivery/evidence capabilities represented by existing specs, docs, and tests

Rationale: keep additive shape but avoid reporting already implemented v1 work as planned.

### D4 — Compute focus readiness from EWMH query prerequisites

Set `can_focus_windows` from the same verified prerequisites that make EWMH window operations usable (`DISPLAY`, `wmctrl`, `xprop`, and successful EWMH active-window/supporting-window probing). Set `can_focus_apps` to the same value for this standalone report only when app-focus semantics are documented as verified X11 window activation; otherwise keep the distinction in detail text. For this repository's v1 baseline, verified window activation is the app/window focus boundary, so both booleans may become true in the complete X11/EWMH fixture.

Rationale: downstream readiness vocabulary expects both fields; the v1 implementation focuses top-level windows as the safe app proxy.

### D5 — Redact environment-derived ydotool candidate paths in serialized reports

Introduce a candidate representation used during live fact gathering that separates the real local path used for `UnixStream::connect` from the serialized label. Keep existing fixture-friendly `ProbeFacts.ydotool_candidates: Vec<(String, bool)>` semantics as already-sanitized labels, and make `gather_system_facts()` produce labels:

- `env:YDOTOOL_SOCKET`
- `env:XDG_RUNTIME_DIR/.ydotool_socket`
- `/tmp/.ydotool_socket`

`connectable_socket` will report the selected label. The public `/tmp/.ydotool_socket` fallback remains literal.

Rationale: avoid private path disclosure while preserving deterministic candidate ordering and availability diagnostics.

### D6 — Keep documentation fixes narrow and test-backed

Update README, release checklist, and illustrative link examples only where review drift was identified. Add tests to assert release checklist no longer includes the archived active-change command and that illustrative context-format examples do not register as broken local Markdown links.

Rationale: reduce blast radius while making the drift regression-resistant.

### D7 — Resolve clippy warnings with local refactors or targeted allows for test/helper ergonomics

Prefer small refactors matching clippy suggestions (`sort_by_key`, `split_whitespace`, direct initializers, simpler conditionals). If helper functions intentionally have many arguments for JSON result construction, use narrow `#[allow(clippy::too_many_arguments)]` on that helper rather than wider crate-level suppression.

Rationale: strict clippy should pass without changing public behavior or project policy.

## Implementation Plan

### Slice 1 — Architecture/ADR traceability

1. Add tracked ADR files 0001-0007 with status, context, decision, consequences, and supersession notes matching the snapshot/index.
2. Extend `tests/final_dod.rs` and/or `scripts/validate-final-dod.py` to reject missing referenced ADR files.
3. Run the final DoD validator test/command.

### Slice 2 — Doctor readiness/capabilities/privacy

1. Update doctor capability list and planned list.
2. Update `readiness_report()` focus booleans to reflect finalized X11/EWMH window focus semantics.
3. Add/adjust unit and CLI tests for implemented capabilities, empty/no-stale planned list, and focus booleans under a complete fixture.
4. Refactor live ydotool candidate gathering to use serialized labels for env-derived paths while connecting to real local paths internally.
5. Add a live-gather/fake-env test proving private ydotool path values are absent from serialized JSON.

### Slice 3 — Documentation drift

1. Refresh README source-overlay posture language.
2. Replace stale release-checklist active archived-change validation with durable validation commands.
3. Convert illustrative skill-template links to non-link path examples.
4. Add documentation tests for the release checklist and illustrative link examples.

### Slice 4 — Strict clippy cleanup

1. Apply clippy-suggested refactors and narrow helper-specific allows as needed.
2. Verify `cargo clippy --all-targets --all-features -- -D warnings`.

## Testing Strategy

- Use test-first apply slices where behavior changes are observable:
  - RED for ADR-reference validation before adding missing ADR files or validation logic.
  - RED for doctor capabilities/focus/privacy expectations before production edits.
  - RED for release-checklist/docs-link checks before doc edits.
  - RED for strict clippy by running the exact command before cleanup, then GREEN after cleanup.
- Preserve existing `make fmt`, `make check`, and `make test` gates.
- Run OpenSpec validation for the active change and all specs.
- Run `scripts/check-overlay` because `.codex/skills/...` documentation changes touch overlay content.
- Run `scripts/validate-final-dod.py` after ADR/DoD validation changes.

## Risks and Mitigations

| Risk | Mitigation |
| --- | --- |
| Reconstructed ADRs accidentally rewrite project history | Mark them as restored durable records that reflect already-referenced decisions; do not supersede or alter ADR 0008/0009. |
| Doctor readiness overclaims focus capability | Tie focus booleans to EWMH query/focus prerequisites and tests; keep recommended/degraded diagnostics for unavailable prerequisites. |
| Ydotool redaction removes useful troubleshooting data | Preserve deterministic candidate labels and connectability booleans; keep public fallback literal. |
| Clippy cleanup changes behavior | Use tests before/after and prefer mechanical refactors; avoid broad suppressions. |
| Release checklist loses useful archived-change validation | Keep durable `openspec validate --all --strict`; active change validation remains a per-change workflow command, not a post-archive release gate. |

## Migration and Compatibility

- Doctor JSON top-level shapes remain additive-compatible: `project`, `version`, `backend`, `readiness`, `capabilities`, and `checks` stay present with the same field types.
- `capabilities.planned` remains an array; it may become empty.
- Existing tests/consumers that require the stale `x11-ewmh-windowing` planned placeholder must update to inspect implemented capabilities instead.
- Restored ADR files add tracked documentation only; they do not change code execution.
- Documentation changes do not require external credentials or target-checkout writes.

## Open Questions

None.
