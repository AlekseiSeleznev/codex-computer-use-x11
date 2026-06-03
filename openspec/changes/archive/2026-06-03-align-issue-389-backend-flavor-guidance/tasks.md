## 1. Adapter Contract Alignment

- [x] 1.1 Add a RED documentation test proving the adapter contract records `agent-sh/computer-use-linux`, selectable backend/flavor, separate future evaluation, and no default behavior change.
- [x] 1.2 Update `docs/codex-desktop-linux-x11-ewmh-adapter.md` with the minimum backend/flavor guidance needed for the test to pass.
- [x] 1.3 Record RED/GREEN evidence for slice 1 in `test-plan.md`.

## 2. Scaffold README Alignment

- [x] 2.1 Add a RED documentation test proving the copyable Linux Feature scaffold README separates current opt-in adapter behavior from backend/flavor experiments.
- [x] 2.2 Update `adapters/codex-desktop-linux/linux-features/x11-ewmh-computer-use/README.md` with the minimum upstream alignment guidance needed for the test to pass.
- [x] 2.3 Record RED/GREEN evidence for slice 2 in `test-plan.md`.

## 3. Verification and Cleanup

- [x] 3.1 Run targeted documentation tests and `openspec validate align-issue-389-backend-flavor-guidance --strict`.
- [x] 3.2 Run full project verification: `make fmt`, `make check`, `make test`, `openspec validate --all --strict`, and `git diff --check`.
- [x] 3.3 Confirm no runtime/scaffold default behavior changed: `feature.json` remains `defaultEnabled=false`, no source changes modify core Computer Use behavior, and `git status --short --untracked-files=all` is clean after the apply checkpoint.
