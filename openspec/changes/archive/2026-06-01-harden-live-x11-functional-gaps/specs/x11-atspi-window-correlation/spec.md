## ADDED Requirements

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
