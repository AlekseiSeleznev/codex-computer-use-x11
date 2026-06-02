## Context Read

- `AGENTS.md`, `CONSTITUTION.md`, `CONTEXT.md`, `ARCHITECTURE.md`, `adr/README.md`, `adr/0008-adopt-x11-root-coordinate-model.md`, `adr/0009-adopt-final-cinnamon-x11-v1-dod-baseline.md`, `adr/0010-adopt-x11-provider-takeover-shim.md`.
- Proposal and delta specs in `openspec/changes/fix-app-state-screenshot-evidence-safety/`.
- Existing specs: `openspec/specs/x11-get-app-state-integration/spec.md`, `openspec/specs/codex-x11-e2e-test-harness/spec.md`, `openspec/specs/x11-packaging-docs-upstreaming/spec.md`, `openspec/specs/x11-screenshot-coordinate-model/spec.md`, `openspec/specs/x11-atspi-window-correlation/spec.md`, `openspec/specs/x11-targeted-input-safety/spec.md`.
- Evidence summaries from `target/e2e-logs/real-live-full-retest/20260601T162050Z/get-app-state.json`, `get-app-state-no-screenshot.json`, `accessibility-tree.json`, and `evidence.json` without printing screenshot payloads.
- Relevant code: `src/app_state.rs`, `src/cli.rs`, `src/mcp.rs`, `scripts/e2e/codex-x11-e2e.py`, `scripts/e2e/fixtures/gtk_atspi_fixture.py`, `scripts/e2e/fixtures/tk_text_pointer_fixture.py`, `tests/get_app_state_cli.rs`, `tests/e2e_harness_scripts.rs`.
- Docs: `docs/e2e-harness.md`, `docs/troubleshooting.md`, `docs/release-checklist.md`, `docs/upstreaming.md`.

## Plan Summary

- The retest's primary defect is in app-state screenshot serialization: `src/app_state.rs` currently reads the temporary PNG and serializes it as `ScreenshotCapture.data_url`, so default JSON can contain megabytes of inline screenshot data.
- The desired behavior is path-only/sanitized screenshot evidence by default, while preserving `--no-screenshot` and layer-degraded app state when screenshot capture fails.
- The fixture runner already exists in `scripts/e2e/codex-x11-e2e.py`, but it currently uses titles/classes containing `codex`/`Codex`, which the new requirement explicitly wants to avoid when filters exclude project-owned or overlay-looking windows.
- Scope remains the supported Cinnamon/X11 `x11-ewmh` baseline; Wayland and portal-required runtime paths remain out of scope.
- No target checkout, bundled `computer-use`, external credentials, or `.secrets.local.env` are needed.

## Question Loop

### Q1: Should default `get-app-state --json` still capture a screenshot, or should screenshot capture require `--screenshot-output`?

**Recommended answer:** Keep screenshot capture enabled by default for compatibility with the existing app-state capability, but write it to a generated safe PNG artifact path when no `--screenshot-output` is supplied.

**Rationale:** Existing specs and MCP calls assume `include_screenshot` defaults to true. Turning it off by default would be a larger behavior change than required. The safety defect is inline serialization, not capture itself. A generated path under `target/e2e-logs/app-state/` or a documented temp evidence directory keeps the layer observable without embedding pixels.

**Resolution:** Accepted from repository context. Specs allow caller-provided or generated path and require path-oriented JSON by default.

### Q2: Should inline screenshot output be removed entirely or retained behind an explicit unsafe opt-in?

**Recommended answer:** Retain only if implementation cost is low, behind explicit `--inline-screenshot` / MCP `inline_screenshot=true`; otherwise remove from public default paths. In either case, industrial evidence must not use it.

**Rationale:** The user listed inline mode as an option to evaluate, not a requirement. Retaining explicit opt-in reduces compatibility risk, but the safe acceptance boundary is the default no-inline behavior.

**Resolution:** Specs require explicit unsafe opt-in only if retained; design will prefer the smallest implementation that keeps default safe.

### Q3: What should fixture titles/classes be if existing filters may exclude project-owned titles containing `Codex`?

**Recommended answer:** Use neutral, run-scoped fixture identity such as title `x11-safe-fixture-<role>-<run-id>` and class `X11SafeFixture<Role>`, and record controlled ownership in metadata rather than relying on a `Codex` substring.

**Rationale:** Existing runner titles are `codex-x11-fixture-...` and classes are `CodexX11Fixture...`. The retest instruction explicitly warns against project-owned/overlay-looking titles such as titles containing `Codex`. Neutral names avoid accidental exclusion while metadata still proves fixture ownership.

**Resolution:** Design/tasks must rename runner fixture identity and update tests/docs accordingly.

## Resolved Terms

- Existing glossary terms `App state`, `Layer-degraded app state`, `Controlled fixture`, `E2E harness`, and `Capability matrix evidence` are sufficient.
- No `CONTEXT.md` glossary update is required for this change because the new phrase “path-only screenshot evidence” is an evidence-format detail rather than a durable domain term.

## Document Updates Applied

- Proposal and specs already encode path-only default app-state screenshot evidence, optional explicit inline opt-in, screenshot-crop unchanged regression behavior, controlled fixture runner metadata/lifecycle, and docs updates.

## Document Updates Required Before Next Gate

None.

## ADR Candidates

- No new durable top-level ADR is required at this point. The change refines ADR 0008's screenshot-by-path posture and ADR 0009's pass/degraded evidence safety within the existing Cinnamon/X11 baseline rather than changing architecture direction.
- Revisit during `adr.md` if design chooses a hard-to-reverse generated evidence directory contract or removes inline compatibility entirely as a durable API decision.

## Open Questions

None.
