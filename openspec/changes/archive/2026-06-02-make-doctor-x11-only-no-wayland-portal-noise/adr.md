## ADR Review

## Existing In-Force ADRs

- `adr/0001-adopt-codex-native-intent-driven-openspec-overlay.md` — Accepted; remains in force for OpenSpec/Codex lifecycle discipline.
- `adr/0003-formalize-project-context-entrypoints.md` — Accepted; remains in force for root context, architecture snapshot, and local secret boundaries.
- `adr/0005-adopt-matt-grill-and-tdd-gates.md` — Accepted; remains in force for mandatory grill/design-review and TDD gates.
- `adr/0006-adopt-claude-artifact-review.md` — Accepted; remains in force, though this session has Claude artifact review disabled in session state.
- `adr/0007-adopt-automatic-checkpoints-and-claude-session-controls.md` — Accepted; remains in force for automatic safe lifecycle checkpoint commits.
- `adr/0008-adopt-x11-root-coordinate-model.md` — Accepted; remains in force and is not directly changed.
- `adr/0009-adopt-final-cinnamon-x11-v1-dod-baseline.md` — Accepted; remains in force. This change aligns doctor readiness with its `x11-ewmh` Cinnamon/X11 baseline and out-of-scope Wayland/portal-required runtime paths.
- `adr/0010-adopt-x11-provider-takeover-shim.md` — Accepted; remains in force and is not directly changed.
- `adr/0011-adopt-rollback-first-install-manifest.md` — Accepted; remains in force and is not directly changed.

Superseded historical ADRs 0002 and 0004 were considered through `adr/README.md` and remain superseded.

## Constitution / Architecture Rules Considered

- `CONSTITUTION.md` requires Rust 2021/Cargo, root `Makefile` checks (`make fmt`, `make check`, `make test`), strict OpenSpec validation, and machine-readable doctor JSON validation.
- `CONSTITUTION.md` forbids reading, printing, staging, committing, archiving, or copying real secrets; this change needs no `.secrets.local.env` access.
- `ARCHITECTURE.md` states the standalone plugin keeps `codex-computer-use-x11` identity and exposes `x11_*` tools; the Rust core owns degraded diagnostics.
- `ARCHITECTURE.md` and ADR 0009 state the supported baseline is Cinnamon/X11 `x11-ewmh`, while Cinnamon Wayland and portal-required runtime paths are unsupported/out of scope until a future design/ADR changes scope.
- `CONTEXT.md` defines `x11-ewmh` as the canonical backend label and distinguishes it from Cinnamon-specific validation.

## Decisions Evaluated

- Keep RemoteDesktop/Wayland JSON fields as neutral compatibility/debug facts vs remove them immediately. Chosen: keep neutral fields for compatibility and remove readiness influence.
- Treat RemoteDesktop portal as a supported input backend vs local X11 input only. Chosen: local X11 input only (`/dev/uinput` and ydotool) for standalone doctor readiness.
- Emit unsupported Wayland runtime issues in doctor readiness vs keep unsupported scope in static docs. Chosen: static docs only; no per-run readiness issue when X11 baseline passes.
- Create a new durable ADR vs rely on ADR 0009. Chosen: no new durable ADR because ADR 0009 already records the X11-only product scope.

## New Durable ADRs Created

- None.

## Superseded ADRs

- None.

## Architecture Snapshot Updates

- None. The current `ARCHITECTURE.md` already says ADR 0009 defines the Cinnamon/X11 `x11-ewmh` baseline and that Wayland/portal-required runtime paths remain unsupported/out of scope.

## No ADR Needed

- This change corrects doctor readiness/reporting semantics under the existing accepted architecture. It is reversible, not a new runtime boundary, and not surprising once ADR 0009 is read: the plugin is X11/EWMH-only, so unsupported RemoteDesktop/Wayland capabilities should not affect doctor readiness.
