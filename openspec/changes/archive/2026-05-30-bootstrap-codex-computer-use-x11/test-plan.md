## TDD Strategy

Apply the project-local `tdd` skill with small vertical tracer bullets. Each slice introduces one observable behavior through a public interface or command-based check, proves RED first, implements only the minimum GREEN path, and refactors only while all relevant checks stay green.

This test plan is intentionally not an "all tests first, all implementation later" plan. During apply, each slice must record its RED command/output and GREEN command/output in the Evidence Log before the corresponding task can be marked complete.

Allowed setup before the first behavior slice is limited to files needed to make the first RED check executable, such as an empty source directory or placeholder files. Do not implement behavior during setup. If a setup step writes more than minimal harness/scaffold, record it as a TDD exception before continuing.

## Vertical TDD Slices

| Slice | Public interface / behavior | RED command and expected failure | GREEN command and expected pass | Refactor criteria |
| --- | --- | --- | --- | --- |
| 1. Root Rust package identity | Repository root is a Rust 2021 package named `codex-computer-use-x11` with package version available to Cargo. | `cargo metadata --no-deps --format-version 1` from repository root fails because `Cargo.toml` is absent or package metadata is missing. If a shell assertion is used, it must fail on missing package name/version/edition. | Add minimal root `Cargo.toml` and `src/lib.rs`; rerun `cargo metadata --no-deps --format-version 1` and assert package name `codex-computer-use-x11`, version `0.1.0`, and edition `2021`. | Keep package minimal: no workspace/subcrates, no extra dependencies beyond later slices, and no target checkout writes. |
| 2. X11 id normalizer: equivalent hex ids | Public library function normalizes equivalent X11 hexadecimal id strings to the same `u64`. | Add a single inline `#[cfg(test)]` unit test in `src/x11_id.rs` asserting `parse_x11_window_id("0x5624b36") == parse_x11_window_id("0x05624b36")`; run `cargo test x11_id --lib`; expected failure: missing module/function or failing assertion. | Implement `src/x11_id.rs` and expose it from `src/lib.rs`; rerun `cargo test x11_id --lib`; expected pass. | Parser stays pure and numeric-only; no command-formatting behavior for `wmctrl`, `xprop`, or `xdotool`. |
| 3. X11 id normalizer: invalid input | Public library function rejects empty and invalid hex ids with the designed error enum. | Add one focused inline unit test for `ParseX11WindowIdError::{Empty, InvalidHex}` using empty string and one invalid hex value; run `cargo test x11_id --lib`; expected failure before error handling exists. | Extend the parser minimally; rerun `cargo test x11_id --lib`; expected pass. | Keep error type small and stable for tests; do not introduce `thiserror` or broad error taxonomy. |
| 4. Doctor CLI success JSON | Built binary `codex-computer-use-x11 doctor --json` exits `0`, writes empty stderr, and emits machine-readable compact JSON with the bootstrap report. | Add `tests/doctor_cli.rs` integration test invoking `env!("CARGO_BIN_EXE_codex-computer-use-x11")` with `doctor --json`; parse stdout as JSON and assert `project = "codex-computer-use-x11"`, `version = env!("CARGO_PKG_VERSION")`, `backend = "x11-ewmh"`, `readiness.ok = true`, empty blockers, `doctor-json` in implemented capabilities, planned capabilities exactly equal to `["x11-ewmh-windowing"]`, and the stable checks `bootstrap-project`, `backend-identity`, and `no-live-x11-probes` with `ok = true`. Assert the `no-live-x11-probes` detail equals or contains `stage 01 performs no live X11 probes or external command execution`. Run `cargo test doctor_cli_success --test doctor_cli`; expected failure: binary/command/report fields missing. | Add minimal `serde`/`serde_json` dependencies, `src/doctor.rs`, and `src/main.rs` success path. Rerun `cargo test doctor_cli_success --test doctor_cli`; expected pass. | Tests parse JSON and ignore trailing whitespace; do not assert pretty formatting. Doctor remains non-invasive: no live X11 probes, no `.secrets.local.env`, no target checkout writes. |
| 5. Doctor CLI error paths | CLI rejects unsupported invocations predictably. | Add integration tests for `--help` and `-h` expecting exit `0`, non-empty usage stdout, and empty stderr; add tests for unknown command and `doctor` without `--json`, expecting non-zero exit and short stderr. Run `cargo test doctor_cli_arguments --test doctor_cli`; expected failure before argument handling exists. | Extend `src/main.rs` argument handling minimally; rerun `cargo test doctor_cli_arguments --test doctor_cli`; expected pass. | Keep parser simple with `std::env::args()`; do not add `clap`, `anyhow`, or broader command surface. |
| 6. Makefile verification wrappers | Root `make fmt`, `make check`, and `make test` delegate to Cargo and propagate failures. | Run `make fmt`, `make check`, and `make test` before root `Makefile` targets exist; expected failure due missing targets/Makefile. This RED is wiring verification, not behavioral code coverage. | Add root `Makefile` targets: `fmt -> cargo fmt -- --check`, `check -> cargo check`, `test -> cargo test`. Run actual `make fmt`, `make check`, and `make test`; expected pass. `make -n` may be used only as optional target-wiring smoke evidence. | The actual make commands are the meaningful GREEN evidence. Do not make `fmt` rewrite files in place. |
| 7. README delivery posture | Public documentation states the bootstrap posture and how to run project checks. | Run the deterministic README text check listed in `Documentation Check Snippets` below; expected failure until `README.md` contains Codex-first, Cinnamon/X11-first, generic X11/EWMH, `x11-ewmh`, standalone plugin, future source overlay, root commands, no live backend in stage 01, and no formal MSRV beyond stable Rust 2021 support. | Add `README.md`; rerun the text check; expected pass. | README is summary-level and links to the normative integration contract instead of duplicating every detail. |
| 8. Integration contract documentation | Normative docs record future source-overlay and sidecar/report boundaries. | Run the deterministic integration-contract text check listed in `Documentation Check Snippets` below; expected failure until `docs/integration-contract.md` includes `x11-ewmh`, upstream `WindowInfo` as primary model, sidecar/report default, non-implemented `WindowObservationMeta` sketch or link, late fallback order after existing desktop-specific backends, `CODEX_DESKTOP_LINUX_FULL_PATH`, and license/reuse policy. | Add `docs/integration-contract.md`; rerun the text check; expected pass. | Documentation remains non-secret and does not claim real backend implementation. This manual/doc verification has the same gate weight as cargo/make checks. |
| 9. Final aggregate verification | Whole bootstrap satisfies OpenSpec and project verification rules. | Before final implementation is complete, at least one of the required aggregate checks should fail or be impossible to run because files/commands are absent. Record the blocker rather than forcing a fake pass. | Run and record: `cargo test`, `make fmt`, `make check`, `make test`, `codex-computer-use-x11 doctor --json` parsed as JSON, documentation checks from slices 7 and 8, `openspec validate bootstrap-codex-computer-use-x11 --type change --json`, `git status --short` with manual visual inspection for unrelated dirty work, and `! git ls-files --error-unmatch .secrets.local.env` (or equivalent) proving the local secret file is not tracked. | Refactor only while all checks remain green; no uncommitted unrelated files; no local secret files staged. |


### Documentation Check Snippets

Use deterministic text checks during slices 7 and 8 instead of ad-hoc grep patterns. The apply agent may implement these inline in shell or as temporary Python snippets; if promoted to tracked scripts later, keep the same required tokens.

README check:

```bash
python3 - <<'PY'
from pathlib import Path
text = Path('README.md').read_text(encoding='utf-8')
search = text.lower()
required = [
    # Hyphenated `codex-first` is required by the project glossary term.
    'codex-first',
    'cinnamon/x11-first',
    'generic x11/ewmh',
    'x11-ewmh',
    'standalone plugin',
    'source overlay',
    'make fmt',
    'make check',
    'make test',
    'no live backend in stage 01',
    'no formal msrv',
    'stable rust 2021',
    'docs/integration-contract.md',
]
missing = [item for item in required if item not in search]
if missing:
    raise SystemExit('README.md missing required text (case-insensitive): ' + ', '.join(missing))
PY
```

Integration contract check:

```bash
python3 - <<'PY'
from pathlib import Path
text = Path('docs/integration-contract.md').read_text(encoding='utf-8')
required = [
    'x11-ewmh',
    'WindowInfo',
    'primary model',
    'sidecar',
    'report',
    'WindowObservationMeta',
    'GNOME extension',
    'GNOME introspect',
    'COSMIC',
    'KWin',
    'Hyprland',
    'i3',
    'CODEX_DESKTOP_LINUX_FULL_PATH',
    'license',
    'reference-first',
]
missing = [item for item in required if item not in text]
if missing:
    raise SystemExit('docs/integration-contract.md missing required text: ' + ', '.join(missing))
PY
```

## Mocking / Boundary Policy

- Prefer behavior tests through public interfaces: Cargo metadata, library unit tests for the pure parser, binary integration tests, Makefile commands, and documentation text checks.
- Do not mock internal collaborators (`doctor`, `main`, or `x11_id`) or assert private implementation details.
- No live X11, `wmctrl`, `xprop`, `xdotool`, `ydotool`, portal, screenshot, AT-SPI, or target-checkout behavior is implemented or invoked in this stage.
- Standalone external-command behavior is out of scope for stage 01. If a future slice adds such behavior, it must use a command-runner seam or fake `PATH` fixture so tests run without live desktop dependencies.
- Source-overlay command style remains documentation-only in this stage: future overlay work defaults to target repo thin `Command::new(...)` wrappers plus pure parser/normalizer fixture tests unless a later design/ADR accepts a dependency-injection runner exception.
- `.secrets.local.env` is not read. No test or command may require external credentials or write to `${CODEX_DESKTOP_LINUX_FULL_PATH}` or the documented development-machine default target checkout.

## Required Checks

Before marking apply complete and before verify/archive, run or explicitly report the blocker for:

- `cargo metadata --no-deps --format-version 1` with package name/version/edition checks.
- `cargo test`.
- `cargo test x11_id --lib`.
- `cargo test --test doctor_cli`.
- `make fmt`.
- `make check`.
- `make test`.
- `codex-computer-use-x11 doctor --json`, parsed as JSON; success must exit `0`, produce empty stderr, satisfy `doctor-cli` spec fields, and assert `capabilities.planned` exactly equals `["x11-ewmh-windowing"]`.
- Documentation text/manual checks proving:
  - `README.md` includes delivery posture, root commands, no live backend in stage 01, and no formal MSRV beyond stable Rust 2021 support.
  - `docs/integration-contract.md` includes or links the non-implemented `WindowObservationMeta` sketch and records the sidecar/report default.
- `openspec validate bootstrap-codex-computer-use-x11 --type change --json`.
- `git status --short`, with manual visual inspection confirming no unrelated dirty work, plus `! git ls-files --error-unmatch .secrets.local.env` or an equivalent deterministic check confirming the local secret file is not tracked.


## Claude Test-Plan Review Disposition

The latest Claude review for this `test-plan` stage returned `pass`, no `mustFix`, and only minor context-answerable findings. No user participation is required.

- The planned capability assertion is exact: `capabilities.planned` must equal `["x11-ewmh-windowing"]` in the stage-01 doctor CLI success test and in the Required Checks section.
- README terminology checks are case-insensitive but intentionally require the hyphenated `codex-first` glossary form, not `Codex first` with a space.
- `git status --short` remains a manual visual inspection for unrelated dirty work because allowed dirty paths depend on the checkpoint moment; the local secret-file policy gets a deterministic check via `git ls-files` to prove `.secrets.local.env` is not tracked.

## Evidence Log

- Slice `1` RED:
  - Command: `cargo metadata --no-deps --format-version 1`
  - Expected failure observed: command exited `101` because the root package manifest did not exist yet.
  - Output/reference: `error: could not find Cargo.toml`.
- Slice `1` GREEN:
  - Command: `cargo metadata --no-deps --format-version 1` plus a JSON assertion for package name/version/edition.
  - Expected pass observed: package identity was `codex-computer-use-x11 0.1.0 2021`.
  - Output/reference: `SLICE1_GREEN_OK codex-computer-use-x11 0.1.0 2021`.
- Slice `1` REFACTOR:
  - Refactor performed or `None`: None; kept a single minimal root package.
  - Checks rerun: package metadata assertion.

- Slice `2` RED:
  - Command: `cargo test x11_id --lib`
  - Expected failure observed: the new equivalent-hex unit test could not compile before the parser existed.
  - Output/reference: unresolved import / missing `parse_x11_window_id`.
- Slice `2` GREEN:
  - Command: `cargo test x11_id --lib`
  - Expected pass observed: equivalent hex ids parse to the same `u64`.
  - Output/reference: `test x11_id::tests::equivalent_hex_ids_parse_to_same_u64 ... ok`.
- Slice `2` REFACTOR:
  - Refactor performed or `None`: None; parser stayed pure and numeric-only.
  - Checks rerun: `cargo test x11_id --lib`.

- Slice `3` RED:
  - Command: `cargo test x11_id --lib`
  - Expected failure observed: the invalid-input test could not compile before the designed error variants existed.
  - Output/reference: no associated constants/variants `Empty` and `InvalidHex`.
- Slice `3` GREEN:
  - Command: `cargo test x11_id --lib`
  - Expected pass observed: empty input returns `ParseX11WindowIdError::Empty` and invalid hex returns `ParseX11WindowIdError::InvalidHex`.
  - Output/reference: `test x11_id::tests::invalid_inputs_return_specific_errors ... ok`.
- Slice `3` REFACTOR:
  - Refactor performed or `None`: None; kept a small local enum and did not add `thiserror`.
  - Checks rerun: `cargo test x11_id --lib`.

- Slice `4` RED:
  - Command: `cargo test doctor_cli_success --test doctor_cli`
  - Expected failure observed: the integration test could not compile before the binary/dependencies existed.
  - Output/reference: `CARGO_BIN_EXE_codex-computer-use-x11` not defined and unresolved `serde_json`.
- Slice `4` GREEN:
  - Command: `cargo test doctor_cli_success --test doctor_cli`
  - Expected pass observed: `doctor --json` exited `0`, stderr was empty, and stdout parsed as the expected compact JSON report.
  - Output/reference: `test doctor_cli_success_json ... ok`.
- Slice `4` REFACTOR:
  - Refactor performed or `None`: None; no live X11 probes, no `.secrets.local.env`, and no target checkout access were added.
  - Checks rerun: `cargo test doctor_cli_success --test doctor_cli`.

- Slice `5` RED:
  - Command: `cargo test doctor_cli_arguments --test doctor_cli`
  - Expected failure observed: `--help` did not yet exit successfully before explicit argument handling.
  - Output/reference: assertion failure: `--help should exit 0`.
- Slice `5` GREEN:
  - Command: `cargo test doctor_cli_arguments --test doctor_cli`
  - Expected pass observed: `--help` and `-h` print usage with exit `0`; unsupported invocations fail with stderr.
  - Output/reference: `test doctor_cli_arguments ... ok`.
- Slice `5` REFACTOR:
  - Refactor performed or `None`: None; kept `std::env::args()` and did not add `clap`, `anyhow`, or broader CLI surface.
  - Checks rerun: `cargo test --test doctor_cli`.

- Slice `6` RED:
  - Command: `make fmt`; `make check`; `make test`
  - Expected failure observed: each command exited `2` before the root `Makefile` existed.
  - Output/reference: `No rule to make target` / missing Makefile targets.
- Slice `6` GREEN:
  - Command: `cargo fmt`; `make fmt`; `make check`; `make test`
  - Expected pass observed: check-only Makefile targets delegated to Cargo and propagated success.
  - Output/reference: `/tmp/bootstrap-aggregate-code.out` shows `cargo fmt -- --check`, `cargo check`, and `cargo test` all passed.
- Slice `6` REFACTOR:
  - Refactor performed or `None`: None; wrappers remained thin over Cargo and `fmt` remained check-only.
  - Checks rerun: `make fmt`, `make check`, `make test`.

- Slice `7` RED:
  - Command: README deterministic Python text check from this test plan.
  - Expected failure observed: the check failed before `README.md` existed.
  - Output/reference: `FileNotFoundError: README.md`.
- Slice `7` GREEN:
  - Command: README deterministic Python text check from this test plan.
  - Expected pass observed: required delivery-posture terms were present.
  - Output/reference: `SLICE7_README_CHECK_OK`; aggregate rerun in `/tmp/bootstrap-aggregate-final.out` reports `README ok`.
- Slice `7` REFACTOR:
  - Refactor performed or `None`: None; README stays summary-level and points to `docs/integration-contract.md`.
  - Checks rerun: README text check.

- Slice `8` RED:
  - Command: integration-contract deterministic Python text check from this test plan.
  - Expected failure observed: the check failed before `docs/integration-contract.md` existed.
  - Output/reference: `FileNotFoundError: docs/integration-contract.md`.
- Slice `8` GREEN:
  - Command: integration-contract deterministic Python text check from this test plan.
  - Expected pass observed: required integration-boundary terms and fallback-order references were present.
  - Output/reference: `SLICE8_INTEGRATION_CONTRACT_CHECK_OK`; aggregate rerun in `/tmp/bootstrap-aggregate-final.out` reports `integration contract ok`.
- Slice `8` REFACTOR:
  - Refactor performed or `None`: None; document remains non-secret and does not claim live backend support.
  - Checks rerun: integration-contract text check.

- Slice `9` RED:
  - Command: aggregate gate attempted before final implementation was complete.
  - Expected failure observed: required aggregate checks were impossible or failing while package/docs/Makefile artifacts were absent.
  - Output/reference: representative blockers were `make fmt`/`make check`/`make test` exit `2`, README `FileNotFoundError`, and integration-contract `FileNotFoundError` during earlier slices.
- Slice `9` GREEN:
  - Command: `cargo metadata --no-deps --format-version 1`; `cargo test`; `cargo test x11_id --lib`; `cargo test --test doctor_cli`; `make fmt`; `make check`; `make test`; built `./target/debug/codex-computer-use-x11 doctor --json` parsed as JSON; README/integration-contract checks; `openspec validate bootstrap-codex-computer-use-x11 --type change --json`; `git ls-files --error-unmatch .secrets.local.env`; target-checkout `git status --short`.
  - Expected pass observed: all Cargo/Make/doc/OpenSpec gates passed, `doctor --json` parsed with exact `capabilities.planned == ["x11-ewmh-windowing"]`, `.secrets.local.env` was not tracked, and the target checkout status was empty.
  - Output/reference: `/tmp/bootstrap-aggregate-code.out`, `/tmp/bootstrap-aggregate-final.out`, and final post-evidence rerun `/tmp/bootstrap-final-post-evidence.out`; `openspec validate` summary `items: 1, passed: 1, failed: 0`; `.secrets.local.env not tracked`; `doctor json ok`.
- Slice `9` REFACTOR:
  - Refactor performed or `None`: Added `/target/` to `.gitignore` so generated Cargo build output is not checkpointed as dirty work; restored unrelated Claude review/check-overlay drift to the committed project state before checkpointing.
  - Checks rerun: final post-evidence checks were rerun before checkpoint and passed; see `/tmp/bootstrap-final-post-evidence.out`.

## TDD Exceptions

None.
