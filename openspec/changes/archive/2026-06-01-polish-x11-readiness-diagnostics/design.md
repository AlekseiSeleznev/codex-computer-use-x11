## Context

The current repository has no active OpenSpec changes after archiving previous work, and `openspec validate --all --strict` passed for 18 canonical specs before this change was created. Recent installed-plugin retest evidence showed the Cinnamon/X11 baseline is usable, industrial fake-live controlled-fixture verification passes, and remaining degradations are expected or under-classified: fake screenshot lacks fake `gdbus`, metadata-only live smoke lacks controlled fixture setup, and current desktop doctor sees AT-SPI bus reachability without tree extraction plus unavailable/incomplete RemoteDesktop portal.

Relevant rules and decisions:

- `CONSTITUTION.md` requires OpenSpec as source of truth, no secrets in tracked files, Rust/Cargo with `make fmt`, `make check`, and `make test` for implementation, and machine-readable `doctor --json` validation for doctor behavior.
- ADR 0005 requires `grill.md`, `design-review.md`, `adr.md`, and `test-plan.md` before apply, and future behavior-changing apply must use RED/GREEN/REFACTOR slices.
- ADR 0008 keeps X11 root/global coordinates and screenshot-crop output integrity in force.
- ADR 0009 keeps the supported claim scoped to Cinnamon/X11 `x11-ewmh`; Wayland and unsafe targeted input without verification are unsupported/out of scope.
- ADR 0010 preserves standalone plugin identity and localized provider takeover boundaries.

## Goals / Non-Goals

**Goals:**

- Make `doctor --json` stable and readable for production X11 readiness: blockers, acceptable degraded layers, optional enrichments, unsupported paths, recommendations, and redacted diagnostics.
- Make AT-SPI diagnostics precise enough to explain bus reachability, tree extraction, no-match, ambiguity, and controlled-fixture pass states.
- Make e2e evidence and matrix validation classify pass/degraded/fail rows with stable reason categories and durable evidence paths.
- Make fake screenshot and metadata-only live smoke outcomes honest without reducing real live screenshot/input safety requirements.
- Strengthen controlled live fixture uniqueness, cleanup, target release, overlay hiding, and stale-target evidence.
- Update documentation so a developer can run and interpret a safe full retest and production-readiness claim.

**Non-Goals:**

- Wayland support or Wayland architecture.
- Portal-based input/screenshot as a required runtime path.
- Sending input, pointer, screenshot, app-state, target, or overlay operations to real user applications during live smoke.
- Secret handling changes or `.secrets.local.env` access.
- Implementation before all planning artifacts are complete and validated.
- Changing standalone MCP tool names or provider takeover identity.

## Decisions

1. **Readiness model:** Keep existing doctor top-level compatibility fields and add structured categories underneath readiness/check/capability facts rather than replacing the JSON contract. `readiness.ok` is true only when X11-baseline blockers are absent; optional enrichment degradation does not force failure.

2. **Diagnostic taxonomy:** Use canonical outcome codes and `reason_category` values consistently across doctor, app-state summaries, AT-SPI correlation, and e2e evidence. Required categories include environment limitation, missing fixture setup, code failure, unsupported out-of-scope, and expected fake-fixture limitation.

3. **AT-SPI handling:** Treat bus reachability, tree extraction, no-match, ambiguity, and controlled-fixture match as separate states. No-match and ambiguity remain safe degradations and never return arbitrary subtrees.

4. **Fixture safety:** Live smoke must prove controlled fixture uniqueness before input/pointer/overlay/screenshot/app-state operations. If no unique controlled fixture exists, the row is `missing_fixture_setup`; the harness must not target ambient apps.

5. **Fake screenshot approach:** Future apply may either implement fake screenshot provider support or preserve degraded fake screenshot evidence with an explicit expected limitation. In both options, real screenshot crop integrity remains strict: provider success without a valid output file is a code failure.

6. **Metadata-only live smoke:** Metadata-only live runs are useful as environment diagnostics but cannot count as controlled live production evidence for fixture-dependent rows. They must say `missing_fixture_setup` / unsafe to test against real apps.

7. **Cleanup evidence:** Controlled live runs record cleanup of overlays, target context, fixture processes, and stale target state. Cleanup failures become explicit degraded/fail evidence depending on safety impact.

8. **Documentation:** Docs explain how to interpret PASS/DEGRADED/FAIL and production claims, including that RemoteDesktop portal absence can be diagnostic but not a Cinnamon/X11 blocker.

## Risks / Trade-offs

- Adding new diagnostic fields can create schema drift if not kept additive and validated by fixture tests.
- Too many reason categories can make summaries noisy; the design chooses a small canonical set and allows detailed codes under each category.
- Fake screenshot pass requires a reliable fake provider fixture; if that is too costly, expected degraded fake limitation is acceptable only when clearly documented and real live screenshot integrity remains verified.
- Live fixture cleanup can be flaky on desktops; evidence must separate environment limitation from code failure without hiding stale state.
- AT-SPI behavior depends on desktop accessibility configuration; controlled GTK fixture evidence improves confidence but cannot make semantic accessibility mandatory for the X11 window/input baseline.

## Migration Plan

No data migration is required. Future apply should proceed in vertical TDD slices:

1. Add parser/model tests for doctor readiness taxonomy and redaction.
2. Add AT-SPI diagnostic outcome tests and controlled fixture evidence shape.
3. Add matrix validator tests for reason categories and metadata-only missing fixture setup.
4. Add or document fake screenshot provider behavior while preserving real crop integrity tests.
5. Add live fixture uniqueness/cleanup tests using fake-live and controlled live harness boundaries.
6. Update docs and run full validation.

Rollback is code/doc rollback only: because fields are additive and no external state is modified, reverting the change restores prior diagnostics. Live smoke cleanup traps must still be safe when partial apply work fails.

## Open Questions

None.
