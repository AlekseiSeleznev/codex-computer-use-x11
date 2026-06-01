## TDD Strategy

Use the project-local `tdd` discipline with small vertical checks around observable public interfaces:

- repository hygiene is verified through Git's public ignore/tracked-file interface;
- generated plugin metadata is verified through the installer public interface that writes `.codex-plugin/plugin.json` into a temporary `CODEX_HOME`;
- full project health is verified through the constitution-required root Makefile commands after the focused slices are green.

No production runtime behavior changes are planned. Tests/checks will be added or run before the corresponding implementation change and evidence will be recorded during apply.

## Vertical TDD Slices

| Slice | Public interface / behavior | RED command and expected failure | GREEN command and expected pass | Refactor criteria |
| --- | --- | --- | --- | --- |
| T1 | Git repository hygiene ignores timestamped backup artifacts and no longer tracks `openspec/config.yaml.bak.*`. | `git check-ignore -q sample.bak.20260531 && test -z "$(git ls-files '*.bak.*')"` should fail initially because `*.bak.*` is not ignored and tracked backup files exist. | Same command passes after adding `*.bak.*` to `.gitignore` and removing tracked backup files. | Keep ignore rule minimal; verify `openspec/config.yaml` remains tracked/trackable. |
| T2 | Installer-generated `.codex-plugin/plugin.json` has the corrected homepage and current tool-surface metadata. | Add focused assertions to `tests/plugin_installer.rs`, then run `cargo test --test plugin_installer plugin_installer_creates_owned_bundle_and_config`; it should fail against the stale homepage/description. | Same focused test passes after updating `scripts/install-codex-plugin.sh` manifest metadata. | Keep assertions on observable generated JSON; avoid coupling marketing text to every exact implementation detail. |
| T3 | Overall OpenSpec and project verification remain clean. | No separate RED expected; this is the final safety net over completed behavior slices. | `openspec validate fix-plugin-manifest-hygiene --strict`, `make fmt`, `make check`, and `make test` pass. | Do not mark complete if any required check fails; report exact blocker instead. |

## Mocking / Boundary Policy

- Use the existing installer test boundary: temporary `CODEX_HOME`, `CODEX_CONFIG_FILE`, and `CODEX_X11_PLUGIN_BINARY` point at test-controlled paths.
- Do not mock internal Rust functions or shell script internals.
- Use Git CLI checks for repository metadata because the behavior is Git-facing, not Rust-facing.
- No external systems or secrets are accessed.

## Required Checks

- Focused RED/GREEN for T1 repository hygiene.
- Focused RED/GREEN for T2 plugin installer manifest metadata.
- `openspec validate fix-plugin-manifest-hygiene --strict`.
- `make fmt`.
- `make check`.
- `make test`.
- Final `git status --short` with no unintended dirty state except explicit checkpoint boundaries.

## Evidence Log

- T1 RED: `git check-ignore -q sample.bak.20260531` returned rc=1 and `git ls-files '*.bak.*'` listed `openspec/config.yaml.bak.20260530150421` and `openspec/config.yaml.bak.20260530150551`.
- T1 GREEN: after adding `*.bak.*` to `.gitignore` and removing tracked backup files, `git check-ignore -q sample.bak.20260531` passed, `test -z "$(git ls-files '*.bak.*')"` passed, and `git ls-files openspec/config.yaml` still listed canonical `openspec/config.yaml`.
- T2 RED: after adding manifest metadata assertions, `cargo test --test plugin_installer plugin_installer_creates_owned_bundle_and_config` failed with stale homepage `https://github.com/AlekseiSelin/codex-computer-use-x11` vs expected `https://github.com/AlekseiSeleznev/codex-computer-use-x11`.
- T2 GREEN: after updating `scripts/install-codex-plugin.sh`, `cargo test --test plugin_installer plugin_installer_creates_owned_bundle_and_config` passed (1 test passed, 0 failed).
- Final verification: `openspec validate fix-plugin-manifest-hygiene --strict` passed.
- Final verification: `make fmt`, `make check`, and `make test` passed; `make test` ran 136 tests with 0 failures.
- Additional review-remediation checks: `cargo clippy --all-targets --all-features -- -D warnings` and `cargo build` passed.

## TDD Exceptions

None.
