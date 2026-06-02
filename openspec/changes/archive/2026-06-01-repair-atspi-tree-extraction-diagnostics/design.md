## Overview

This change repairs AT-SPI readiness by making the bridge-disabled case explicit and by correcting controlled GTK fixture launch semantics. The target behavior is:

1. `doctor --json` sees AT-SPI bus reachability and sanitized bridge-env facts.
2. If `NO_AT_BRIDGE` is present while tree extraction is unavailable, doctor reports `diagnostic_state=atspi_gtk_bridge_disabled_by_environment`, `reason_category=environment_limitation`, and a remediation that names `NO_AT_BRIDGE`.
3. Controlled GTK fixture subprocesses remove `NO_AT_BRIDGE` from their child environment and record that it is absent. They may set `GTK_MODULES=gail:atk-bridge` where the current Cinnamon/X11 environment needs that explicit bridge module hint.
4. Fake tests and validator fixtures prove the diagnosis without inspecting real user windows.
5. Docs explain remediation and safe retest commands without modifying global user environment.

## Current State

- `src/doctor.rs` already has `AccessibilityReport { atspi_bus_available, tree_available, diagnostic_state, reason_category, recommendation, ... }` and canonical states including `atspi_tree_extraction_unavailable`.
- `ProbeFacts` currently stores a raw `env: HashMap<String, String>` and AT-SPI booleans, but not a sanitized bridge-env summary.
- `system_probe_facts()` sets `atspi_tree_available=false` and does not run a controlled GTK fixture.
- `accessibility_report(...)` chooses `atspi_tree_extraction_unavailable` whenever bus is true and tree is false.
- `scripts/e2e/codex-x11-e2e.py` starts fixtures through `ControlledFixtureManager.start(...)` without a fixture-specific env override.
- Fake industrial evidence currently records `env={"GTK_MODULES":"gail:atk-bridge","NO_AT_BRIDGE":"0"}`.
- `scripts/e2e/fixtures/gtk_atspi_fixture.py` defaults metadata `NO_AT_BRIDGE` to `0`.
- `docs/troubleshooting.md` currently recommends the `NO_AT_BRIDGE=0` fixture path.

## Design Decisions

### 1. Represent bridge env as sanitized facts

Add a small sanitized bridge environment model for doctor/evidence rather than serializing arbitrary environment values:

```text
AccessibilityBridgeEnv {
  no_at_bridge_present: bool,
  no_at_bridge_value: Option<String>,   # only "1"/"0"/"<set>" style sanitized values, no secrets
  gtk_modules: Option<String>,          # only include GTK_MODULES because it is already an explicit bridge control
}
```

Implementation can keep this as a nested struct or equivalent fields under `checks`/`accessibility`; the public contract is that `NO_AT_BRIDGE` presence is machine-readable and unrelated environment variables are not dumped.

### 2. Detect bridge-disabled tree unavailability in doctor

Extend `accessibility_report` or its caller to accept the sanitized bridge-env facts. Diagnostic priority becomes:

1. `controlled_fixture_pass` -> `controlled_fixture_pass`
2. `!bus` -> `atspi_bus_unavailable`
3. `bus && !tree && no_at_bridge_present` -> `atspi_gtk_bridge_disabled_by_environment`
4. `bus && !tree` -> `atspi_tree_extraction_unavailable`
5. tree + explicit match outcome -> existing outcome mapping
6. tree + no outcome -> `tree_extraction_available`

The recommendation for the new state must say:

- remove or avoid inheriting `NO_AT_BRIDGE=1` for GTK fixture/application processes;
- restart the affected Cinnamon/Codex session or fixture process after changing environment;
- verify with the controlled GTK fixture;
- keep this as optional semantic enrichment for the X11 baseline unless fixture AT-SPI pass evidence is being claimed.

`readiness.recommended_next_step` should prefer the bridge-specific remediation over the generic “Enable AT-SPI tree extraction” message when this state is present.

### 3. Correct controlled GTK fixture child environment

Introduce a helper in the e2e harness, for example:

```python
def fixture_env(role: str) -> dict[str, str]:
    env = os.environ.copy()
    if role == "gtk":
        env.pop("NO_AT_BRIDGE", None)
        env["GTK_MODULES"] = env.get("GTK_MODULES") or "gail:atk-bridge"
    return env
```

Pass this env to `subprocess.Popen` for fixtures. Do not mutate `os.environ` globally. For Tk fixtures, leave the environment unchanged unless existing tests require otherwise.

GTK fixture metadata should record:

```json
{
  "bridge_env": {
    "GTK_MODULES": "gail:atk-bridge",
    "NO_AT_BRIDGE": null,
    "NO_AT_BRIDGE_PRESENT": false
  }
}
```

The exact field names may follow existing evidence conventions, but tests must prove the metadata no longer claims `NO_AT_BRIDGE=0`.

### 4. Validation and fake evidence behavior

Add fake/unit tests before implementation:

- Doctor test: bus true, tree false, env contains `NO_AT_BRIDGE=1` -> `atspi_gtk_bridge_disabled_by_environment`.
- Doctor test: bus true, tree false, `NO_AT_BRIDGE` absent -> existing `atspi_tree_extraction_unavailable`.
- Recommended-next-step test: bridge-disabled state recommends removing/unsetting `NO_AT_BRIDGE` and controlled GTK fixture verification.
- E2E harness test: parent has `NO_AT_BRIDGE=1`, GTK fixture env/metadata records absent child `NO_AT_BRIDGE` and `GTK_MODULES=gail:atk-bridge`.
- Validator/fake evidence test: AT-SPI degraded row with bridge-disabled state and `environment_limitation` is accepted; no live controlled fixture does not become `code_failure` solely from safe non-targeting.
- Docs test: troubleshooting includes the new section and no longer recommends `NO_AT_BRIDGE=0` as the bridge-enable path.

### 5. Documentation updates

Update `docs/troubleshooting.md` and any README/release checklist references that mention `NO_AT_BRIDGE=0`.

Add a section titled “AT-SPI bus reachable but tree extraction unavailable” with:

- what `atspi_bus_available=true` and `tree_available=false` mean;
- package checks (`at-spi2-core`, `libatk-adaptor`, `libatk-bridge2.0-0t64`, `libatspi2.0-0t64` or distro equivalents);
- settings/process checks (`toolkit-accessibility`, `at-spi-bus-launcher`, AT-SPI DBus daemon, `at-spi2-registryd`);
- `NO_AT_BRIDGE=1` as a likely bridge-disable cause when inherited by Codex/fixture processes;
- remediation: remove/do not inherit `NO_AT_BRIDGE`, restart affected sessions/processes, then run controlled GTK fixture evidence;
- safety: no real user windows as fallback;
- scope: degraded semantic accessibility enrichment is expected/allowed for the X11 baseline but cannot be called an AT-SPI pass without controlled evidence.

## Safety and Privacy

- Do not read `.secrets.local.env`.
- Do not serialize arbitrary environment maps. Only bridge-relevant keys may appear in sanitized evidence.
- Do not change global user environment, desktop settings, or bundled `computer-use`.
- Do not send input/pointer/overlay/screenshot/app-state/AT-SPI operations to real user windows as fallback.
- Live evidence must use controlled fixtures with run-scoped title/class/process identity.

## Files Expected to Change During Apply

- `src/doctor.rs` — bridge-env facts, diagnostic state, recommendation, tests.
- `scripts/e2e/codex-x11-e2e.py` — fixture child environment, evidence, fake/validator behavior.
- `scripts/e2e/fixtures/gtk_atspi_fixture.py` — metadata records absent `NO_AT_BRIDGE` correctly.
- `tests/e2e_harness_scripts.rs` and possibly doctor tests — TDD coverage.
- `docs/troubleshooting.md`, `README.md`, and/or `docs/release-checklist.md` — documentation wording and retest guidance.
- OpenSpec specs may be synced only during archive, not during apply unless a planning correction is needed.

## Rejected Options

- **Set `NO_AT_BRIDGE=0` for the fixture.** Rejected because presence-based bridge suppression may still treat it as disabled, and current live evidence suggests this is misleading.
- **Run real-window AT-SPI probes as fallback.** Rejected by user safety constraints and controlled-fixture policy.
- **Make AT-SPI bridge-disabled state a baseline blocker.** Rejected because ADR 0009 treats AT-SPI semantic enrichment as degradable for the X11 baseline.
- **Patch bundled `computer-use` or provider takeover code.** Rejected as out of scope and conflicting with ADR 0010 boundaries.

## Rollback

All changes are repository-local. Reverting the apply commit/worktree changes restores the previous generic AT-SPI tree-unavailable diagnosis and fixture metadata. No global desktop or user environment changes are performed by this design.
