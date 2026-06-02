## Context

The archived `add-x11-doctor-capability-detection` change synced expanded canonical specs but left the original bootstrap-created `## Purpose` placeholders in:

- `openspec/specs/doctor-cli/spec.md`
- `openspec/specs/x11-integration-contract/spec.md`

`CONSTITUTION.md` requires OpenSpec validation for changed artifacts, small visible checkpoints, no secret values, and no `.secrets.local.env` access unless an external system is needed. This maintenance change needs no external system or secrets.

The in-force ADR set (`0001`, `0003`, `0005`, `0006`, `0007`) governs workflow, context, grill/TDD, Claude review, and checkpoint behavior. None needs supersession because this change only updates spec metadata text.

No diagram is needed: there is no runtime boundary, integration path, deployment unit, or architecture interaction. The change is a two-file canonical spec metadata edit plus OpenSpec lifecycle artifacts.

## Goals / Non-Goals

**Goals:**

- Replace the two canonical `TBD` Purpose placeholders with accurate descriptions.
- Keep existing normative requirements and scenarios semantically unchanged.
- Provide deterministic verification that the placeholder strings are gone.
- Preserve auditability through OpenSpec proposal/spec/grill/design/review/ADR/test-plan/tasks artifacts.

**Non-Goals:**

- No Rust code changes.
- No `doctor --json` runtime behavior changes.
- No source-overlay or local target checkout changes.
- No durable ADR or `ARCHITECTURE.md` update.
- No broad rewrite of canonical specs beyond Purpose metadata.

## Decisions

1. **Edit canonical Purpose metadata directly during apply.**
   - Reason: OpenSpec delta specs model normative requirements and scenarios; prior archive evidence showed delta sync does not update canonical `## Purpose` prose directly.
   - Alternative rejected: rely on archive spec sync only. That could add requirements about Purpose text while leaving placeholders in place.

2. **Use concise, capability-specific purpose prose.**
   - `doctor-cli` should describe the CLI smoke-test and capability/readiness JSON contract.
   - `x11-integration-contract` should describe X11/EWMH backend identity, source-overlay compatibility, and integration constraints.
   - Avoid adding implementation details, current-machine observations, or backlog-specific history to Purpose text.

3. **Verify with text checks and OpenSpec validation.**
   - Required checks: `openspec validate cleanup-spec-purpose-metadata --type change --json`, `openspec validate --all --strict`, and a grep/text check that neither canonical Purpose section contains `TBD` or the old bootstrap archive phrase.
   - Rust `make fmt/check/test` is not required because no Rust files should change; if a Rust file changes unexpectedly, stop and run the full Rust verification set.

## Risks / Trade-offs

- **Risk:** The delta specs add metadata requirements that remain in canonical specs after archive.
  - Mitigation: Keep those requirements narrowly scoped and useful as future regression checks for spec metadata quality.
- **Risk:** Purpose prose could become too implementation-specific.
  - Mitigation: Keep prose stable and high-level; leave behavior details in requirements/scenarios.
- **Trade-off:** Direct canonical edits during apply are less typical than archive-only spec sync, but they are necessary for fields not controlled by delta sync.

## Migration Plan

- Apply: edit only the two canonical spec `## Purpose` sections.
- Verify: run OpenSpec validation and placeholder absence checks.
- Archive: archive the maintenance change after verification. If archive sync re-applies metadata requirements, ensure canonical Purpose text still remains non-placeholder afterward.
- Rollback: revert the apply/archive commits; no runtime data migration exists.

## Open Questions

None.
