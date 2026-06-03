## Context Read

- `openspec/changes/prepare-codex-desktop-linux-x11-ewmh-adapter/proposal.md`
- `openspec/changes/prepare-codex-desktop-linux-x11-ewmh-adapter/specs/**/*.md`
- `AGENTS.md`, `CONSTITUTION.md`, `CONTEXT.md`, `ARCHITECTURE.md`, `adr/README.md`, and in-force ADRs 0001, 0003, 0005, 0006, 0007, 0008, 0009, 0010, 0011
- Current release/package inputs: `Cargo.toml`, `Cargo.lock`, `VERSION`, `README.md`, `INSTALL_CODEX.md`, `CHANGELOG.md`, `scripts/install-codex-plugin.sh`, `scripts/uninstall-codex-plugin.sh`, `docs/install-uninstall.md`, `tests/plugin_installer.rs`, `tests/packaging_docs.rs`, `tests/e2e_harness_scripts.rs`
- Upstream read-only evidence from `/home/as/Документы/AI_PROJECTS/codex-desktop-linux`: `README.md` Linux Features section, `linux-features/README.md`, `docs/linux-features-architecture.md`, `scripts/lib/linux-features.js`, `linux-features/read-aloud-mcp/*`, and bundled `read-aloud` / `computer-use` plugin manifests

## Plan Summary

- The change prepares this repository as the source of truth for a later upstream `linux-features/x11-ewmh-computer-use/` adapter rather than changing upstream or replacing core Computer Use now.
- Release packaging must produce a deterministic plugin bundle tarball and SHA256 sidecar so downstream can stage a pinned artifact without understanding the repository internals.
- The adapter contract and scaffold must preserve maintainer constraints: fully opt-in, disabled by default, no submodule, no global doctor changes, no bundled `computer-use` replacement, and no core rewrite.
- The scaffold should model upstream `read-aloud-mcp` only where needed: feature metadata, stage hook, marketplace entry, conservative plugin gate patch, and self-contained Node tests.
- TDD and verification must cover package integrity, forbidden-file exclusion, manifest consistency, scaffold feature enablement, stage safety, patch idempotence, and docs cross-links.

## Question Loop

### Question 1: Should the adapter scaffold include a plugin gate patch or rely only on staged marketplace metadata?

- **Recommended answer**: Include a conservative `patches.js` plugin gate descriptor.
- **Rationale**: Upstream `read-aloud-mcp` uses a main-bundle plugin gate patch for a separate Linux MCP plugin. The user explicitly said to assume we likely need one and write it conservatively unless marketplace/stage is enough. A narrow idempotent patch keeps the future upstream PR aligned with the existing pattern while tests ensure it does not change bundled `computer-use` descriptor behavior.
- **Resolution**: Resolved from upstream `read-aloud-mcp` pattern and user requirements; no user question required.

### Question 2: Should release packaging duplicate installer bundle generation or introduce a shared bundle helper?

- **Recommended answer**: Extract or reuse shared manifest/bundle-writing logic where practical, with tests comparing packaged `.mcp.json` and plugin manifest against the existing standalone installer contract.
- **Rationale**: The specs require avoiding drift between installer and package artifact. If full extraction is larger than necessary, the package script may still write the bundle explicitly, but tests must lock the contract and future refactors can consolidate further.
- **Resolution**: Resolved by spec requirements; design should choose the smallest safe reuse point after inspecting installer implementation.

### Question 3: Should this change publish a new GitHub release or modify the upstream checkout?

- **Recommended answer**: No. Prepare release artifacts and release-ready docs only. Publishing and upstream PR work require separate explicit approval.
- **Rationale**: User explicitly said do not publish without approval and do not modify upstream in this change. Constitution/Git discipline also require explicit approval for push/PR/archive and external release actions.
- **Resolution**: Resolved; implementation must not publish, push, PR, archive, or write to upstream checkout.

## Resolved Terms

- **Linux Feature adapter** — a thin optional upstream `linux-features/<feature-id>/` integration that stages/gates this standalone plugin without rewriting core Computer Use.
- **Pinned release artifact** — a versioned tarball plus checksum from this repository for downstream staging from immutable release bytes.
- **Adapter scaffold** — a copyable reference implementation in this repository, inert until copied into upstream `linux-features/`.

`CONTEXT.md` was updated inline with these glossary terms.

## Document Updates Applied

- Added `CONTEXT.md` glossary entries for Linux Feature adapter, pinned release artifact, and adapter scaffold.
- Proposal and specs already encode the maintainer constraints, staging modes, no-upstream-mutation boundary, no-release-publication boundary, and conservative patch/scaffold test requirements.

## Document Updates Required Before Next Gate

None.

## ADR Candidates

- **Adapter handoff boundary** is a candidate for per-change ADR review: source of truth remains this repository; upstream adapter is a thin disabled-by-default Linux Feature. It may not need a new durable top-level ADR because ADR 0009 and ADR 0010 already preserve standalone identity, upstream separation, and no global masquerade.
- **Release artifact as adapter input** is likely change-local unless the design discovers a broader long-lived distribution architecture decision.

## Open Questions

None.
