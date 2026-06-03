# Project Context Glossary

This file captures project language for `intent-driven-codex`. It is a glossary
only: no implementation details, no project policy, no architecture decisions,
and no secret values.

## Terms

### Intent-Driven Codex

A Codex-native overlay that helps a project run Intent-Driven Development with
OpenSpec as the lifecycle engine and source of truth.

### OpenSpec

The lifecycle engine used to create, validate, apply, and archive change
artifacts. OpenSpec orders artifacts and validates specs; it does not enforce
Codex-only project context such as the constitution or local secrets.

### Codex overlay

The project-local `.codex/prompts` and `.codex/skills` layer that tells Codex how
to operate OpenSpec workflows, project context preflight, grill gates, TDD,
verification, Git checkpoints, and installation checks.

### Project constitution

The root `CONSTITUTION.md` file. It contains project rules, required
technologies, MCP/external-system guidance, secret-handling policy,
documentation sources, verification rules, and additional AI instructions.

### Architecture snapshot

The root `ARCHITECTURE.md` file. It summarizes the current architecture state and
links to in-force ADRs. Durable rationale remains in `adr/`.

### Grill gate

A mandatory Matt `grill-with-docs` artifact gate that resolves material
uncertainty by reading context first and then asking one focused question at a
time only when needed. In v0.1.3, `grill.md` runs before design and
`design-review.md` runs after design.

### TDD slice

One vertical RED -> GREEN -> REFACTOR cycle through observable behavior and a
public interface. It is not a horizontal batch of all tests followed by all code.

### Design review gate

The post-design Matt `grill-with-docs` gate recorded in `design-review.md`. It
stress-tests the completed design before ADR and test planning.

### x11-ewmh

The canonical project term and backend label for the generic X11/EWMH window-control path. It is distinct from Cinnamon-specific validation and from a window's `client_type`.

### Standalone plugin

A delivery path where this project is validated as its own CLI/MCP integration before code is adapted into the Codex Desktop Linux target.

### Source overlay

A delivery path where code or concepts from this project are adapted into the local Codex Desktop Linux target checkout for future upstream-style integration.

### Overlay drift

A source-overlay state where project-owned marker blocks, generated files, target anchors, or recorded baseline metadata no longer agree with what the overlay installer expects. Drift is a safety status: Codex should report it and avoid blind install/uninstall assumptions until the inconsistency is repaired or explicitly handled.

### Active window

The window-manager-reported top-level window that is currently active for user interaction. In this project, active-window identity is the observable focus fact used to decide whether a requested X11 window actually became focused.

### Focus verification

A safety check that compares the requested target window with the freshly observed active window before any later targeted input path treats the target as safe.

### FocusNotVerified

A machine-readable failure outcome meaning a focus attempt did not prove that the requested window became the active window. It is a safe degraded result, not proof that the window manager or parser is broken.
### AT-SPI window correlation

The process of matching an `x11-ewmh` `WindowInfo` record to one AT-SPI application/window subtree using multiple signals such as reliable PID metadata, title/name, wm_class/app name, bounds overlap, and focus state. A confident match can feed semantic UI context; an ambiguous or degraded match must not return an arbitrary subtree.

### Accessibility tree

A structured AT-SPI view of application UI elements, including roles, names, bounds, states, actions, values, and parent/child relationships. In this project it is semantic context for a selected window, not proof that input is safely targeted.


### X11 root coordinates

Global pixel coordinates in the X11 root window space. In this project they are the canonical coordinate space for window bounds, pointer points, screenshot crop rectangles, and future screenshot/window context composition.

### Crop rectangle

A finite rectangular region in X11 root coordinates, represented by signed `x`/`y` origin plus positive `width`/`height`, that can be validated against a target window and display geometry before screenshot capture.

### Bounds provenance

The recorded source and confidence context for a window's geometry, such as `wmctrl -lpGx` primary bounds, optional `xwininfo` alternate bounds, and whether frame/client ambiguity or source disagreement was observed.

### App state

The composed Computer Use read model that brings together current window context, screenshot data or screenshot diagnostics, accessibility tree data or accessibility diagnostics, capability diagnostics, and a short agent-facing message for one state-reading turn.

### Layer-degraded app state

An app state response where one layer, such as target window resolution, screenshot capture, or AT-SPI correlation, failed or was ambiguous while other layers still returned usable facts. Layer degradation must be reported in that layer's error/diagnostic field instead of fabricating missing data or failing the whole read when JSON can still be emitted.

### Target window

An application window explicitly selected for the current automation task. A target window is a session context hint for subsequent inspection or actions, not proof that the window is still present or focused.

### Window group

A named collection of target windows used to keep multi-window tasks understandable, with one active target window at a time for that group.

### Overlay window

A project-owned visual indicator, such as a colored border, that may help a user see which window is targeted. Overlay windows are not application targets and must not be treated as safe targets for automation.

### Stale target

A previously saved target window whose underlying desktop window can no longer be found in the current listing. A stale target is historical context only and must be refreshed or released before targeted use.

### E2E harness

A repeatable end-to-end smoke boundary that produces machine-readable evidence from the Codex-facing installation or target-source delivery path. In this project, the E2E harness has fake no-GUI mode for deterministic CI evidence and live mode for Cinnamon/X11 evidence.

### Capability matrix evidence

A per-capability record showing whether each required v1 Computer Use group passed or degraded for each delivery path, with concrete reasons for degraded outcomes. Missing evidence is a harness failure; explicit degraded evidence is allowed when a layer is unavailable.

### Upstream target matrix

A documentation and release-planning map that separates where a future change belongs: backend/windowing behavior goes toward the Computer Use Linux backend lineage, while Codex Desktop packaging and wrapper integration belong to the Codex Desktop Linux wrapper lineage.

### Runtime command dependency

An installed external command that project code may invoke at runtime, such as an X11 or input utility. Invoking a command is distinct from copying, vendoring, or adapting that command's source code.

### Release checklist

The project-owned handoff checklist that records required validation evidence before claiming v1 readiness, including OpenSpec validation, project checks, e2e evidence, rollback evidence, license review, and clean git state.


### Rollback-first install

An installation contract where every planned mutation records enough before-state to restore the system before the mutation is applied, and rollback refuses blind restoration when current state has drifted away from installer-owned after-state.

### Backup manifest

A durable, non-secret record of installer-owned changes, including before-state, after-state, changed-vs-already-present classification, file ownership, file mode, checksums, and enough partial-install progress to support idempotent rollback.

### Final DoD

The final Definition-of-Done evidence gate that answers whether the project has reached the documented Cinnamon/X11 v1 Computer Use baseline, including explicit pass/degraded capability evidence instead of an absolute claim for every desktop/session mode.

### Architecture decision ledger

A consolidated list of final v1 decision topics and their recorded outcomes, used to make sure durable architecture and safety choices are discoverable from tracked project documents rather than chat history.

### Reason category

A stable machine-readable label in e2e or doctor evidence that explains why a capability row is degraded or failed, such as environment limitation, missing fixture setup, code failure, unsupported out-of-scope path, or expected fake-fixture limitation.

### Controlled fixture

A test-owned application window or helper process created or uniquely selected by the e2e harness for safe live validation. Controlled fixtures are the only valid targets for live input, pointer, screenshot, app-state, target-context, and overlay evidence.

### Linux Feature adapter

A thin optional integration that lives in `codex-desktop-linux` under `linux-features/<feature-id>/` and stages or gates this repository's standalone plugin without rewriting core Computer Use behavior.

### Pinned release artifact

A versioned tarball plus checksum produced by this repository so a downstream adapter can stage the standalone plugin from immutable release bytes instead of copying or reinterpreting the source checkout.

### Adapter scaffold

A copyable reference implementation stored in this repository for a later upstream PR. It is inert in this repository runtime until copied into the upstream `linux-features/` boundary.


### Backend flavor route

A future upstreaming option where X11/EWMH behavior is evaluated for integration into an existing Computer Use Linux backend project, such as `agent-sh/computer-use-linux`, as a selectable backend/flavor. It is distinct from the current Linux Feature adapter path and must be evaluated in a separate change before it affects default Codex Desktop Linux behavior.
