## 1. Plugin UI metadata and asset

- [x] 1.1 Add RED assertions in `tests/plugin_installer.rs` for author/developer `AlekseiSeleznev`, `interface.websiteURL`, missing privacy/terms links, marketplace `X11 Computer Use` display name, and installed `assets/app-icon.png`.
- [x] 1.2 Add a tracked project-owned `assets/app-icon.png` and update `scripts/install-codex-plugin.sh` to copy it into the installed plugin bundle.
- [x] 1.3 Update generated plugin/marketplace metadata to match the spec and pass the focused installer test.

## 2. Plugin smoke stale-install detection

- [x] 2.1 Add RED coverage in the e2e harness tests for rejecting stale six-tool installs and validating UI metadata/icon fields.
- [x] 2.2 Update `scripts/e2e/codex-x11-e2e.py` metadata/tool validation to enforce the current fourteen-tool surface and UI metadata contract.
- [x] 2.3 Run focused e2e harness tests and fake plugin smoke.

## 3. MCP desktop environment hydration

- [x] 3.1 Add RED MCP tests proving missing `DISPLAY` can be hydrated from deterministic fixture sources and explicit `DISPLAY` is preserved.
- [x] 3.2 Implement MCP startup desktop-env hydration with an allowlist of graphical/session variables and deterministic test seams.
- [x] 3.3 Run focused MCP tests and confirm no non-JSON stdout noise or secret values are emitted.

## 4. Verification and user-local refresh

- [x] 4.1 Run `make fmt`, `make check`, `make test`, and `openspec validate fix-codex-plugin-ui-installation --strict`.
- [x] 4.2 Run `scripts/e2e/codex-plugin-smoke.sh --fake`; run live plugin smoke if safe or record the exact blocker.
- [x] 4.3 Reinstall the user-local plugin from the updated installer, verify current installed binary tool discovery, and report Codex UI refresh/restart instructions if live tool discovery cannot reload in-process.
- [x] 4.4 Update `test-plan.md` evidence log and checkpoint the completed apply group.
