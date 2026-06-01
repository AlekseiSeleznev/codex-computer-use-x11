## ADDED Requirements

### Requirement: X11 root coordinate model
The standalone X11/EWMH implementation MUST define window bounds, pointer coordinates, and screenshot crop rectangles in global/root X11 pixel coordinates. Known `x` and `y` positions MUST stay signed (`Option<i32>` in the upstream-compatible model), unknown positions MUST be represented as absent/null, and width/height MUST remain unsigned positive dimensions.

#### Scenario: Preserve negative monitor coordinates
- **GIVEN** an X11 window listing source reports a window at `x = -1280`, `y = 24`, `width = 1000`, and `height = 700`
- **WHEN** the standalone code serializes the window's bounds in a JSON report
- **THEN** `bounds.x` is `-1280`
- **AND** `bounds.y` is `24`
- **AND** `bounds.width` is `1000`
- **AND** `bounds.height` is `700`
- **AND** no coordinate is converted to an unsigned wraparound value or a sentinel `0`

#### Scenario: Preserve unknown coordinates as null
- **GIVEN** a bounds source can identify a window's size but cannot identify its root position
- **WHEN** the standalone code serializes the upstream-compatible bounds
- **THEN** `bounds.x` is null
- **AND** `bounds.y` is null
- **AND** `bounds.width` and `bounds.height` remain positive unsigned dimensions
- **AND** diagnostics explain why the position is unknown

### Requirement: Window bounds CLI report
The standalone CLI MUST provide `window-bounds --window-id <id> --json` to resolve one current X11/EWMH window and return its bounds, coordinate-model metadata, and geometry provenance. The report MUST keep `backend` equal to `x11-ewmh`, MUST avoid secret values, and MUST make `wmctrl` frame/client uncertainty explicit instead of silently claiming client-content bounds.

#### Scenario: Report bounds with coordinate metadata
- **GIVEN** `wmctrl -lpGx` lists window `0x2` with bounds `10,20 800x600`
- **AND** `_NET_ACTIVE_WINDOW` reports `0x2`
- **WHEN** a developer runs `codex-computer-use-x11 window-bounds --window-id 0x2 --json`
- **THEN** the command exits with status code 0
- **AND** stdout is valid JSON
- **AND** `success` is `true`
- **AND** `window.window_id` is `2`
- **AND** `bounds.x` is `10`
- **AND** `bounds.y` is `20`
- **AND** `coordinate_model.space` is `x11_root_global_pixels`
- **AND** `diagnostics.primary_source` mentions `wmctrl -lpGx`
- **AND** `diagnostics.bounds_semantics` warns that frame and client-area geometry can differ

#### Scenario: Report xwininfo disagreement without changing primary bounds silently
- **GIVEN** `wmctrl -lpGx` reports window `0x2` at `3840,0 1920x1040`
- **AND** `xwininfo -id 0x2` reports absolute upper-left `1920,0` and size `1920x1040`
- **WHEN** `window-bounds --window-id 0x2 --json` builds the report
- **THEN** `bounds.x` remains the primary `wmctrl` value `3840`
- **AND** `diagnostics.alternate_sources` includes an `xwininfo` entry with `x = 1920`
- **AND** `diagnostics.bounds_agree` is `false`
- **AND** `diagnostics.degraded_reasons` explains that geometry sources disagreed

#### Scenario: Refuse missing window id
- **GIVEN** the current X11 window listing does not contain window id `0x99`
- **WHEN** a developer runs `codex-computer-use-x11 window-bounds --window-id 0x99 --json`
- **THEN** the command exits with a non-zero status code
- **AND** stdout is valid JSON
- **AND** `success` is `false`
- **AND** `error_code` equals `WindowNotFound`
- **AND** no screenshot provider command is attempted

### Requirement: Crop rectangle validation
Screenshot crop requests MUST validate finite global/root X11 crop rectangles before provider invocation. Targeted crops MUST be inside the selected window bounds, dimensions MUST be positive, screen geometry MUST support negative monitor offsets when the display reports them, and any clamp or refusal MUST be represented in structured diagnostics.

#### Scenario: Default crop uses full target bounds
- **GIVEN** `wmctrl -lpGx` lists window `0x2` with bounds `10,20 800x600`
- **AND** the display geometry contains that rectangle
- **WHEN** a developer runs `codex-computer-use-x11 screenshot-crop --window-id 0x2 --output /tmp/window.png --json` without explicit crop coordinates
- **THEN** the validated crop rectangle is `x = 10`, `y = 20`, `width = 800`, `height = 600`
- **AND** `crop.source` is `window_bounds`
- **AND** the screenshot provider receives that exact rectangle if provider invocation proceeds

#### Scenario: Reject crop outside target bounds before provider invocation
- **GIVEN** `wmctrl -lpGx` lists window `0x2` with bounds `10,20 100x100`
- **AND** the caller requests crop rectangle `x = 0`, `y = 20`, `width = 50`, `height = 50`
- **WHEN** `screenshot-crop --window-id 0x2 ... --json` validates the request
- **THEN** the command exits with a non-zero status code
- **AND** stdout is valid JSON
- **AND** `success` is `false`
- **AND** `error_code` equals `CropOutsideTargetBounds`
- **AND** `screenshot_invoked` is `false`
- **AND** no DBus or external screenshot command is attempted

#### Scenario: Reject non-positive crop dimensions
- **GIVEN** a requested crop rectangle has `width = 0` or `height = 0`
- **WHEN** `screenshot-crop` validates the request
- **THEN** the command exits with a non-zero status code
- **AND** `success` is `false`
- **AND** `error_code` equals `InvalidCropRect`
- **AND** diagnostics identify the invalid dimension

#### Scenario: Clamp to reported root screen when target is partially offscreen
- **GIVEN** a display geometry spans root rectangle `x = -1280`, `y = 0`, `width = 3200`, and `height = 1080`
- **AND** a target window's known bounds overlap that screen but extend beyond one edge
- **WHEN** the crop validator builds a provider rectangle
- **THEN** the provider rectangle is clamped to the intersection with the root screen
- **AND** `crop.clamped` is `true`
- **AND** diagnostics include the requested and provider rectangles

### Requirement: Screenshot crop provider boundary
The standalone screenshot-crop command MUST treat the existing Codex screenshot provider model as primary integration guidance and MAY use a GNOME Shell-compatible DBus `ScreenshotArea` command as the standalone live-smoke provider. It MUST not emit screenshot pixels or data URLs by default, MUST write only to the caller-provided output path, and MUST report provider availability or failure as structured JSON.

#### Scenario: Invoke GNOME Shell-compatible ScreenshotArea with validated crop
- **GIVEN** crop validation succeeds for rectangle `x = 10`, `y = 20`, `width = 800`, and `height = 600`
- **AND** `gdbus` is available on `PATH`
- **WHEN** `screenshot-crop --window-id 0x2 --output /tmp/window.png --json` invokes the provider
- **THEN** the provider command targets `org.gnome.Shell.Screenshot`
- **AND** it calls `org.gnome.Shell.Screenshot.ScreenshotArea` with `10 20 800 600 false /tmp/window.png`
- **AND** the JSON report includes `screenshot_invoked` equal to `true`
- **AND** `output_path` equals the caller-provided path
- **AND** the report does not include screenshot pixel data or a data URL

#### Scenario: Report provider unavailable without losing crop diagnostics
- **GIVEN** crop validation succeeds
- **AND** no supported standalone screenshot provider command is available
- **WHEN** `screenshot-crop` handles the request
- **THEN** stdout is valid JSON
- **AND** `success` is `false`
- **AND** `error_code` equals `ScreenshotProviderUnavailable`
- **AND** `crop` still contains the validated provider rectangle
- **AND** diagnostics mention that Codex Desktop Linux should prefer its existing screenshot provider when integrating this behavior

#### Scenario: Keep screenshot capability separate from input capability
- **GIVEN** the desktop exposes `org.gnome.Shell.Screenshot` or `org.freedesktop.portal.Screenshot.Screenshot`
- **AND** portal RemoteDesktop is absent or has an empty introspection table
- **WHEN** doctor and screenshot-coordinate reports describe provider readiness
- **THEN** screenshot availability is reported independently from pointer or keyboard input readiness
- **AND** RemoteDesktop absence does not hide a working screenshot provider
- **AND** screenshot readiness does not require `gnome-shell --version` to succeed on Cinnamon
