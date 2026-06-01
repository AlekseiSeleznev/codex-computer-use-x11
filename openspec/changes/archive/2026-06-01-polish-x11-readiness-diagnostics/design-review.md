## Context Read

- `CONSTITUTION.md`, `CONTEXT.md`, `ARCHITECTURE.md`, `adr/README.md`.
- In-force ADRs 0005, 0007, 0008, 0009, and 0010.
- Proposal, delta specs, `grill.md`, and `design.md` for `polish-x11-readiness-diagnostics`.
- Existing canonical specs for doctor, AT-SPI, e2e harness, screenshot, app-state, target/overlay, and packaging docs.

## Design Summary

- The design is additive: it preserves existing doctor bootstrap fields and refines evidence/readiness classification.
- X11/Cinnamon remains the only supported runtime scope; Wayland and portal-required paths are diagnostics/out of scope.
- Controlled fixtures are mandatory for live input/pointer/overlay and fixture-dependent production evidence.
- Fake screenshot behavior can be fixed with a fake provider or kept degraded only when the limitation is explicit and isolated from real crop integrity.
- Cleanup and stale-target evidence become first-class production-readiness checks.

## Question Loop

### Question 1: Does additive doctor JSON preserve existing consumers?

- **Recommended answer:** Yes, provided all existing top-level fields and bootstrap types remain unchanged and new classification fields are additive.
- **Rationale:** Existing `doctor-cli` specs require bootstrap field compatibility. The design does not remove or rename fields.
- **Resolution:** No design change required; test plan includes JSON shape regression checks.

### Question 2: Is `missing_fixture_setup` too strict for metadata-only live smoke?

- **Recommended answer:** No. It is strict by design for production/industrial acceptance while still allowing metadata-only smoke to provide environment diagnostics.
- **Rationale:** The unsafe alternative is silently testing real user apps or treating missing controlled evidence as a code/environment pass.
- **Resolution:** Specs and design keep metadata-only live smoke useful but not production-pass evidence for fixture-dependent rows.

### Question 3: Should AT-SPI degradation make `readiness.ok=false` on the current desktop?

- **Recommended answer:** No, not for the Cinnamon/X11 window/input baseline. It should degrade semantic accessibility enrichment unless a specific AT-SPI-controlled fixture requirement is being validated.
- **Rationale:** ADR 0009 allows degraded diagnostics for environment-dependent capabilities and requires explicit evidence rather than fabricated pass claims.
- **Resolution:** Specs keep AT-SPI as optional enrichment for baseline doctor readiness while controlled fixture AT-SPI evidence remains required for that row when claiming live semantic accessibility pass.

## Design Findings

- No constitution conflict found. The change does not need external systems or secrets.
- No architecture conflict found. The change refines ADR 0009 evidence semantics and does not expand scope to Wayland.
- The major safety edge case is accidental fallback to ambient real apps; specs now require fixture uniqueness proof and block ambiguous/missing fixtures.
- The major evidence-quality edge case is treating provider success without durable screenshot output as pass; specs keep this as code failure.
- The major cleanup edge case is stale target/overlay state after failures; specs require cleanup evidence.

## Document Updates Applied

- Design keeps Wayland/portal-required runtime paths as unsupported/out-of-scope diagnostics.
- Design preserves doctor JSON compatibility and explicitly plans additive fields.
- Design records future TDD slice order and rollback notes.
- `CONTEXT.md` was updated with `Reason category` and `Controlled fixture`.

## Document Updates Required Before Next Gate

None.

## ADR Candidates

- No durable ADR required. This is a refinement within existing accepted ADRs rather than a new hard-to-reverse architecture decision.

## Open Questions

None.
