## Why

The canonical `doctor-cli` and `x11-integration-contract` specs still contain bootstrap placeholder Purpose text, which makes the archived source of truth look unfinished even though the requirements have been expanded and archived.
This maintenance change replaces those placeholders with accurate, durable purpose descriptions without changing product behavior.

## What Changes

- Replace the `Purpose: TBD` bootstrap placeholder in `openspec/specs/doctor-cli/spec.md` with a concise description of the doctor CLI specification's role.
- Replace the `Purpose: TBD` bootstrap placeholder in `openspec/specs/x11-integration-contract/spec.md` with a concise description of the X11 integration contract's role.
- Keep all normative requirements and scenarios unchanged unless a spec delta explicitly identifies a metadata-only expectation.
- No Rust code, CLI behavior, source-overlay behavior, external systems, or secrets are involved.

## Capabilities

- Modify `doctor-cli` to require canonical spec purpose metadata that describes the `doctor --json` capability/readiness surface.
- Modify `x11-integration-contract` to require canonical spec purpose metadata that describes the X11/EWMH/source-overlay integration contract.

## Impact

- Affects documentation/spec metadata only:
  - `openspec/specs/doctor-cli/spec.md`
  - `openspec/specs/x11-integration-contract/spec.md`
- Uses the existing intent-driven OpenSpec lifecycle and project-local checkpoint discipline.
- No architecture change, durable ADR, secret handling, external-system access, dependency, or runtime behavior change is expected.
- Verification should include OpenSpec validation and a text check proving the two placeholder strings are gone.
