## ADDED Requirements

### Requirement: Root Rust bootstrap package
The project MUST provide a standalone Rust 2021 package at the repository root for the initial `codex-computer-use-x11` implementation, and it MUST keep the package independent from the Codex Desktop Linux integration target checkout. The initial root `Cargo.toml` MUST include a root `[package]`; a `[workspace]` table is optional only if it keeps the root package as the primary package for this bootstrap stage.

#### Scenario: Initialize the standalone Rust package
- **GIVEN** a clean checkout of `codex-computer-use-x11`
- **WHEN** a developer inspects the repository root
- **THEN** a root `Cargo.toml` declares the `codex-computer-use-x11` package
- **AND** the root package remains the primary bootstrap crate even if a workspace table is present
- **AND** the initial Rust source lives under root `src/`
- **AND** no files in the checkout referenced by `CODEX_DESKTOP_LINUX_FULL_PATH` or its documented development-machine default are modified by this bootstrap package

### Requirement: Repeatable project verification commands
The project MUST expose root-level Makefile commands for formatting, checking, and testing, and those commands MUST be thin wrappers over Cargo so direct Cargo usage remains equivalent.

#### Scenario: Run the project verification surface
- **GIVEN** the Rust toolchain is available in the development environment
- **WHEN** a developer runs `make fmt`, `make check`, and `make test` from the repository root
- **THEN** the commands execute `cargo fmt -- --check`, `cargo check`, and `cargo test` respectively
- **AND** each command exits non-zero when the corresponding Cargo command fails
- **AND** `make fmt` exits non-zero when formatting violations are present instead of reformatting files in place

### Requirement: Machine-local integration target path
The project MUST treat the Codex Desktop Linux integration target checkout as machine-local configuration referenced by `CODEX_DESKTOP_LINUX_FULL_PATH`, not as a hard-coded portable path.

#### Scenario: Document the integration target path
- **GIVEN** project documentation or scripts need to refer to the integration target checkout
- **WHEN** the target path is described or consumed
- **THEN** the durable name is `CODEX_DESKTOP_LINUX_FULL_PATH`
- **AND** any concrete local default path is documented only as the current development-machine default, not as a portable requirement
- **AND** no secret file is required to read or validate the local target path

### Requirement: Project posture documentation
The project MUST document its delivery posture so future implementers can verify that bootstrap work remains Codex-first, Cinnamon/X11-first, and generic X11/EWMH-oriented without prematurely patching the integration target.

#### Scenario: Document delivery posture
- **GIVEN** a developer reads the project README or bootstrap project documentation
- **WHEN** the delivery posture is described
- **THEN** the documentation states that the project is Codex-first
- **AND** it states that first validation targets Cinnamon/X11
- **AND** it states that the backend strategy is generic X11/EWMH using `x11-ewmh`
- **AND** it names standalone plugin and future source overlay as delivery paths
