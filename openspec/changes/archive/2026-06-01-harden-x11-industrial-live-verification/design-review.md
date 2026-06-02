## Context Read

- Planning artifacts: `proposal.md`, all delta specs under `specs/`, `grill.md`, and `design.md` for `harden-x11-industrial-live-verification`.
- Project context: `AGENTS.md`, `CONSTITUTION.md`, `CONTEXT.md`, `ARCHITECTURE.md`, and `adr/README.md`.
- Relevant in-force ADRs: `0005` (grill/TDD gates), `0007` (automatic safe checkpoints and Claude session controls), `0008` (X11 root-coordinate model), `0009` (Cinnamon/X11 v1 DoD and safe degraded evidence), and `0010` (localized provider takeover with standalone identity preserved).
- Retest evidence: `target/e2e-logs/full-x11-retest-20260601T123839Z/report.md`, `plugin-live/**/evidence.json`, screenshot crop logs, fixture logs, GTK fixture readiness, and overlay-enabled log.
- Relevant implementation/docs: `src/coordinates.rs`, `src/cli.rs`, `scripts/e2e/codex-plugin-smoke.sh`, `scripts/e2e/codex-x11-e2e.py`, `docs/e2e-harness.md`, `docs/troubleshooting.md`, `docs/release-checklist.md`, and `docs/final-architecture-dod.md`.

## Design Summary

- The design adds deterministic screenshot crop path resolution and post-provider PNG verification so ambiguous provider success cannot leak into `success=true`.
- The design adds a controlled fixture manager for industrial live plugin checks while preserving fake and metadata smoke evidence as separate lower-risk layers.
- The design keeps live desktop mutation fixture-only and does not use user applications as fallback targets.
- The design extends evidence classification with canonical lower-case statuses and reason categories so missing fixture orchestration cannot masquerade as an environment limitation.
- The design avoids architecture churn: no global plugin rename, no backend rewrite, no RemoteDesktop portal requirement, and no weakening of input or AT-SPI safety rules.

## Question Loop

### Question 1: Could stricter industrial validation break existing fake and metadata-only smoke consumers?

- **Recommended answer:** Add an explicit industrial profile or acceptance mode while preserving existing validation behavior for legacy/fake evidence.
- **Rationale:** Existing `validate-matrix` accepts `pass`/`degraded` rows and is part of the current release checklist. Industrial checks should fail on missing fixtures, but that must not invalidate old deterministic fake evidence or metadata freshness smoke unless the caller asks for industrial acceptance.
- **Resolution:** Resolved from design and docs. The design already recommends `validate-matrix --industrial` or equivalent profile. Tasks must preserve backward compatibility for non-industrial validation.

### Question 2: Is resolving relative screenshot paths against cwd safe enough?

- **Recommended answer:** Yes, as long as the resolved absolute path is reported, parent directories are validated before the provider call, the provider receives the absolute path, and success requires postflight PNG verification.
- **Rationale:** The retest failure came from ambiguous relative provider handling. Cwd resolution removes provider ambiguity while preserving CLI ergonomics. Postflight verification prevents false success even if the provider reports misleading status.
- **Resolution:** Resolved by proposal/spec/design updates. No user question required.

### Question 3: Should environment limitation include missing fixture dependencies?

- **Recommended answer:** Only after the harness attempted fixture orchestration and recorded a concrete system/toolkit limitation. A skipped or unimplemented fixture path remains `missing_fixture_setup` and blocks industrial acceptance.
- **Rationale:** The live plugin smoke currently marks fixture-dependent rows degraded because it does not orchestrate fixtures. Industrial acceptance needs to distinguish "desktop cannot support this fixture" from "harness never tried".
- **Resolution:** Resolved in specs and design; tasks must add validator fixtures for all three categories.

### Question 4: Is a new durable ADR required for an industrial evidence profile?

- **Recommended answer:** No, not now. Per-change ADR is enough unless implementation changes the final v1 architecture claim or makes industrial profile a project-wide replacement for all existing release gates.
- **Rationale:** ADR 0009 already requires explicit pass/degraded evidence and rejects silent omissions. The new profile is a stricter verification implementation inside that accepted posture.
- **Resolution:** Resolved; record no durable ADR required in `adr.md`, with a future ADR trigger if release policy is broadened.

## Design Findings

- **Finding: Status casing needed normalization.** Existing harness evidence uses lower-case `pass`/`degraded`. The initial delta spec used uppercase human labels. The spec was updated to require canonical machine JSON statuses `pass`, `degraded`, and `fail`; human reports may still display uppercase.
- **Finding: Industrial profile must be backward compatible.** Existing fake mode and metadata live smoke are useful and should remain valid under default validation. Industrial acceptance should be explicit in CLI, documentation, or release checklist.
- **Finding: Fixture ownership is the critical safety control.** Tool-level focus/bounds verification remains necessary but is not sufficient for live E2E safety; harness-level allowlisting prevents accidental targeting of real user applications before tool calls are made.
- **Finding: Screenshot postflight must not trust provider output alone.** The provider status, file existence, non-empty size, readability, and PNG signature are all required because retest observed false provider output with no file.
- **Finding: GTK AT-SPI dependency reporting must be precise.** A missing PyGObject/GTK bridge is acceptable degraded evidence only when the harness attempted the GTK fixture and records the missing dependency; Tk no-match remains separate.

## Document Updates Applied

- Updated `specs/codex-x11-e2e-test-harness/spec.md` to normalize industrial matrix statuses to canonical lower-case JSON values `pass`, `degraded`, and `fail`.

## Document Updates Required Before Next Gate

None.

## ADR Candidates

- No durable top-level ADR is required for this planning state.
- Future durable ADR candidate only if apply changes the project-wide final DoD/release architecture so that the industrial profile replaces existing fake/metadata gates rather than extending them.

## Open Questions

None.
