## Context Read

- `grill.md` resolutions: unset/remove `NO_AT_BRIDGE` for GTK fixture subprocesses; keep AT-SPI bridge-disabled as degraded optional semantic enrichment; no real-window fallback; no global environment mutation.
- `design.md`: proposes sanitized bridge env facts, new doctor diagnostic priority, child-process fixture env helper, fake/validator tests, and documentation updates.
- Existing code review: `src/doctor.rs` currently collapses bus-reachable/tree-unavailable into `atspi_tree_extraction_unavailable`; e2e currently records `NO_AT_BRIDGE=0` in fake GTK fixture evidence and metadata.
- ADRs 0008, 0009, 0010 remain in force.

## Plan Summary Under Review

The design adds a narrow diagnostic state and fixes fixture env handling without changing baseline readiness, global user state, provider identity, or live targeting policy.

## Question Loop

### Question 1: Could unsetting `NO_AT_BRIDGE` in the child env accidentally hide a real parent-session problem?

- **Recommended answer:** No, if evidence records both parent diagnosis and fixture child env facts.
- **Rationale:** Doctor should still report inherited parent `NO_AT_BRIDGE=1`; the fixture child env sanitation is intentional controlled verification. The two facts answer different questions: “why is current session degraded?” and “can a correctly launched GTK fixture expose a tree?”.
- **Resolution:** Apply should keep doctor bridge-env facts separate from fixture metadata and tests should cover both.

### Question 2: Does adding `GTK_MODULES=gail:atk-bridge` overfit Linux Mint 22.3 Cinnamon/X11?

- **Recommended answer:** It is acceptable when recorded as a fixture-specific compatibility hint, not a global requirement.
- **Rationale:** The user scoped this change to X11/Cinnamon baseline and provided that `GTK_MODULES=gail:atk-bridge` is present in the environment. The design says “when needed” and does not mutate global state.
- **Resolution:** Keep `GTK_MODULES` in fixture metadata and docs as a Cinnamon/X11 fixture hint; do not claim it is required for all GTK versions/desktops.

### Question 3: Is the new diagnostic state too specific for existing consumers?

- **Recommended answer:** No, because existing fields remain additive and consumers already handle string diagnostic states.
- **Rationale:** Prior specs require canonical AT-SPI states and additive doctor JSON. The new state is a more specific subtype of tree extraction unavailable with the same `environment_limitation` category.
- **Resolution:** Preserve `tree_available=false`, `atspi_bus_available=true`, reason category, and existing bootstrap fields.

### Question 4: Could `NO_AT_BRIDGE` value redaction make diagnosis less useful?

- **Recommended answer:** Record presence and a sanitized value class only.
- **Rationale:** `NO_AT_BRIDGE=1` itself is non-secret and useful, but arbitrary environment serialization is not. A sanitized field such as `present=true` and `value="1"` is enough.
- **Resolution:** Do not serialize a full environment map; tests should fail if unrelated/private env values appear.

### Question 5: What should happen if controlled GTK fixture still fails after `NO_AT_BRIDGE` is absent?

- **Recommended answer:** Report a non-bridge `atspi_tree_extraction_unavailable`, `missing_fixture_setup`, or precise `environment_limitation` depending on where the failure occurred.
- **Rationale:** The bridge-disabled diagnosis should not mask package, gsettings, process, PyGObject, display, or harness setup failures.
- **Resolution:** Test both present and absent `NO_AT_BRIDGE` cases; docs list package/gsettings/process checks after the bridge-env check.

## Design Findings

- The design corrects a likely bug in existing planning/implementation (`NO_AT_BRIDGE=0`) without requiring a new durable ADR.
- The privacy boundary is adequate if the implementation avoids serializing arbitrary env vars.
- The live safety boundary is adequate because no new fallback path to real user windows is introduced.
- The readiness semantics are consistent with ADR 0009: bridge-disabled AT-SPI is degraded semantic enrichment, not an X11 baseline blocker.
- The design should explicitly update any existing tests/docs that assert `NO_AT_BRIDGE=0`; tasks include this.

## Document Updates Applied

- No updates outside OpenSpec artifacts were made during design review.

## Document Updates Required Before Next Gate

- `adr.md` should record no durable ADR required and list ADR 0008/0009/0010 constraints.
- `test-plan.md` should require RED tests for the old `NO_AT_BRIDGE=0` expectation before implementation changes.
- `tasks.md` should order apply by TDD slices: doctor model, fixture env, validator/evidence, docs, verification.

## ADR Candidates

No durable ADR candidate. The only material choice, unsetting `NO_AT_BRIDGE`, is a diagnostic/fixture correction within existing architecture.

## Open Questions

None.
