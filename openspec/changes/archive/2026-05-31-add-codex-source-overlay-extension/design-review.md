## Context Read

- Change artifacts: `proposal.md`, `specs/codex-source-overlay-extension/spec.md`, `grill.md`, and `design.md` for `add-codex-source-overlay-extension`.
- Project rules/context: `CONSTITUTION.md`, `CONTEXT.md` including the new `Overlay drift` term, `ARCHITECTURE.md`, `adr/README.md`, `adr/0008-adopt-x11-root-coordinate-model.md`, and backlog 00/06.
- Current project code/docs and existing script style: `scripts/install-codex-plugin.sh`, `scripts/uninstall-codex-plugin.sh`, `scripts/install-overlay`, existing Rust integration tests, `README.md`, and `docs/integration-contract.md`.
- Target checkout context at `1a6f343ee7ce597019a4c573259c2a9838376874`: registry/backend/test/diagnostics anchors named in `design.md`.

## Design Summary Reviewed

The design uses public shell wrappers over a Python marker-block overlay engine. It validates target structure, installs one generated `x11_ewmh.rs` backend, patches target registry/module/diagnostic anchors with owned markers, reports clean/applied/drifted states, and returns the real target checkout to clean state after smoke verification. The target-facing backend is a late fallback that preserves existing `WindowInfo` and stock tool surfaces.

## Question Loop

### Q1: Should install automatically repair drifted targets?

Recommended answer: No. Install should be idempotent for clean/applied states, but it should refuse drift by default.

Rationale: Drift means the target or owned overlay content no longer matches assumptions. Blind repair could overwrite a moving upstream target or hide a partial manual edit. Because the user asked for autonomous work but not destructive target repair, refusing drift is safer and still compatible with status/uninstall workflows.

Resolution: Applied. Updated `design.md` failure handling so `install` refuses drift; `status` reports drift and `uninstall` removes only owned content when safe.

### Q2: Does the generated backend need a new dependency in the target crate?

Recommended answer: No. Use existing target dependencies and `std::process::Command` with `anyhow`, matching existing i3/KWin backend style.

Rationale: Adding a dependency such as `x11rb` would be harder to upstream and require a new license/dependency decision. The backlog only needs a reversible scaffold and contract checks; native X11 can be a future change if shell commands are insufficient.

Resolution: Answered from design and target code. No new target dependencies.

### Q3: Should diagnostics schema grow new public fields for X11?

Recommended answer: No in this change. Patch existing checks and backend map/registry facts narrowly; do not redesign `WindowingReport` or `ReadinessReport`.

Rationale: The overlay must remain reversible and source-compatible. The existing target already has a `backends` map, capability vectors, and readiness fields. New public fields would turn this into an upstream API change rather than a removable overlay.

Resolution: Design keeps existing report vocabulary and patches strict portal method detection narrowly.

### Q4: Are fake-target tests enough before real-target smoke?

Recommended answer: Fake-target tests are required first but not sufficient. Real-target status/apply/target-cargo-test/uninstall/clean evidence is required when the configured checkout is available.

Rationale: Marker insertion can pass on fixtures but fail against current target anchors, and generated Rust can compile-fail due to upstream shape changes. Real-target smoke is part of acceptance, but it must be reversible.

Resolution: Captured in test plan requirements and design verification plan.

## Resolved Terms and Context Updates

No new glossary updates beyond `Overlay drift` were needed. Existing terms `Source overlay`, `x11-ewmh`, `Focus verification`, and `X11 root coordinates` cover the technical vocabulary.

## Document Updates Applied

- Updated `design.md` to make drift refusal explicit for install.

## Document Updates Required Before Next Gate

None.

## ADR Candidates

No durable ADR is required. The design decisions are reversible script mechanics and target patch boundaries within the already accepted source-overlay architecture. A durable ADR should be reconsidered only if the project decides to keep a permanent target branch, introduce a target public API change, or add a native X11 dependency such as `x11rb`.

## Open Questions

None.
