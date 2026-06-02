## Context Read

- `CONSTITUTION.md` — project rules, Rust/Cargo and Makefile constraints, `CODEX_DESKTOP_LINUX_FULL_PATH`, secret policy, verification rules.
- `CONTEXT.md` — glossary, including newly resolved `x11-ewmh`, Standalone plugin, and Source overlay terms.
- `ARCHITECTURE.md` — intent-driven lifecycle, checkpoint/review model, source-of-truth boundaries, in-force ADR list.
- `adr/README.md` — durable ADR rules and current/superseded ADR index.
- `docs/intent-driven-lifecycle.md` and `docs/intent-driven-update-safety.md` — grill gate behavior, optional Claude review, TDD gate, update safety.
- `openspec/changes/bootstrap-codex-computer-use-x11/proposal.md` — change intent, research refresh, scope, risks, and impact.
- `openspec/changes/bootstrap-codex-computer-use-x11/specs/project-bootstrap/spec.md` — root Rust/Makefile/path/documentation requirements.
- `openspec/changes/bootstrap-codex-computer-use-x11/specs/doctor-cli/spec.md` — CLI name, `doctor --json` shape, non-invasive behavior, upstream doctor boundary.
- `openspec/changes/bootstrap-codex-computer-use-x11/specs/x11-integration-contract/spec.md` — `x11-ewmh`, upstream `WindowInfo`, sidecar diagnostics, id normalization, standalone/source-overlay command seams.
- `openspec/changes/bootstrap-codex-computer-use-x11/reviews/proposal-claude-review.json` and `reviews/specs-claude-review.json` — auxiliary reviewer questions and should-fix items.
- Target repo files inspected during proposal/specs research: `computer-use-linux/Cargo.toml`, `src/main.rs`, `src/server.rs`, `src/diagnostics.rs`, `src/atspi_tree.rs`, `src/screenshot.rs`, `src/windowing/types.rs`, `src/windowing/registry.rs`, `src/windowing/target.rs`, and `src/windowing/backends/*` under `${CODEX_DESKTOP_LINUX_FULL_PATH}` / the current development-machine default.

## Plan Summary

- Bootstrap creates a standalone root-level Rust package named `codex-computer-use-x11` with `make fmt`, `make check`, and `make test` wrappers over Cargo.
- Bootstrap CLI exposes `codex-computer-use-x11 doctor --json` as a non-invasive smoke-test surface, not a live X11 probe or strict upstream `doctor_report()` clone.
- The X11 integration contract fixes the canonical backend label as `x11-ewmh`, keeps upstream `WindowInfo` as the primary model, and keeps X11-only provenance/reliability in a sidecar/report by default.
- The first TDD tracer bullet is small: X11 id normalization, `doctor --json`, and verification command surface; no `wmctrl`, `xprop`, `xdotool`, Cinnamon extension, or target repo patch is in scope for stage 01.
- Source-overlay decisions remain constrained but not implemented: future `x11-ewmh` is a late fallback after existing desktop-specific backends and should follow target repo command-wrapper/parser-test style unless a design/ADR exception is accepted.

## Question Loop

### 1. Should `doctor --json` be a strict subset of the target repo `doctor_report()`?

- **Recommended answer:** No. Treat it as a standalone bootstrap smoke-test surface that loosely mirrors upstream readiness concepts but is not structurally coupled to upstream `doctor_report()`.
- **Rationale:** Stage 01 must stay portable and small. The target `doctor_report()` is richer and tied to portal/input/window/accessibility checks that this bootstrap explicitly does not implement yet.
- **Resolution:** Resolved from proposal/spec research and applied to `proposal.md` and `doctor-cli/spec.md`. Design may later choose a tighter relationship, but only explicitly.

### 2. Where should the initial Rust package live?

- **Recommended answer:** At the repository root with root `Cargo.toml` and root `src/`.
- **Rationale:** The change is a standalone bootstrap, not a multi-crate architecture exercise. A root package gives the CLI, tests, and future MCP server an obvious entry point; subcrates can be added later if design evidence requires them.
- **Resolution:** Resolved from user direction to decide independently and Claude review feedback. Applied to `CONSTITUTION.md`, `proposal.md`, and `project-bootstrap/spec.md`.

### 3. How portable should the local target checkout path be?

- **Recommended answer:** Use `CODEX_DESKTOP_LINUX_FULL_PATH` as the durable name; concrete local paths are development-machine defaults only.
- **Rationale:** The path is not secret, but hard-coding it as a portable requirement would mislead other machines and future contributors.
- **Resolution:** Applied to `CONSTITUTION.md`, `proposal.md`, `project-bootstrap/spec.md`, and `doctor-cli/spec.md`.

### 4. Should bootstrap specs pin exact planned capability names?

- **Recommended answer:** Pin only stable implemented capability identity (`doctor-json`) and require `planned` to be non-empty. Leave exact future planned capability names design-owned.
- **Rationale:** Bootstrap needs enough observable behavior for tests without freezing future naming before design validates CLI/MCP/source-overlay boundaries.
- **Resolution:** Applied to `doctor-cli/spec.md`.

### 5. Should standalone external-command tests use a specific seam style now?

- **Recommended answer:** Require the property, not the exact abstraction: standalone tests that exercise external command behavior must use a command-runner seam or fake `PATH`, while the exact seam style remains design-owned.
- **Rationale:** Testability is non-negotiable, but choosing trait object vs function pointer vs fake PATH is a design detail best resolved with code shape in view.
- **Resolution:** Applied to `proposal.md` and `x11-integration-contract/spec.md`.

### 6. Should source-overlay command code get a dependency-injection runner by default?

- **Recommended answer:** No. Default to the target repo's thin `Command::new(...)` wrapper plus pure parser/normalizer fixture-test style. Require an explicit design/ADR exception before adding a DI runner to the target repo.
- **Rationale:** The target repo already has established style; importing a standalone-style runner into source overlay would be a wider architectural change.
- **Resolution:** Applied to `x11-integration-contract/spec.md`.

### 7. Is the sidecar/report shape required before design?

- **Recommended answer:** No. The pre-design contract only needs the boundary: do not extend upstream `WindowInfo` by default; put X11-only provenance/reliability into a sidecar/report. The concrete sidecar shape belongs in `design.md`.
- **Rationale:** The sidecar shape depends on CLI report structure, future overlay mapping, and what diagnostics are actually needed. Freezing it in specs would over-design stage 01.
- **Resolution:** No spec update required. Carry as a design input.

### 8. Should internal failure-path behavior and Rust MSRV block design?

- **Recommended answer:** No. Success-path `doctor --json` behavior, stderr-on-success, Cargo/Makefile wrappers, and Rust 2021 are specified. Internal error formatting and MSRV can be addressed in design/test-plan if they affect implementation.
- **Rationale:** Stage 01 tracer bullets do not require a full error taxonomy, and current project rules already constrain Rust/Cargo enough for bootstrap planning.
- **Resolution:** No OpenSpec blocker. Design should note whether to use current stable Rust or an explicit MSRV.

No user-facing questions were asked because each material uncertainty was answerable from repository context, proposal/spec content, target code inspection, or the user's instruction to choose the best answer and continue.

## Resolved Terms

- `x11-ewmh` — canonical backend label for the generic X11/EWMH path, distinct from Cinnamon validation and `client_type`.
- Standalone plugin — separate CLI/MCP validation delivery path before target repo adaptation.
- Source overlay — future delivery path adapting this project's code/concepts into the local Codex Desktop Linux target checkout.

`CONTEXT.md` was updated inline with these glossary terms.

## Document Updates Applied

- `CONTEXT.md` — added glossary entries for `x11-ewmh`, Standalone plugin, and Source overlay.
- `proposal.md` — clarified the standalone doctor report relationship to upstream `doctor_report()`, command-seam ownership, and `CODEX_DESKTOP_LINUX_FULL_PATH` path framing.
- `specs/doctor-cli/spec.md` — added `doctor-json` implemented capability, non-empty design-owned planned capabilities, and standalone doctor report boundary from upstream `doctor_report()`.
- `specs/project-bootstrap/spec.md` — clarified root `[package]` requirement and optional workspace table constraints.
- `specs/x11-integration-contract/spec.md` — split source-overlay command style/default vs DI-runner exception scenarios and clarified standalone command testing seam requirements.

## Document Updates Required Before Next Gate

None. Design should consume the recorded design inputs above, especially sidecar/report shape, exact command seam style, bootstrap doctor check names/extensions, CLI/MCP packaging, source-overlay file layout, and whether to state an explicit Rust MSRV.

## ADR Candidates

- **Potential ADR: Standalone/plugin vs source-overlay architecture.** Candidate only if `design.md` finds materially divergent architecture, data models, or command-execution seams between the standalone project and the target repo overlay. Not required yet.
- **Potential ADR: Expanding upstream `WindowInfo`.** Candidate only if design rejects the sidecar/report default and proposes adding X11-specific diagnostics to upstream `WindowInfo`. Not required yet.
- **Potential ADR: Source-overlay dependency-injection runner.** Candidate only if design proposes a DI runner in the target repo instead of thin `Command::new(...)` wrappers plus pure parser tests. Not required yet.

## Open Questions

None.
