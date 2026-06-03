## ADDED Requirements

### Requirement: Standalone X11 row in Computer Use settings
Codex Desktop Linux MUST render an `X11 Computer Use` plugin control row on the `Settings -> Computer use` page when the local plugin catalog exposes plugin id `codex-computer-use-x11`. The row MUST use the normal plugin settings install/enable controls and MUST NOT require the plugin to be renamed to `computer-use` or installed under `openai-bundled`.

#### Scenario: Render X11 row from installed local marketplace plugin
- **GIVEN** the Codex Desktop plugin catalog includes `codex-computer-use-x11@codex-computer-use-x11`
- **AND** the plugin interface metadata has display name `X11 Computer Use`
- **WHEN** the user opens `Settings -> Computer use`
- **THEN** the Control section includes an `X11 Computer Use` row
- **AND** the row is backed by the `codex-computer-use-x11` plugin object
- **AND** the row can be installed, enabled, disabled, or opened using the same settings plugin control component as other Computer Use rows

#### Scenario: Preserve bundled Computer Use and Chrome rows
- **GIVEN** the bundled `computer-use` plugin and Chrome plugin are available
- **AND** the local `codex-computer-use-x11` plugin is available
- **WHEN** the settings page builds its Control rows
- **THEN** the existing bundled `Any App` row is still based on plugin id `computer-use`
- **AND** the existing `Google Chrome` row is still based on `chrome`, `chrome-dev`, or `chrome-internal`
- **AND** the new `X11 Computer Use` row is additional rather than a replacement

#### Scenario: No masquerading as openai bundled Computer Use
- **GIVEN** `codex-computer-use-x11` is installed in the user's local marketplace
- **WHEN** the Codex Desktop launcher syncs bundled marketplaces
- **THEN** it MUST NOT overwrite `$CODEX_HOME/plugins/cache/openai-bundled/computer-use`
- **AND** it MUST NOT rewrite the bundled marketplace plugin name `computer-use` to `codex-computer-use-x11`
- **AND** the standalone plugin remains under its owned namespace `codex-computer-use-x11`

### Requirement: Settings webview patch is safe and idempotent
The source-overlay patch for Codex Desktop Linux MUST modify only the recognized Computer Use settings webview bundle shape, MUST be idempotent, and MUST fail soft with a clear warning when upstream minified anchors drift.

#### Scenario: Patch current minified settings bundle once
- **GIVEN** a current Codex Desktop webview asset contains the Computer Use settings page with hardcoded `computer-use` and Chrome plugin lookups
- **WHEN** the Linux patcher runs with Computer Use UI enabled
- **THEN** the asset is patched to look up `codex-computer-use-x11`
- **AND** the patched row title includes `X11 Computer Use`
- **AND** the patched source contains only one row-injection marker after one patch run

#### Scenario: Re-running patcher does not duplicate row
- **GIVEN** the Computer Use settings webview asset is already patched for `codex-computer-use-x11`
- **WHEN** the Linux patcher runs again
- **THEN** the resulting asset remains byte-for-byte equivalent or semantically unchanged for the injected row
- **AND** there is still exactly one `codex-computer-use-x11` lookup for the settings row

#### Scenario: Warn and skip on unexpected settings page shape
- **GIVEN** a webview asset mentions `computer-use` but does not contain the expected settings row construction pattern
- **WHEN** the Linux patcher evaluates that asset
- **THEN** it MUST leave the asset unchanged
- **AND** it MUST warn that the X11 Computer Use settings row patch was skipped
- **AND** other Linux Computer Use UI patches may continue according to their existing fail-soft policy

### Requirement: Verification covers source overlay and real UI readiness
The change MUST provide automated evidence for the patcher and script smoke boundary, and MUST record whether the real Codex Desktop UI can be visually verified on the current machine.

#### Scenario: Patcher tests prove row lookup and idempotence
- **GIVEN** the target checkout contains patcher unit tests
- **WHEN** the verification command for the patcher runs
- **THEN** tests prove the row is injected for `codex-computer-use-x11`
- **AND** tests prove re-running the patch does not duplicate the row
- **AND** tests prove unrelated bundles are not changed

#### Scenario: Real UI verification is recorded
- **GIVEN** the patched Codex Desktop Linux app can be rebuilt or the extracted app asset can be patched locally
- **WHEN** verification reaches the real UI smoke boundary
- **THEN** the result records whether `Settings -> Computer use` shows `X11 Computer Use`
- **AND** if live visual verification is unavailable, the result records the exact blocker and keeps automated patcher evidence as the fallback
