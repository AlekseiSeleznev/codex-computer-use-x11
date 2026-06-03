# doctor-cli Specification Delta

## MODIFIED Requirements

### Requirement: Doctor strict portal and screenshot facts
The doctor command MUST distinguish screenshot providers from RemoteDesktop input capability, MUST NOT treat an empty successful portal introspection table as proof that a portal interface is available, and MUST gather the live non-invasive portal/screenshot/AT-SPI probe outputs needed by app-state diagnostics when those commands are available.

#### Scenario: Accept Screenshot portal version 2
- **GIVEN** portal Screenshot introspection includes the `Screenshot` method and reports version 2
- **WHEN** `doctor --json` returns its report
- **THEN** the portal screenshot fact is available
- **AND** the report does not require version 3-only properties to mark basic screenshot availability

#### Scenario: Reject empty RemoteDesktop introspection
- **GIVEN** portal RemoteDesktop introspection exits successfully but contains no concrete RemoteDesktop methods or properties
- **WHEN** `doctor --json` returns its report
- **THEN** the portal RemoteDesktop input fact is unavailable
- **AND** `readiness.can_send_development_input` does not become true from that empty introspection table
- **AND** the report includes a degraded reason or check detail explaining that no RemoteDesktop methods were found

#### Scenario: Report Cinnamon GNOME Shell compatible screenshot provider
- **GIVEN** `org.gnome.Shell.Screenshot` is owned by the Cinnamon process and exposes screenshot methods
- **WHEN** `doctor --json` returns its report
- **THEN** the report includes a GNOME Shell-compatible DBus screenshot provider fact
- **AND** the report keeps that provider distinct from XDG Portal Screenshot availability
- **AND** the report does not require `gnome-shell --version` to succeed before recognizing this DBus screenshot provider

#### Scenario: Live doctor probes feed app-state diagnostics
- **GIVEN** `busctl` or `gdbus` is available on `PATH`
- **WHEN** `doctor --json` gathers live non-invasive DBus diagnostics
- **THEN** it records RemoteDesktop, portal Screenshot, GNOME Shell-compatible screenshot, and AT-SPI bus reachability from actual introspection or call output when the probes can run
- **AND** failed or unavailable probes become structured unavailable/degraded facts rather than panics
- **AND** no secret values, screenshots, or target-checkout writes are produced by those probes
