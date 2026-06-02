# x11-atspi-window-correlation Specification

## Purpose
Defines the standalone AT-SPI accessibility-tree correlation contract for resolving an X11/EWMH window to a confident semantic subtree and reporting degraded or ambiguous states without returning arbitrary UI nodes.
## Requirements
### Requirement: Window-scoped accessibility tree CLI
The standalone CLI MUST provide `accessibility-tree --window-id <id> --json` to resolve an X11/EWMH window, correlate it with AT-SPI candidates, and emit a machine-readable report. The command MUST return a subtree only when the correlation is confident and MUST otherwise return structured ambiguous or degraded diagnostics without panicking.

#### Scenario: Return correlated subtree for a confident match
- **GIVEN** the current X11 window listing contains a window with id `0x2`, reliable pid metadata, title `Preferences`, class `org.gnome.Settings`, bounds, and verified focus state
- **AND** AT-SPI candidates include one application/window subtree whose pid, name, and bounds match those window signals
- **WHEN** a developer runs `codex-computer-use-x11 accessibility-tree --window-id 0x2 --json`
- **THEN** the command exits with status code 0
- **AND** stdout is valid JSON
- **AND** `success` is `true`
- **AND** `correlation.status` is `matched`
- **AND** `correlation.confidence` is `high`
- **AND** `tree` contains AT-SPI nodes for the matched candidate
- **AND** `error_code` is null

#### Scenario: Refuse missing window before AT-SPI collection
- **GIVEN** the current X11 window listing does not contain window id `0x99`
- **WHEN** a developer runs `codex-computer-use-x11 accessibility-tree --window-id 0x99 --json`
- **THEN** the command exits with a non-zero status code
- **AND** stdout is valid JSON
- **AND** `success` is `false`
- **AND** `input_sent` is absent or `false`
- **AND** `error_code` equals `WindowNotFound`
- **AND** no AT-SPI tree command is attempted

#### Scenario: Degrade when AT-SPI is unavailable
- **GIVEN** the requested X11 window resolves from the current listing
- **AND** AT-SPI collection is unavailable on this desktop session
- **WHEN** `accessibility-tree --window-id <id> --json` is handled
- **THEN** the command exits with a non-zero status code
- **AND** `success` is `false`
- **AND** `correlation.status` is `degraded`
- **AND** `error_code` equals `AtspiUnavailable`
- **AND** `tree` is empty
- **AND** diagnostics explain the AT-SPI blocker while preserving the X11 window listing diagnostics

### Requirement: Correlation matcher confidence and ambiguity
The matcher MUST score AT-SPI candidates using multiple signals and MUST report confidence and reasons. Reliable sidecar PID metadata, title/name similarity, wm_class/app-name similarity, bounds overlap, and focused-window state MUST be independent signals. The matcher MUST NOT select an arbitrary candidate when the top candidates are ambiguous or below threshold.

#### Scenario: Reliable pid plus title produces high confidence
- **GIVEN** a target window has reliable pid `4242`, title `Editor Alpha`, wm_class `code.Code`, and bounds
- **AND** AT-SPI candidates include one candidate with pid `4242`, name `Editor Alpha`, and overlapping bounds
- **WHEN** the matcher evaluates the candidates
- **THEN** it selects that candidate
- **AND** `confidence` is `high`
- **AND** `reasons` mention reliable pid, title/name, and bounds overlap

#### Scenario: Unreliable pid requires non-PID evidence
- **GIVEN** a target window has sidecar PID reliability `unreliable`
- **AND** AT-SPI candidates include one candidate whose pid equals the raw window pid but whose title/class/bounds do not match
- **AND** another candidate has matching title/class/bounds but a different pid
- **WHEN** the matcher evaluates the candidates
- **THEN** it selects the title/class/bounds candidate when the non-PID score reaches the threshold
- **AND** `confidence` is `medium`
- **AND** `reasons` state that PID was not treated as reliable evidence

#### Scenario: Ambiguous candidates are refused
- **GIVEN** two AT-SPI candidates have equivalent title/class/bounds evidence for the same target window
- **WHEN** the matcher evaluates the candidates
- **THEN** no subtree is returned
- **AND** `correlation.status` is `ambiguous`
- **AND** `success` is `false`
- **AND** `error_code` equals `AmbiguousAccessibilityMatch`
- **AND** diagnostics include the candidate object references and scores needed for disambiguation

#### Scenario: Browser multi-process candidate can match without PID
- **GIVEN** a browser window's X11 pid differs from the AT-SPI application pid
- **AND** the window title, class/app name, and bounds overlap identify exactly one AT-SPI candidate
- **WHEN** the matcher evaluates the candidates
- **THEN** it may select that candidate with `medium` confidence
- **AND** `reasons` include non-PID evidence
- **AND** the report does not claim a reliable PID match

#### Scenario: Terminal child process pid is not mistaken for the semantic owner
- **GIVEN** a terminal window has a terminal app/window pid and a foreground child process pid
- **AND** AT-SPI exposes the terminal application/window node rather than the foreground child process
- **WHEN** the matcher evaluates the candidates
- **THEN** it prefers the terminal application/window evidence over the child process pid alone
- **AND** it reports medium or high confidence only when title/class/bounds or terminal context corroborate the match

### Requirement: AT-SPI subtree report shape
The accessibility-tree report MUST include stable, automation-friendly fields for the selected window, correlation result, candidate diagnostics, and tree nodes. Tree nodes MUST include object reference, role, name, optional bounds, states, actions, value/editability indicators when available, depth, parent index, and child count. The report MUST avoid secret values and unrelated local environment data.

#### Scenario: Report includes match diagnostics
- **GIVEN** an AT-SPI correlation attempt has at least one candidate
- **WHEN** the CLI or MCP report is serialized
- **THEN** the JSON includes `project`, `version`, `backend`, `success`, `window`, `correlation`, `tree`, `error_code`, `note`, and `diagnostics`
- **AND** `correlation` includes `status`, `confidence`, `score`, `reasons`, and the matched object reference when present
- **AND** diagnostics include candidate summaries with scores but not secret values

#### Scenario: Tree size is bounded
- **GIVEN** the matched AT-SPI subtree is large
- **WHEN** the report is built
- **THEN** the implementation applies documented node and depth limits
- **AND** diagnostics state whether the tree was truncated
- **AND** the command remains responsive enough for MCP smoke testing

### Requirement: Class and app token matching avoids substring false positives
The AT-SPI matcher MUST compare short X11 class/app tokens using exact or token-boundary semantics rather than arbitrary substring matching. Short tokens such as `tk` MUST NOT match unrelated names such as `gtk`, `gtk3`, `ibus-ui-gtk3`, or `xdg-desktop-portal-gtk` solely by substring. Candidate reasons MUST show whether class/app evidence matched, did not match, or was missing.

#### Scenario: Tk does not match gtk3
- **GIVEN** a target X11 window has `WM_CLASS` of `Tk`
- **AND** an AT-SPI candidate name is `ibus-ui-gtk3`
- **WHEN** the matcher evaluates class/app evidence
- **THEN** the candidate does not receive class/app match score for `Tk`
- **AND** its reasons do not claim `wm_class/app name matched`
- **AND** the report may list the candidate only with non-matching score reasons

#### Scenario: Exact app token may match
- **GIVEN** a target X11 window has app id `org.gnome.Settings`
- **AND** an AT-SPI candidate name token is `org.gnome.Settings`
- **WHEN** the matcher evaluates class/app evidence
- **THEN** the candidate may receive class/app match score
- **AND** the reason identifies an exact or token-boundary match

### Requirement: Target-scoped xprop enrichment augments correlation only for requested windows
When AT-SPI correlation runs for a resolved target window, the implementation MUST be able to run bounded `xprop -id <target>` enrichment for that single target to capture `_NET_WM_PID`, `WM_CLIENT_MACHINE`, `WM_NAME`, `_NET_WM_NAME`, `WM_CLASS`, and `_NET_WM_WINDOW_TYPE`. Normal `list-windows` MUST NOT spawn unbounded per-window `xprop` calls across all windows.

#### Scenario: Accessibility correlation enriches one target
- **GIVEN** the current X11 listing resolves target window `0x2`
- **AND** `xprop -id 0x2` is available
- **WHEN** `accessibility-tree --window-id 0x2 --json` builds correlation inputs
- **THEN** diagnostics record one target-scoped xprop enrichment attempt
- **AND** the enrichment includes any parsed target `_NET_WM_PID`, names, class, machine, and window type values
- **AND** those fields may contribute to candidate score reasons when present

#### Scenario: List windows does not do unbounded xprop enrichment
- **GIVEN** the desktop has many listed windows
- **WHEN** `list-windows --json` runs without a target-scoped enrichment request
- **THEN** diagnostics keep normal per-window xprop listing disabled
- **AND** the command does not spawn `xprop -id` once per listed window
- **AND** diagnostics explain that target-scoped enrichment is available through correlation paths

### Requirement: Correlation diagnostics expose missing signals and score reasons
AT-SPI correlation reports MUST include enough per-candidate diagnostics to explain why no candidate matched, including score reasons and missing target/candidate signals. Missing reliable PID, missing AT-SPI bounds, missing title/name, missing class/app evidence, and missing focus evidence MUST be explicit where they affect confidence.

#### Scenario: No match reports missing signals
- **GIVEN** a target window resolves with unreliable pid metadata
- **AND** AT-SPI candidates exist but no candidate reaches the medium-confidence threshold
- **WHEN** `accessibility-tree --window-id <target> --json` emits a no-match report
- **THEN** `success` is false
- **AND** `error_code` equals `NoAccessibilityMatch`
- **AND** candidate diagnostics include scores, reasons, and `missing_signals`
- **AND** no arbitrary subtree is returned

#### Scenario: Bounds-only evidence does not select a subtree
- **GIVEN** a target window and one AT-SPI candidate have overlapping bounds
- **AND** reliable pid, title/name, class/app, and focus evidence are missing or contradictory
- **WHEN** the matcher evaluates the candidate
- **THEN** the candidate does not reach matched confidence solely from bounds
- **AND** the report includes a degraded/no-match diagnostic rather than a subtree

### Requirement: Live fixtures separate Tk limitations from AT-SPI-positive GTK evidence
The live e2e fixture set MUST include at least one GTK application/window that is expected to expose useful AT-SPI semantics and MUST keep Tkinter windows for keyboard and pointer safety checks. Reports and docs MUST state that Tk/Tkinter may be limited for AT-SPI and that a Tk no-match is not enough evidence that AT-SPI correlation is broken.

#### Scenario: GTK fixture provides positive AT-SPI acceptance
- **GIVEN** live mode starts a GTK safe fixture with a stable title and accessible button or entry
- **WHEN** `x11_accessibility_tree` targets the GTK fixture
- **THEN** the report matches a high- or medium-confidence AT-SPI subtree
- **AND** the subtree includes at least one expected accessible node
- **AND** capability matrix AT-SPI status may pass for the GTK fixture

#### Scenario: Tk fixture remains a safe keyboard and pointer fixture
- **GIVEN** live mode starts Tkinter safe text, button, or canvas windows
- **WHEN** AT-SPI correlation cannot match those Tk windows
- **THEN** the capability matrix records the Tk limitation as fixture-specific degraded evidence
- **AND** keyboard and pointer checks can still pass against those windows
- **AND** the harness does not lower the matcher threshold to bounds-only to make Tk pass

### Requirement: Industrial live AT-SPI uses GTK bridge fixture
Industrial live verification MUST treat a GTK fixture launched with accessibility bridge environment as the semantic AT-SPI pass path. The fixture subprocess MUST remove `NO_AT_BRIDGE` from its environment rather than setting it to `0`, because the disabling contract is presence-based for bridge suppression in common GTK/ATK bridge integrations. Tk/Tkinter AT-SPI no-match MAY be recorded as expected degraded fixture evidence, but MUST NOT be used as the only live AT-SPI acceptance signal and MUST NOT cause the matcher to relax confidence rules.

#### Scenario: GTK bridge environment is recorded
- **GIVEN** live smoke starts the GTK AT-SPI fixture
- **WHEN** the harness records fixture metadata
- **THEN** evidence includes that `GTK_MODULES=gail:atk-bridge` was set for the fixture process when required by the desktop environment
- **AND** evidence includes that `NO_AT_BRIDGE` was absent from the fixture process environment
- **AND** evidence includes the fixture title, process id when available, and selected window id

#### Scenario: GTK tree pass includes expected semantic node
- **GIVEN** the GTK fixture is ready and selected as the target window
- **WHEN** `x11_accessibility_tree` runs against the fixture
- **THEN** the report matches a high- or medium-confidence subtree
- **AND** the tree contains an expected role, name, action, or value from the fixture
- **AND** the AT-SPI capability row references that tree evidence as the pass reason

#### Scenario: Tk no-match is not the pass path
- **GIVEN** the Tk fixture is used for keyboard and pointer checks
- **WHEN** `x11_accessibility_tree` returns `NoAccessibilityMatch` for the Tk fixture
- **THEN** evidence records the Tk AT-SPI result as fixture-specific degraded evidence
- **AND** the industrial AT-SPI pass still requires the GTK bridge fixture or a documented accessible equivalent

### Requirement: AT-SPI diagnostics use canonical match outcome codes
AT-SPI correlation diagnostics MUST use canonical outcome codes that distinguish bus/probe availability, extraction, no-match, ambiguous-match, and fixture-backed pass states.

#### Scenario: Canonical outcomes cover every AT-SPI probe result
- **GIVEN** an AT-SPI accessibility-tree or app-state request is evaluated
- **WHEN** the result is serialized in CLI, MCP, app-state, doctor, or e2e evidence
- **THEN** the outcome is one of a documented canonical set covering bus unavailable, bus reachable, tree extraction unavailable, no matching subtree, ambiguous match, and matched subtree
- **AND** every non-pass outcome includes a reason and next diagnostic hint
- **AND** low-confidence or ambiguous data is not normalized into a successful subtree

#### Scenario: Controlled GTK fixture pass is distinguishable from live ambient success
- **GIVEN** live smoke starts a controlled GTK AT-SPI fixture with a unique title or class
- **WHEN** `x11_accessibility_tree` returns the expected role or name from that fixture
- **THEN** the evidence records a controlled-fixture pass outcome
- **AND** the fixture id, target window id, and correlation signals are included in sanitized evidence
- **AND** ambient user application windows are not used as fallback pass evidence

### Requirement: Cinnamon X11 recommendations are actionable
AT-SPI degraded diagnostics MUST include recommendations that are specific enough for Cinnamon/X11 troubleshooting without making AT-SPI mandatory for the X11 window/input baseline.

#### Scenario: Missing bridge produces setup recommendation
- **GIVEN** AT-SPI bus reachability or tree extraction is degraded because bridge support or accessibility enablement appears missing
- **WHEN** diagnostics are emitted
- **THEN** the recommendation identifies the likely setup category
- **AND** it avoids claiming a code failure without evidence
- **AND** it states whether the X11 baseline remains usable without semantic accessibility enrichment

### Requirement: AT-SPI bridge-disabled state is canonical
AT-SPI diagnostics emitted by accessibility-tree, app-state, doctor, and e2e evidence MUST use a canonical bridge-disabled outcome when the AT-SPI bus is reachable but a GTK/ATK bridge-disabling environment prevents useful tree extraction.

#### Scenario: Bridge-disabled outcome is not collapsed into generic unavailable
- **GIVEN** an AT-SPI probe reaches the accessibility bus
- **AND** the probed process environment has `NO_AT_BRIDGE` present
- **AND** no usable GTK application tree is exposed
- **WHEN** the probe serializes its diagnostic state
- **THEN** the outcome is `atspi_gtk_bridge_disabled_by_environment`
- **AND** the report preserves `atspi_bus_available=true` and `tree_available=false`
- **AND** no arbitrary AT-SPI subtree is returned as a pass

#### Scenario: Safe degradation preserves X11 context
- **GIVEN** an X11/EWMH window target resolves successfully
- **AND** AT-SPI diagnostics report `atspi_gtk_bridge_disabled_by_environment`
- **WHEN** `x11_accessibility_tree` or `x11_get_app_state` emits a report
- **THEN** the report includes the resolved X11 target diagnostics
- **AND** the AT-SPI layer is degraded with `reason_category=environment_limitation`
- **AND** the report does not send input, pointer, overlay, screenshot, or app-state operations to an uncontrolled real user window as fallback

### Requirement: AT-SPI collector probe exposes canonical doctor facts
The AT-SPI correlation layer MUST provide a lightweight probe result that doctor can consume without duplicating or diverging from collector logic. The probe result MUST include canonical match outcome, candidate count, whether any tree was obtainable, whether a controlled fixture passed, and sanitized degraded reasons. A successful collector output with one or more candidates MUST be interpreted the same way for doctor probing and for window-scoped accessibility-tree collection.

#### Scenario: Probe reports obtainable tree without requiring a target selector
- **GIVEN** the AT-SPI bus is reachable
- **AND** the collector can enumerate application or window candidates
- **WHEN** the lightweight probe runs for doctor diagnostics
- **THEN** the probe returns `tree_available=true`
- **AND** it returns `candidate_count` equal to the number of candidate roots considered
- **AND** it returns a canonical `match_outcome` value

#### Scenario: Probe uses the same collector success contract as accessibility tree
- **GIVEN** the collector process returns valid JSON with `ok=true`
- **AND** the JSON contains one or more AT-SPI candidates that the accessibility-tree path can score
- **WHEN** the lightweight doctor probe parses that collector output
- **THEN** it treats the collector as available
- **AND** it reports `match_outcome=tree_available`
- **AND** it does not collapse the result to `collector_unavailable`

#### Scenario: Controlled fixture pass is distinguishable from ambient candidates
- **GIVEN** a controlled GTK fixture is configured for live validation
- **AND** the collector finds the expected fixture semantic node
- **WHEN** the lightweight probe runs
- **THEN** the probe records `controlled_fixture_pass=true`
- **AND** it still reports the ambient candidate count separately
- **AND** the report does not expose uncontrolled user-window text beyond sanitized diagnostics

#### Scenario: Probe preserves bridge-disabled reason
- **GIVEN** the process or activation environment contains `NO_AT_BRIDGE=1`
- **WHEN** the lightweight probe evaluates AT-SPI readiness
- **THEN** the probe returns a bridge-disabled canonical outcome
- **AND** it does not collapse the result into a generic tree-unavailable state

