## ADR Review

## Existing In-Force ADRs

- `adr/README.md` — considered as the available durable ADR index in this checkout; it lists ADR 0001, 0003, 0005, 0006, and 0007 as in force and ADR 0002/0004 as superseded.
- `ARCHITECTURE.md` — considered as the current architecture snapshot and source of summarized in-force ADR constraints.
- Numbered top-level ADR body files (`adr/0001-*.md`, etc.) are referenced by `ARCHITECTURE.md` and `adr/README.md` but are not present in this checkout, so this review does not invent body details beyond the available summaries.

## Constitution / Architecture Rules Considered

- Use Rust 2021 and root Cargo/Makefile verification (`make fmt`, `make check`, `make test`).
- Keep the project Codex-first, Cinnamon/X11-first, and generic X11/EWMH-oriented.
- Use `x11-ewmh` as the canonical backend id.
- Keep source-overlay compatibility with the target repo's `WindowInfo`, `ReadinessReport`, `WindowingReport`, and command-style constraints.
- Keep local secrets out of artifacts and do not require `.secrets.local.env` for this local desktop listing work.
- Do not write to the machine-local Codex Desktop Linux target checkout for this standalone change.
- Preserve mandatory grill/design-review/TDD/checkpoint gates.
- Safe lifecycle checkpoint commits are automatic in this session; merge, push, archive, and destructive operations remain outside this fast-forward.

## Decisions Evaluated

- Use `wmctrl -lpGx` as the standalone MVP window listing source instead of introducing native `x11rb` now.
- Keep primary `windows[]` objects compatible with target `WindowInfo` and place raw X11/provenance/PID reliability fields in diagnostics sidecars.
- Mark focus from `_NET_ACTIVE_WINDOW` via `xprop -root` when available.
- Avoid unbounded per-window `xprop -id` enrichment for `_NET_WM_WINDOW_TYPE` and hidden state in the MVP.
- Use a standalone command seam or fake `PATH` fixtures for TDD without imposing a dependency-injection runner on the future target repo source overlay.
- Treat live smoke output as sensitive enough to avoid recording real window titles in artifacts or chat.

## New Durable ADRs Created

- None.

## Superseded ADRs

- None.

## Architecture Snapshot Updates

- None. The current architecture snapshot already supports a standalone generic X11/EWMH listing stage and the existing source-overlay compatibility boundaries.

## No ADR Needed

- No durable ADR is needed for this change because the selected approach is an incremental, reversible MVP within the existing architecture: `wmctrl -lpGx` can later be replaced by native X11 or bounded enrichment without changing the project-wide architecture.
- The `WindowInfo` primary/sidecar split is already captured by the canonical `x11-integration-contract` spec, so this change does not introduce a new hard-to-reverse architecture decision.
- No existing ADR is superseded, and `ARCHITECTURE.md` does not need a snapshot update.
