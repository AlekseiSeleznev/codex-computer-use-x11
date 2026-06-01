## Why

The installed `codex-computer-use-x11` plugin now passes safe retest for the supported Cinnamon/X11 baseline, but the remaining degraded evidence needs production-grade classification so operators can distinguish blockers, expected environment limitations, missing fixtures, and code failures without reading chat history. This change polishes diagnostics, readiness, controlled-fixture evidence, and documentation for the X11-only v1 claim while explicitly keeping Wayland and portal-required runtime paths out of scope.

## What Changes

- Harden `doctor --json` into a stable, machine-readable readiness model with explicit blockers, acceptable X11 degradations, optional enrichments, and unsupported/out-of-scope paths.
- Expand AT-SPI diagnostics so bus reachability, tree extraction, no-match, ambiguous-match, and controlled-fixture pass outcomes are distinct and actionable for Cinnamon/X11.
- Improve e2e evidence schema and matrix validation with canonical `reason_category` values for `environment_limitation`, `missing_fixture_setup`, `code_failure`, `unsupported_out_of_scope`, and expected fake-fixture limitations.
- Clarify fake-mode screenshot behavior by either adding a fake screenshot provider fixture or documenting the fake `gdbus` limitation as expected degraded evidence without weakening real screenshot-crop integrity.
- Make metadata-only live smoke report `missing_fixture_setup` when no controlled fixtures are available, rather than looking like a code failure or a safe production pass.
- Strengthen controlled live fixture uniqueness, lifecycle cleanup, and safety evidence so live input/pointer/overlay operations never fall back to real user applications.
- Update README/docs/troubleshooting/retest guidance so PASS, DEGRADED, doctor readiness, Wayland out-of-scope status, and production-claim evidence are understandable.

## Capabilities

Modified capabilities:

- `doctor-cli` — readiness JSON, AT-SPI/portal diagnostic classification, redaction, and X11-only supported-baseline semantics.
- `x11-atspi-window-correlation` — AT-SPI diagnostic taxonomy and controlled GTK fixture acceptance evidence.
- `codex-x11-e2e-test-harness` — evidence schema, fake/live smoke classification, fixture safety, matrix validation, and cleanup evidence.
- `x11-screenshot-coordinate-model` — fake screenshot fixture/degradation semantics while preserving real screenshot crop output integrity.
- `x11-get-app-state-integration` — metadata-only live smoke and sanitized layer summaries that classify fixture setup limitations correctly.
- `x11-target-window-groups-overlays` — overlay/target release cleanup and stale target prevention evidence for controlled fixtures.
- `x11-packaging-docs-upstreaming` — production-readiness and troubleshooting documentation for X11-only PASS/DEGRADED semantics.

## Impact

- Affected code will likely include the Rust CLI/MCP doctor and diagnostics model, AT-SPI probe reporting, e2e harness scripts, matrix validator, controlled fixture helpers, screenshot fake fixtures or documentation, and README/docs/troubleshooting files.
- No external systems or credentials are required; `.secrets.local.env` must remain unread, unprinted, unstaged, and uncommitted.
- The supported architecture remains ADR 0009's Cinnamon/X11 `x11-ewmh` baseline; Wayland and portal-required runtime paths remain unsupported/out of scope unless a future ADR changes that scope.
- ADR 0008's X11 root-coordinate model and screenshot-output-path safety remain in force.
- ADR 0010's standalone plugin identity and settings-provider separation remain in force; this change does not rename tools or change provider takeover architecture.
- Verification must include OpenSpec strict validation and, during future apply, public-interface TDD slices plus `make fmt`, `make check`, `make test`, `doctor --json` JSON validation, fake smoke, controlled live fixture smoke where available, and matrix validation.
