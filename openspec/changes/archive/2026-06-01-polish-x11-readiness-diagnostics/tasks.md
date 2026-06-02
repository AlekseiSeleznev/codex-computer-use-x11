## 1. Doctor readiness and diagnostics

- [x] 1.1 Add RED tests for additive doctor readiness categories, `readiness.ok` aggregation, and bootstrap field compatibility.
- [x] 1.2 Implement stable doctor JSON fields for blockers, degraded-but-acceptable X11 limitations, optional enrichments, unsupported/out-of-scope paths, and recommendations.
- [x] 1.3 Add RED tests for AT-SPI bus/tree/no-match/ambiguous/fixture-pass diagnostic states and private path redaction.
- [x] 1.4 Implement AT-SPI diagnostic taxonomy and Cinnamon/X11 actionable recommendations without making AT-SPI a baseline blocker.
- [x] 1.5 Validate `doctor --json` as machine-readable JSON and preserve no-secret/no-private-path reporting.

## 2. Evidence schema and matrix validator

- [x] 2.1 Add matrix validator fixture tests for canonical `reason_category` values and missing category failures.
- [x] 2.2 Implement reason-category normalization and validation for pass/degraded/fail rows.
- [x] 2.3 Make metadata-only live smoke classify fixture-dependent skipped rows as `missing_fixture_setup` with an unsafe-real-app warning.
- [x] 2.4 Ensure unsupported Wayland and portal-required runtime paths are reported as out-of-scope diagnostics rather than X11 blockers.

## 3. Screenshot and app-state evidence polish

- [x] 3.1 Decide within the accepted design whether to add a fake screenshot provider fixture or keep fake screenshot degraded as an expected fake-fixture limitation.
- [x] 3.2 Add RED/GREEN tests for the selected fake screenshot behavior.
- [x] 3.3 Preserve strict real screenshot-crop output integrity tests: output path, file existence, readability, dimensions/metadata, and no inline data URLs in summaries.
- [x] 3.4 Update app-state evidence summaries to keep usable metadata visible while classifying screenshot/AT-SPI layer degradation with reason categories.

## 4. Controlled live fixture safety and cleanup

- [x] 4.1 Add tests or fake-live scenarios proving controlled fixture uniqueness before input, pointer, screenshot, app-state, target, and overlay operations.
- [x] 4.2 Reject ambiguous or missing controlled fixture targets instead of falling back to real user applications.
- [x] 4.3 Record cleanup evidence for overlay hiding, target release, fixture process shutdown, and stale target context.
- [x] 4.4 Treat cleanup failures as explicit degraded/fail rows with concrete evidence paths and reasons.

## 5. Documentation and retest guidance

- [x] 5.1 Update README/docs/troubleshooting to define PASS, DEGRADED, FAIL, reason categories, and doctor readiness interpretation.
- [x] 5.2 Document why Wayland and portal-required runtime paths are out of scope for this X11-only production baseline.
- [x] 5.3 Document safe full retest commands and evidence requirements for fake, fake-live, metadata-only live, controlled live fixture, doctor JSON, and matrix validation runs.
- [x] 5.4 Ensure docs warn that live input/pointer/overlay must target controlled fixtures only and never real user apps.

## 6. Verification checkpoint

- [x] 6.1 Run `make fmt`, `make check`, and `make test` or record exact blockers.
- [x] 6.2 Run fake plugin smoke, fake-live industrial fixture smoke, and matrix validation.
- [x] 6.3 Run controlled live fixture smoke on Cinnamon/X11 when available or record precise environment limitations.
- [x] 6.4 Run `openspec validate polish-x11-readiness-diagnostics --type change --strict` and `openspec validate --all --strict`.
- [x] 6.5 Confirm `git status --short` contains no unrelated or secret files before verify/archive.
