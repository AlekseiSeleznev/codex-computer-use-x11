# Design Review — fix-review-drift-and-doctor-readiness

## Context Read

- `openspec/changes/fix-review-drift-and-doctor-readiness/proposal.md`
- `openspec/changes/fix-review-drift-and-doctor-readiness/specs/**/spec.md`
- `openspec/changes/fix-review-drift-and-doctor-readiness/grill.md`
- `openspec/changes/fix-review-drift-and-doctor-readiness/design.md`
- `CONSTITUTION.md`, `CONTEXT.md`, `ARCHITECTURE.md`, `adr/README.md`
- Relevant implementation/test docs discovered by review: `src/doctor.rs`, `tests/doctor_cli.rs`, `tests/final_dod.rs`, `tests/packaging_docs.rs`, `README.md`, `docs/release-checklist.md`, `.codex/skills/grill-with-docs/CONTEXT-FORMAT.md`

## Design Summary Reviewed

The design remediates review drift by restoring tracked ADR history, adding ADR-reference validation, refreshing doctor capability/readiness reporting, redacting private ydotool candidate paths, fixing release/README/example-doc drift, and cleaning strict clippy warnings without widening the project's required verification policy.

## Stress-Test Question Loop

### Q1 — Does reconstructing ADRs 0001-0007 violate append-only ADR history?

- **Concern:** Accepted ADR bodies should not be rewritten. Recreating missing files could look like retroactive history editing.
- **Recommended answer:** Treat them as restored tracked records for decisions already referenced by current snapshot/index. Keep their content concise, status-explicit, and consistent with existing references; do not alter ADR 0008/0009 and do not invent new decision outcomes.
- **Resolution:** Design is acceptable. The apply step should label the ADRs as accepted/superseded records matching existing architecture references and keep any new rationale for this remediation in the change-local `adr.md`, not by changing prior accepted semantics.

### Q2 — Does ydotool label redaction break the existing spec that expects `/tmp/.ydotool_socket`?

- **Concern:** The prior doctor socket spec required selecting `/tmp/.ydotool_socket` after stale environment candidates.
- **Recommended answer:** Preserve `/tmp/.ydotool_socket` as a literal public fallback while redacting only environment-derived paths. Existing deterministic ordering remains intact.
- **Resolution:** Design matches the modified spec and preserves compatibility for public fallback diagnostics.

### Q3 — Are focus readiness booleans overclaimed in headless/degraded environments?

- **Concern:** `can_focus_windows` and `can_focus_apps` should not become true merely because JSON can be emitted.
- **Recommended answer:** Compute them from EWMH query/focus prerequisites. Complete fixture/live-ready cases can be true; missing display/tools/EWMH support remain false with blockers or degraded diagnostics.
- **Resolution:** Design is safe if tests cover both complete fixture and missing display/tool cases.

### Q4 — Does `capabilities.planned=[]` break bootstrap compatibility?

- **Concern:** Old tests expected `planned` to contain `x11-ewmh-windowing`.
- **Recommended answer:** The canonical spec already allows `planned` to be empty when planned capabilities become implemented. Preserve array type and move finalized items to `implemented`.
- **Resolution:** Update tests to assert array shape and no stale planned placeholder; this is an intentional compatibility-preserving semantic correction.

### Q5 — Should the final DoD validator parse every Markdown link or only ADR references?

- **Concern:** A broad Markdown link checker could fail on intentionally external or illustrative links and create noise.
- **Recommended answer:** Keep final DoD validation focused on architecture/ADR references, and use focused documentation tests for the specific illustrative link examples that caused local checker confusion.
- **Resolution:** Design remains narrow and avoids expanding validator scope beyond the requirement.

### Q6 — Are clippy fixes allowed to use `#[allow]` attributes?

- **Concern:** Suppression can hide code quality issues.
- **Recommended answer:** Use mechanical refactors for clippy suggestions when straightforward. Use narrow function-level `#[allow(clippy::too_many_arguments)]` only for helper/result constructors where grouping parameters would reduce readability or change no runtime behavior.
- **Resolution:** Acceptable. The apply step should avoid crate/module-level broad allows.

## Findings Requiring Design Changes

None. The reviewed design satisfies the specs and grill resolutions.

## Resolved Terms and CONTEXT.md Updates

- No new glossary terms were identified.
- `CONTEXT.md` does not require changes.

## OpenSpec Artifact Updates Applied or Required

- No proposal/spec/design edits required from this review.
- The ADR review should explicitly state that this change restores traceability for existing decisions rather than introducing a new durable architecture decision.
- The test plan should include RED/GREEN/REFACTOR slices for ADR validation, doctor JSON semantics/privacy, docs drift, and strict clippy cleanup.

## ADR Candidates

- No new durable ADR candidate meets the hard-to-reverse/surprising/trade-off threshold.
- Restored ADR files are historical/durable records already referenced by current project context; they are apply artifacts, not new decisions created by this change.

## Open Questions

None.
