# Release checklist

Use this checklist before claiming v1 handoff readiness, archiving the OpenSpec change, or preparing a GitHub push. The checklist records commands and evidence; it must not contain secret values. The final readiness report is `docs/final-architecture-dod.md` and the deterministic gate is `scripts/validate-final-dod.py`.

## Required local validation

```bash
cargo test --test packaging_docs
cargo test --test final_dod
scripts/validate-final-dod.py
make fmt
make check
make test
openspec validate --all --strict
git status --short
```

## E2E evidence

Run deterministic fake evidence first:

```bash
scripts/e2e/codex-plugin-smoke.sh --fake --log-dir target/e2e-logs/release-plugin-fake
scripts/e2e/codex-source-overlay-smoke.sh --fake --log-dir target/e2e-logs/release-source-overlay-fake
scripts/e2e/codex-x11-e2e.py validate-matrix --evidence target/e2e-logs/release-plugin-fake/evidence.json
scripts/e2e/codex-x11-e2e.py validate-matrix --evidence target/e2e-logs/release-source-overlay-fake/evidence.json
```

Optional live evidence may be collected only when the target checkout exists and starts clean:

```bash
git -C "$CODEX_DESKTOP_LINUX_FULL_PATH" status --short
scripts/e2e/codex-source-overlay-smoke.sh --live --target "$CODEX_DESKTOP_LINUX_FULL_PATH" --log-dir target/e2e-logs/release-source-overlay-live
scripts/uninstall-codex-source-overlay.sh --target "$CODEX_DESKTOP_LINUX_FULL_PATH"
scripts/status-codex-source-overlay.sh --target "$CODEX_DESKTOP_LINUX_FULL_PATH"
git -C "$CODEX_DESKTOP_LINUX_FULL_PATH" status --short
```

If optional live evidence is skipped, record the reason, such as no clean target checkout or no safe live desktop session.

## Production claim evidence

Before claiming Cinnamon/X11 production readiness, retain path-based evidence
for each of these checks:

- `doctor --json` parsed as machine-readable JSON with the X11 readiness
  taxonomy, blockers, degraded acceptable rows, optional enrichments, and
  unsupported/out-of-scope paths.
- fake smoke for the installed standalone plugin and source overlay, followed by
  matrix validation.
- metadata-only live smoke when controlled fixtures are not available; this is
  freshness evidence only and should classify fixture-dependent rows as
  `missing_fixture_setup`.
- controlled live fixture smoke for production claims, using unique neutral controlled
  fixture windows and cleanup evidence for target release, overlay hiding, and
  fixture process shutdown.

Evidence must keep screenshots as file paths and metadata, with no inline screenshot data URLs in ordinary logs or summaries. For app-state, use `--screenshot-output <path>` or generated `screenshot.path` evidence; `--inline-screenshot` is unsafe for release evidence. Do not include tokens,
credentials, private local configuration, or `.secrets.local.env` values.

## License refresh

- Repeat the license refresh before copying code or making release/upstream claims.
- Confirm `docs/license-attribution.md` still reflects the observed references or update it through a new change.
- Preserve the rule that runtime command invocation is distinct from source copying/vendoring.

## Secret-safety gate

- Do not read, print, commit, archive, or copy `.secrets.local.env`.
- Use variable names such as `CODEX_DESKTOP_LINUX_FULL_PATH`; do not record private path secrets beyond documented non-secret local defaults.
- Check logs and evidence for tokens, credentials, private local configuration, and private endpoints before sharing.

## Archive and push gate

Before archive:

```bash
git status --short
openspec validate --all --strict
```

After archive, confirm the archive path, synced specs, final project checks, and clean git status. Only then commit the archive/spec sync and push when explicitly approved by the user.

## Industrial live verification gate

Before claiming production/industrial live readiness on Cinnamon/X11, collect fixture-backed live evidence in addition to deterministic fake evidence:

```bash
scripts/e2e/codex-plugin-smoke.sh --live --industrial --log-dir target/e2e-logs/<run-id>/plugin-live
scripts/e2e/codex-x11-e2e.py validate-matrix --industrial --evidence target/e2e-logs/<run-id>/plugin-live/<run>/evidence.json
```

The live industrial run must use controlled fixtures for fixture-dependent capabilities. It must not send input, pointer actions, screenshots, app-state inspection, target-window, or overlay operations to uncontrolled real user applications. Evidence should remain under `target/e2e-logs/<run-id>` and reference screenshots by file path/metadata, not by huge inline data URLs.

Accept `environment_limitation` degraded rows only when fixture orchestration was attempted and the missing desktop/toolkit dependency is recorded. Do not accept `missing_fixture_setup`, `unsafe_target_selection`, `code_failure`, `malformed_evidence`, or `not_evaluated` as release-ready industrial live evidence.
