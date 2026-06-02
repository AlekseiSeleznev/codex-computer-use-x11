## Context Read

- `CONSTITUTION.md` — project rules, verification, secret handling, and OpenSpec lifecycle discipline.
- `CONTEXT.md` — glossary terms for OpenSpec, Codex overlay, project constitution, architecture snapshot, grill gate, TDD slice, `x11-ewmh`, standalone plugin, and source overlay.
- `ARCHITECTURE.md` and `adr/README.md` — in-force ADR set and lifecycle/checkpoint constraints.
- `openspec/changes/cleanup-spec-purpose-metadata/proposal.md` — scope is metadata-only cleanup of two canonical spec Purpose sections.
- `openspec/changes/cleanup-spec-purpose-metadata/specs/doctor-cli/spec.md` and `specs/x11-integration-contract/spec.md` — delta requirements for non-placeholder Purpose metadata.
- `openspec/specs/doctor-cli/spec.md` and `openspec/specs/x11-integration-contract/spec.md` — confirmed current `## Purpose` text still contains bootstrap `TBD` placeholders.

## Plan Summary

- The change replaces placeholder Purpose metadata in two canonical specs that were originally created by archiving `bootstrap-codex-computer-use-x11`.
- The change is documentation/spec metadata only; it must not alter Rust code, runtime CLI behavior, source-overlay behavior, or target-checkout files.
- The delta specs intentionally make non-placeholder Purpose metadata observable through text checks because OpenSpec delta syntax cannot directly patch the canonical `## Purpose` section during artifact planning.
- Verification should combine OpenSpec validation with a grep/text check that the two placeholder strings are gone.

## Question Loop

- Question considered: Should this change edit canonical spec Purpose metadata directly during apply rather than relying on archive spec sync?
  - Recommended answer: Yes. The archived previous change already recorded that OpenSpec delta specs do not update canonical Purpose text directly, so apply should edit the canonical spec metadata and archive should preserve the audit trail.
  - Rationale: Otherwise the maintenance change could add requirements about metadata while leaving the actual `## Purpose` placeholders unchanged.
  - Resolution: Answered from repository evidence; no user question required.

- Question considered: Does this require a durable ADR?
  - Recommended answer: No.
  - Rationale: Replacing placeholder spec purpose prose is easy to reverse, not architecturally surprising, and does not involve a durable trade-off.
  - Resolution: No ADR candidate.

## Resolved Terms

None. Existing glossary terms are sufficient; no `CONTEXT.md` update required.

## Document Updates Applied

None during grill. Proposal and specs already encode metadata-only scope and verification intent.

## Document Updates Required Before Next Gate

None.

## ADR Candidates

None.

## Open Questions

None.
