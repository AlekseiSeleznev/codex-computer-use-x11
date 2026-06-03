## MODIFIED Requirements

### Requirement: Doctor capability detection report
The doctor JSON report MUST include structured capability facts for the local X11/EWMH desktop session, installed X11/input tools, accessibility, X11/EWMH readiness, screenshot providers, input backends, and degraded reasons. Degraded reasons MUST be exposed as an additive `readiness.degraded_reasons` array of strings so downstream automation has a stable machine-readable field distinct from blocking readiness failures. For the supported `x11-ewmh` baseline, RemoteDesktop portal and Wayland facts MUST be neutral compatibility/debug facts only when they remain serialized; they MUST NOT feed readiness blockers, degraded reasons, optional enrichments, unsupported-out-of-scope entries, or recommended next steps.

#### Scenario: Report Cinnamon X11 environment facts
- **GIVEN** the process environment contains an X11 session with Cinnamon desktop variables and an X11 display
- **WHEN** `doctor --json` returns its report
- **THEN** the report includes session or environment facts for session type, current desktop, desktop session, display, Wayland display presence, and runtime directory presence when those compatibility fields still exist
- **AND** the report identifies the environment as compatible with the Cinnamon/X11 baseline without using `cinnamon` as the backend id
- **AND** `backend` remains `x11-ewmh`
- **AND** Wayland display presence is neutral debug context and does not create a readiness degraded reason, unsupported-out-of-scope readiness issue, recommendation, or blocker

#### Scenario: Report installed desktop-control tools
- **GIVEN** `wmctrl`, `xprop`, `xdotool`, and `ydotool` are available on `PATH`
- **WHEN** `doctor --json` returns its report
- **THEN** the report includes installed-tool facts for each command
- **AND** each available command has an ok check or equivalent fact
- **AND** unavailable required X11 commands are reported as blockers or degraded reasons according to the X11 baseline instead of causing a panic
- **AND** unavailable RemoteDesktop or Wayland-specific commands are not reported as readiness degradation for the X11 baseline

#### Scenario: Report accessibility and input backend facts
- **GIVEN** AT-SPI, `/dev/uinput`, ydotool, and optional compatibility portal facts have been evaluated
- **WHEN** `doctor --json` returns its report
- **THEN** the report includes accessibility availability facts
- **AND** the accessibility facts distinguish at least AT-SPI bus reachability from whether accessibility is enabled for tree extraction
- **AND** the report includes distinct input backend facts for the X11-supported `/dev/uinput` and `ydotool` development input paths and the X11-native `xdotool` candidate availability
- **AND** RemoteDesktop portal input facts may remain serialized only as neutral compatibility/debug facts
- **AND** `readiness.can_send_development_input` is true when at least one supported X11 baseline development input backend is verified: `abs_pointer` via read/write `/dev/uinput` or `ydotool` with a connectable socket
- **AND** RemoteDesktop portal availability or absence does not make `readiness.can_send_development_input` true or false for the X11-only plugin
- **AND** X11-native `xdotool` availability is reported as a separate candidate fact and does not by itself satisfy upstream-shaped `can_send_development_input` unless a later accepted design explicitly maps it

#### Scenario: Report degraded reasons separately from blockers
- **GIVEN** one optional X11-baseline enrichment such as semantic AT-SPI tree extraction is unavailable while the required X11 baseline remains available
- **WHEN** `doctor --json` returns its report
- **THEN** `readiness.degraded_reasons` includes a string explaining the unavailable optional X11-baseline enrichment
- **AND** `readiness.blockers` includes only failures that block the current supported X11 readiness target
- **AND** `readiness.degraded_reasons` is present even when it is empty
- **AND** RemoteDesktop portal absence, `WAYLAND_DISPLAY` presence, and Wayland session facts are excluded from `readiness.degraded_reasons`

### Requirement: Doctor strict portal and screenshot facts
The doctor command MUST distinguish screenshot providers from X11 input capability and MUST NOT treat an empty successful portal introspection table as proof that a portal interface is available. For the standalone `x11-ewmh` plugin, RemoteDesktop portal probing is compatibility/debug-only if retained; the X11 readiness model MUST NOT require RemoteDesktop portal probing and MUST NOT surface RemoteDesktop portal absence as a degraded or recommended readiness signal.

#### Scenario: Accept Screenshot portal version 2 as compatibility fact
- **GIVEN** portal Screenshot introspection includes the `Screenshot` method and reports version 2
- **WHEN** `doctor --json` returns its report
- **THEN** the portal screenshot fact is available if the compatibility field remains serialized
- **AND** the report does not require version 3-only properties to mark basic screenshot availability
- **AND** the screenshot portal fact does not imply RemoteDesktop input readiness for the X11 baseline

#### Scenario: Reject empty RemoteDesktop introspection without readiness noise
- **GIVEN** portal RemoteDesktop introspection exits successfully but contains no concrete RemoteDesktop methods or properties
- **WHEN** `doctor --json` returns its report for the `x11-ewmh` baseline
- **THEN** any retained RemoteDesktop portal compatibility fact is unavailable or debug-only
- **AND** `readiness.can_send_development_input` does not become true from that empty introspection table
- **AND** `readiness.degraded_reasons` does not include `RemoteDesktop portal unavailable or incomplete`
- **AND** `readiness.optional_enrichments` does not include `remote_desktop_portal_unavailable`
- **AND** `readiness.unsupported_out_of_scope` does not include a RemoteDesktop portal readiness issue
- **AND** `readiness.recommended_next_step` does not recommend fixing or enabling RemoteDesktop portal input

#### Scenario: Report Cinnamon GNOME Shell compatible screenshot provider
- **GIVEN** `org.gnome.Shell.Screenshot` is owned by the Cinnamon process and exposes screenshot methods
- **WHEN** `doctor --json` returns its report
- **THEN** the report includes a GNOME Shell-compatible DBus screenshot provider fact
- **AND** the report keeps that provider distinct from XDG Portal Screenshot availability when both fields remain serialized
- **AND** the report does not require `gnome-shell --version` to succeed before recognizing this DBus screenshot provider

#### Scenario: Live doctor probes are non-invasive and X11-scoped
- **GIVEN** `busctl` or `gdbus` is available on `PATH`
- **WHEN** `doctor --json` gathers live non-invasive DBus diagnostics
- **THEN** it may record portal Screenshot, GNOME Shell-compatible screenshot, AT-SPI bus reachability, and compatibility RemoteDesktop facts from actual introspection or call output when the probes can run
- **AND** failed or unavailable probes become structured unavailable/debug facts rather than panics
- **AND** RemoteDesktop or Wayland probe failure does not feed X11 readiness degraded reasons, optional enrichments, unsupported-out-of-scope readiness entries, recommended next steps, or blockers
- **AND** no secret values, screenshots, or target-checkout writes are produced by those probes

### Requirement: Doctor X11 readiness taxonomy
The doctor JSON report MUST expose a stable X11 readiness taxonomy that separates blocking X11 baseline failures, acceptable degraded X11-baseline limitations, neutral compatibility/debug facts, and documented unsupported product scope without removing existing bootstrap-compatible fields. RemoteDesktop portal absence and Wayland runtime hints MUST NOT be emitted as readiness noise for the `x11-ewmh` baseline.

#### Scenario: X11 baseline remains ready with RemoteDesktop portal absent
- **GIVEN** `DISPLAY`, `wmctrl`, `xprop`, X11/EWMH window probes, AT-SPI tree extraction, and at least one supported local input backend such as `ydotool` or `/dev/uinput` are usable
- **AND** RemoteDesktop portal probing is unavailable, incomplete, or absent
- **WHEN** a developer runs `codex-computer-use-x11 doctor --json`
- **THEN** `readiness.ok` is true
- **AND** `readiness.blockers` is empty
- **AND** `readiness.degraded_reasons` is empty
- **AND** `readiness.blockers_detailed` is empty
- **AND** `readiness.optional_enrichments` does not include `remote_desktop_portal_unavailable`
- **AND** `readiness.unsupported_out_of_scope` does not include RemoteDesktop portal absence
- **AND** `readiness.recommended_next_step` does not mention RemoteDesktop, portal input, or Wayland remediation

#### Scenario: Doctor omits forbidden portal and Wayland readiness strings
- **GIVEN** the X11 baseline is otherwise ready
- **AND** RemoteDesktop portal probing is unavailable or incomplete
- **WHEN** `doctor --json` serializes readiness fields
- **THEN** `readiness.degraded_reasons`, `readiness.optional_enrichments`, `readiness.unsupported_out_of_scope`, `readiness.recommended_next_step`, and blockers do not contain `RemoteDesktop portal unavailable or incomplete`
- **AND** they do not contain `remote_desktop_portal_unavailable`
- **AND** they do not contain `wayland_runtime_out_of_scope`
- **AND** they do not contain recommendations to fix RemoteDesktop portal or Wayland for this X11 baseline

#### Scenario: WAYLAND_DISPLAY beside X11 is neutral
- **GIVEN** the session has a usable X11 `DISPLAY`
- **AND** `XDG_SESSION_TYPE` is `x11`
- **AND** `WAYLAND_DISPLAY` is present in the environment
- **AND** all required X11/EWMH, AT-SPI tree, and local input baseline checks pass
- **WHEN** `doctor --json` computes supported-scope readiness
- **THEN** the report may preserve `environment.wayland_display_present=true` as neutral debug context
- **AND** `readiness.ok` remains true
- **AND** `readiness.blockers` is empty
- **AND** `readiness.degraded_reasons` is empty
- **AND** `readiness.unsupported_out_of_scope` is empty or at least does not include `wayland_runtime_out_of_scope`
- **AND** `readiness.recommended_next_step` does not include a Wayland-specific warning or remediation

#### Scenario: X11 window probing blocker is distinct from neutral debug facts
- **GIVEN** the current environment has no usable `DISPLAY` or cannot run required X11/EWMH window probes
- **WHEN** a developer runs `codex-computer-use-x11 doctor --json`
- **THEN** `readiness.ok` is false
- **AND** `readiness.blockers` contains a machine-readable X11 window-probing blocker
- **AND** neutral RemoteDesktop, Wayland, AT-SPI, or screenshot facts do not hide the X11 blocker
