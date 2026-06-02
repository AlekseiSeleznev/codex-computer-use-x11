## 1. App-state screenshot CLI safety

- [x] 1.1 Add a RED CLI test proving `get-app-state --window-id <id> --json` contains no `data:image`/`;base64,` by default and references a non-empty PNG path when the fake screenshot provider succeeds.
- [x] 1.2 Implement path-oriented `ScreenshotCapture` metadata in `src/app_state.rs` and stop serializing `data_url` by default.
- [x] 1.3 Add a RED CLI test for `--screenshot-output <path>` resolving/writing the requested PNG path.
- [x] 1.4 Implement CLI parsing and app-state screenshot output path preflight/resolution for `--screenshot-output <path>`.
- [x] 1.5 Add a RED CLI test for invalid screenshot output parent that preserves window/accessibility diagnostics while setting `screenshot_error`.
- [x] 1.6 Implement screenshot-layer-only degradation for output path/provider/PNG validation failures.
- [x] 1.7 Add/keep regression tests proving `--no-screenshot` remains supported and no-inline.
- [x] 1.8 Add/keep screenshot-crop regression coverage proving `screenshot-crop` remains path-only and unchanged.

## 2. MCP app-state screenshot safety

- [x] 2.1 Add a RED MCP test proving `x11_get_app_state` default output has no inline screenshot blob.
- [x] 2.2 Add MCP `screenshot_output` argument support and route it through `GetAppStateParams`.
- [x] 2.3 If inline screenshot compatibility is retained, add explicit `inline_screenshot` argument/flag tests and document it as unsafe; otherwise ensure no public default path emits `data_url`.
- [x] 2.4 Verify existing `x11_get_app_state` tool name, `include_screenshot`, target selectors, and diagnostics layer behavior remain compatible.

## 3. E2E harness and controlled real-live fixture runner

- [x] 3.1 Add a RED harness test proving raw/summarized app-state evidence omits inline screenshot blobs and records screenshot path metadata.
- [x] 3.2 Update fake-live/industrial app-state calls to pass an app-state screenshot output path under the run directory and write sanitized evidence.
- [x] 3.3 Add a RED fixture runner test expecting neutral controlled fixture titles/classes that do not contain `Codex` or overlay marker strings.
- [x] 3.4 Rework `ControlledFixtureManager` title/class generation and metadata JSON to use neutral run-scoped fixture identity while keeping PID/title/class/window-id safety checks.
- [x] 3.5 Ensure GTK fixture environment metadata records `NO_AT_BRIDGE` absent and `GTK_MODULES` bridge configuration without mutating global user environment.
- [x] 3.6 Verify fixture cleanup records process termination plus target-window/overlay release state on success and failure.
- [x] 3.7 Ensure fake/fake-live fixtures remain deterministic CI evidence and are not documented as primary REAL LIVE evidence.

## 4. Documentation updates

- [x] 4.1 Add RED docs tests expecting path-only `get-app-state` wording, `--screenshot-output`, explicit unsafe inline opt-in if retained, controlled real-live fixture runner instructions, and `NO_AT_BRIDGE=1` remediation.
- [x] 4.2 Update `docs/e2e-harness.md` with safe app-state screenshot paths, controlled real-live fixture retest flow, metadata files, fake/fake-live vs real-live evidence, and safe target rules.
- [x] 4.3 Update `docs/troubleshooting.md` with app-state screenshot path diagnostics and existing `NO_AT_BRIDGE=1` presence-based remediation.
- [x] 4.4 Update `docs/release-checklist.md` to reject inline app-state screenshot blobs in durable evidence and require controlled real-live fixture evidence for industrial claims.

## 5. Verification and archive readiness

- [x] 5.1 Update `test-plan.md` Evidence Log with RED/GREEN evidence for every behavior-changing slice.
- [x] 5.2 Run targeted CLI/MCP/harness/docs tests for this change and record results.
- [x] 5.3 Run `make fmt`.
- [x] 5.4 Run `make check`.
- [x] 5.5 Run `make test`.
- [x] 5.6 Run `openspec validate --all --strict`.
- [x] 5.7 Run deterministic fake/fake-live e2e matrix validation including `validate-matrix --industrial` against generated fixture evidence.
- [x] 5.8 If a safe Cinnamon/X11 desktop is available, run controlled real-live fixture smoke and industrial validation; otherwise record the verification limitation without fabricating pass evidence.
- [x] 5.9 Confirm `git status --short` contains only intentional tracked changes and no `.secrets.local.env`, uncontrolled screenshots, or inline screenshot payload evidence is staged.
