## 1. Repository backup hygiene (TDD slice T1)

- [x] 1.1 Capture RED evidence that Git currently does not ignore `*.bak.*` and still tracks `openspec/config.yaml.bak.*` artifacts.
- [x] 1.2 Add a minimal `*.bak.*` rule to `.gitignore` while preserving existing secret/session ignore rules.
- [x] 1.3 Remove tracked `openspec/config.yaml.bak.20260530150421` and `openspec/config.yaml.bak.20260530150551` backup artifacts without removing canonical `openspec/config.yaml`.
- [x] 1.4 Capture GREEN evidence that `git check-ignore` recognizes timestamped backups and `git ls-files '*.bak.*'` is empty.

## 2. Plugin manifest metadata (TDD slice T2)

- [x] 2.1 Add focused failing assertions in `tests/plugin_installer.rs` for corrected manifest homepage and current tool-surface metadata.
- [x] 2.2 Capture RED evidence from `cargo test --test plugin_installer plugin_installer_creates_owned_bundle_and_config` against the stale installer manifest.
- [x] 2.3 Update `scripts/install-codex-plugin.sh` generated manifest homepage, long description, and default prompts to match the current repository and representative `x11_*` tool surface.
- [x] 2.4 Capture GREEN evidence from the same focused plugin installer test.

## 3. Verification and checkpoint

- [x] 3.1 Update `openspec/changes/fix-plugin-manifest-hygiene/test-plan.md` evidence log with T1/T2 RED/GREEN command evidence.
- [x] 3.2 Run `openspec validate fix-plugin-manifest-hygiene --strict`.
- [x] 3.3 Run `make fmt`.
- [x] 3.4 Run `make check`.
- [x] 3.5 Run `make test`.
- [x] 3.6 Verify final `git status --short` and create a scoped apply checkpoint commit; do not push or archive without explicit approval.
