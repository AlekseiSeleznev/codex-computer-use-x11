## 1. Coordinate and bounds reporting

- [x] 1.1 RED: Add focused public CLI test for `window-bounds --window-id <id> --json` preserving signed root coordinates and coordinate metadata.
- [x] 1.2 GREEN: Implement minimal coordinate/bounds module and CLI parsing/reporting for `window-bounds`.
- [x] 1.3 REFACTOR: Reuse existing `list_windows`/JSON report patterns without duplicating command serialization logic unnecessarily.
- [x] 1.4 RED: Add focused public CLI test for `xwininfo` alternate bounds disagreement diagnostics.
- [x] 1.5 GREEN: Add optional `xwininfo` probing/parsing and bounds provenance diagnostics while preserving `wmctrl` primary bounds.

## 2. Crop validation and screenshot provider boundary

- [x] 2.1 RED: Add focused public CLI test proving `screenshot-crop` refuses a targeted crop outside target bounds before provider invocation.
- [x] 2.2 GREEN: Implement crop rectangle parsing/validation, target-bounds checks, and structured refusal reports.
- [x] 2.3 RED: Add focused public CLI test proving default full-window crop invokes fake `gdbus` `ScreenshotArea` with the exact validated rectangle and output path.
- [x] 2.4 GREEN: Implement standalone GNOME Shell-compatible `gdbus ScreenshotArea` provider boundary and metadata-only JSON report.
- [x] 2.5 RED: Add parser/validator unit test for negative `xrandr --listmonitors` offsets and root-screen crop clamping.
- [x] 2.6 GREEN: Implement screen geometry parsing from `xrandr`, fallback `xdpyinfo`, and crop-to-screen intersection diagnostics.

## 3. Architecture, docs, and evidence

- [x] 3.1 Update README/docs to document `window-bounds`, `screenshot-crop`, X11 root coordinates, bounds provenance, and screenshot metadata/no-data-url behavior.
- [x] 3.2 Update `test-plan.md` evidence log with RED/GREEN command outcomes for every TDD slice.
- [x] 3.3 Run focused live smoke for `window-bounds` and `screenshot-crop` on the local Cinnamon/X11 session, using a temporary output file that is deleted after existence/metadata checks.

## 4. Verification and archive readiness

- [x] 4.1 Run `openspec validate add-x11-screenshot-coordinate-model --strict --no-interactive`.
- [x] 4.2 Run `make fmt`.
- [x] 4.3 Run `make check`.
- [x] 4.4 Run `make test`.
- [x] 4.5 Confirm no target checkout writes, no secret files staged, and no unrelated dirty state.
