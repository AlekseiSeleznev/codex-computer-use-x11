## Context Read

- `CONSTITUTION.md` — Rust/Cargo root package constraints, root `Makefile` verification commands, `CODEX_DESKTOP_LINUX_FULL_PATH`, secret-handling rules, and verification policy.
- `CONTEXT.md` — glossary terms for `x11-ewmh`, Standalone plugin, Source overlay, OpenSpec, grill gate, design-review gate, and TDD slice.
- `ARCHITECTURE.md` and `adr/README.md` — intent-driven lifecycle, mandatory gate order, checkpoint discipline, Claude artifact-review role, TDD discipline, durable ADR rules, and current in-force/superseded ADR index.
- `openspec/changes/bootstrap-codex-computer-use-x11/proposal.md` — stage-01 bootstrap scope, research refresh, target checkout findings, license posture, delivery posture, and impact.
- `openspec/changes/bootstrap-codex-computer-use-x11/specs/project-bootstrap/spec.md` — root Rust package, Cargo/Makefile verification surface, machine-local target path, and delivery posture requirements.
- `openspec/changes/bootstrap-codex-computer-use-x11/specs/doctor-cli/spec.md` — binary name, `doctor --json` success-path report shape, standalone doctor boundary, and non-invasive behavior requirements.
- `openspec/changes/bootstrap-codex-computer-use-x11/specs/x11-integration-contract/spec.md` — `x11-ewmh`, fallback order, upstream `WindowInfo`, sidecar diagnostics, X11 id normalization, and command-seam policy.
- `openspec/changes/bootstrap-codex-computer-use-x11/grill.md` — pre-design resolved decisions, glossary updates, and design inputs.
- `openspec/changes/bootstrap-codex-computer-use-x11/design.md` — current design after post-review fixes.
- `openspec/changes/bootstrap-codex-computer-use-x11/reviews/design-claude-review.json` — auxiliary Claude design review; final persisted verdict `pass`, no `mustFix`, and minor `shouldFix`/question items that were resolved by updating `design.md` before this artifact.
- Project docs available locally: `docs/intent-driven-lifecycle.md` and `docs/intent-driven-update-safety.md`.
- Relevant code: no Rust source exists yet for this change; implementation is still gated by ADR, test-plan, and tasks.

## Design Summary

- The design creates one root Rust 2021 package named `codex-computer-use-x11`, with `src/lib.rs` exposing `doctor` and `x11_id`, `src/main.rs` acting only as CLI adapter, and root `Makefile` wrappers over Cargo.
- `doctor --json` is a standalone, non-invasive smoke-test surface that emits compact valid JSON with project identity, readiness, capabilities, and stable stage-01 bootstrap check names; it does not call live X11 tools or patch the target checkout.
- The X11 id normalizer is a pure parser from equivalent hexadecimal X11 id strings to canonical `u64`, with inline unit tests in `src/x11_id.rs`.
- Future X11/EWMH integration keeps upstream `WindowInfo` as the primary model, uses `x11-ewmh` as backend id, keeps X11-only diagnostics in a sidecar/report by default, and documents a non-implemented `WindowObservationMeta` sketch in the normative integration contract.
- Apply remains blocked until ADR, test-plan, and tasks are complete; apply must use vertical RED/GREEN/REFACTOR slices and verify both automated commands and manual documentation deliverables.

## Question Loop

No user-facing questions were asked.

All material uncertainties were answerable from repository context, existing OpenSpec artifacts, the pre-design grill resolutions, the committed design, and the user's standing instruction to decide the best answer and continue when a decision does not require user participation.

Resolved during this design-review gate:

1. **Should compact doctor JSON be a deliberate contract?**
   - **Recommended answer:** Yes. Keep compact JSON plus a trailing newline as the CLI output, but tests should parse JSON and ignore trailing whitespace rather than compare formatting.
   - **Rationale:** The spec requires machine-readable JSON; compact output is stable for tools, and manual inspection can use external pretty-printers.
   - **Resolution:** Applied to `design.md` Decision 3.

2. **Should CLI error paths have RED evidence?**
   - **Recommended answer:** Yes, because the design commits to those behaviors even though the spec gates the success path.
   - **Rationale:** TDD discipline should not allow implemented behavior to appear without a RED signal when the design makes it observable.
   - **Resolution:** Applied to `design.md` Decision 6 by adding error-path assertions for unknown commands and `doctor` without `--json`.

3. **Are bootstrap check names stable or placeholders?**
   - **Recommended answer:** Stable for stage-01 apply.
   - **Rationale:** Tests now assert `no-live-x11-probes`; allowing silent rename would break the design's own verification contract.
   - **Resolution:** Applied to `design.md` Decision 4.

4. **Where should normalizer unit tests live?**
   - **Recommended answer:** Inline in `src/x11_id.rs` under `#[cfg(test)]`.
   - **Rationale:** The parser is a pure module; inline unit tests preserve the first RED/GREEN slice without needing a built binary.
   - **Resolution:** Applied to `design.md` Decision 6.

5. **Which docs are normative for future integration details?**
   - **Recommended answer:** `docs/integration-contract.md` is normative; `README.md` is a summary and should link to it.
   - **Rationale:** Future contributors need one durable location for sidecar/source-overlay details after OpenSpec design artifacts are archived.
   - **Resolution:** Applied to `design.md` Decision 7 and Migration Plan. `test-plan.md` and `tasks.md` must carry explicit non-automated check steps for README MSRV-posture text and the integration-contract `WindowObservationMeta`/sidecar content, with the same gate weight as cargo/make verification.

## Design Findings

- **No architecture conflict found.** The design respects `CONSTITUTION.md` required technologies: Rust 2021, root package by default, root `Makefile` wrappers, local target path via `CODEX_DESKTOP_LINUX_FULL_PATH`, and no secret access.
- **No OpenSpec/spec conflict found.** The design covers all three capability specs and keeps implementation deferred until the required ADR, test-plan, and tasks gates are complete.
- **No glossary conflict found.** The design consistently uses `x11-ewmh`, Standalone plugin, and Source overlay as defined in `CONTEXT.md`.
- **No external-system/secret risk found.** Stage 01 needs no `.secrets.local.env` access and does not call external systems or patch the target checkout.
- **Claude design review findings were actionable but not user-blocking.** The final persisted Claude design review had verdict `pass`, no `mustFix`, three `shouldFix` items, and two questions. They were resolved by making the design explicit about stable bootstrap check names, inline normalizer tests, manual documentation verification, JSON newline semantics, and README MSRV-posture verification.
- **Manual verification must be preserved.** The future `docs/integration-contract.md` content and README MSRV-posture note are not covered by automated tests, so `test-plan.md` and `tasks.md` must carry explicit non-automated checks with the same gate weight as `cargo test`, `make fmt`, `make check`, and `make test`.
- **Bootstrap readiness is deliberately scoped.** `readiness.ok = true` means the stage-01 smoke-test surface is internally ready; future live-probe work owns flipping readiness or adding blockers when real backend checks are introduced.

## Document Updates Applied

- `openspec/changes/bootstrap-codex-computer-use-x11/design.md` — clarified that compact doctor JSON's trailing newline is not semantic for tests.
- `openspec/changes/bootstrap-codex-computer-use-x11/design.md` — declared the stage-01 bootstrap check names `bootstrap-project`, `backend-identity`, and `no-live-x11-probes` as stable apply identifiers.
- `openspec/changes/bootstrap-codex-computer-use-x11/design.md` — specified inline `#[cfg(test)]` unit tests in `src/x11_id.rs` for the X11 id normalizer.
- `openspec/changes/bootstrap-codex-computer-use-x11/design.md` — required CLI error-path RED tests because the design commits to those behaviors.
- `openspec/changes/bootstrap-codex-computer-use-x11/design.md` — made `README.md` MSRV-posture text and `docs/integration-contract.md` `WindowObservationMeta`/sidecar content required manual verification items.

## Document Updates Required Before Next Gate

None.

The manual verification items above must be carried into `test-plan.md` and `tasks.md` as explicit non-automated checks, and the test-plan must note that the Makefile RED step is wiring verification while actual `make fmt`, `make check`, and `make test` runs are the meaningful post-GREEN evidence. No further proposal/spec/grill/design edits are required before ADR planning.

## ADR Candidates

None for stage 01.

Rationale: the design records future source-overlay and sidecar defaults, but stage 01 implements only a standalone bootstrap package, doctor JSON surface, parser, docs, and verification wrappers. The root package layout, no-MSRV posture, compact JSON, and sidecar documentation are reversible within this repository and already constrained by OpenSpec artifacts. The per-change `adr.md` should still record that no new durable ADR is needed unless ADR review finds a broader architecture decision.

## Claude Design-Review Disposition

The rerun of Claude review for this `design-review` stage returned `pass`, no `mustFix`, two non-blocking `shouldFix` items, and two questions. No user participation is required.

- `capabilities.planned = ["x11-ewmh-windowing"]` is an exact stage-01 design-owned label for apply, even though the spec only requires `planned` to be non-empty. Future planned capability names can change only through later design/spec updates.
- `backend = "x11-ewmh"` is identity, not a claim that a live X11 backend is implemented or ready. `readiness.ok = true` is scoped to the bootstrap smoke-test surface; future live-probe work owns changing readiness/blocker semantics.
- The `no-live-x11-probes` check name remains intentional. Its apply detail should say `stage 01 performs no live X11 probes or external command execution` so the name and broader non-invasive guarantee are both clear.
- `test-plan.md` and `tasks.md` must carry the manual README/integration-contract checks with the same gate weight as cargo/make checks, as already required above.

## Open Questions

None.
