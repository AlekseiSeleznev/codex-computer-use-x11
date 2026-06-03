# Project Constitution

This file is the persistent, Git-tracked project context that Codex must read
before `/opsx:*` workflows, direct OpenSpec lifecycle skill actions, and other
substantial project work.

It is **not** an OpenSpec change artifact. It is not archived with changes.
Keep it in the project root and update it when project-wide rules change.

> **No secrets here.** Store real logins, passwords, tokens, and private service
> URLs only in local `.secrets.local.env` or the local environment. This file may
> contain credential variable names, but never their values.

## Required Technologies

Record mandatory languages, frameworks, tools, runtimes, package managers,
architecture styles, coding standards, and compatibility constraints.

Template defaults for this repository:

- OpenSpec CLI 1.3.x-compatible public commands.
- Codex project-local overlay under `.codex/prompts` and `.codex/skills`.
- Markdown OpenSpec artifacts with Gherkin-style scenarios.
- Bash helper scripts under `scripts/`.
- Durable ADRs under top-level `adr/`.
- Root `ARCHITECTURE.md` as the current architecture snapshot.

Project-specific additions:

- Rust 2021 edition and Cargo are the default implementation stack for the standalone `codex-computer-use-x11` crate because the integration target is Rust.
- The initial standalone Rust package/workspace lives at the repository root unless an accepted design/ADR introduces subcrates.
- Project-level verification commands use a root `Makefile`: `make fmt`, `make check`, and `make test` as thin wrappers over Cargo.
- The local integration target checkout is machine-specific and referenced by `CODEX_DESKTOP_LINUX_FULL_PATH`; the current development-machine default is `/home/as/Документы/AI_PROJECTS/codex-desktop-linux`; `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full` is legacy evidence only unless explicitly selected.

## MCP Servers

List MCP servers Codex should use for particular development blocks. Include
non-secret parameters only. If a server needs secrets, reference variable names
from `.secrets.local.env`.

| Development block | MCP server | When to use | Non-secret parameters | Secret variables |
| --- | --- | --- | --- | --- |
| OpenSpec lifecycle | local OpenSpec CLI | Read status, instructions, validate, archive | Project root | None |
| GitHub publishing | GitHub CLI / connector | Repository metadata, releases, PRs | Repository owner/name | `GITHUB_TOKEN` if required by local tooling |
| Local integration target | filesystem / Git CLI | Read and compare the Codex Desktop Linux target checkout | `CODEX_DESKTOP_LINUX_FULL_PATH` or documented local default path | None |

## External Systems

List external systems that are accessed without MCP. Do not put real endpoints,
logins, passwords, tokens, or other secret values here. Use variable names and
store values in `.secrets.local.env`.

| System | Purpose | Access method | Required variables | Notes |
| --- | --- | --- | --- | --- |
| None currently | This bootstrap project does not require non-MCP external-system access | N/A | None | Add rows before introducing any external system |

## Secret Handling

- Real secret values live only in local `.secrets.local.env` or the local
  environment.
- `.secrets.local.env` must be ignored by Git and must never be staged,
  committed, archived, printed in chat, copied into OpenSpec artifacts, or used
  in screenshots/logs/diffs.
- Git-tracked files may document variable names, comments, and empty
  placeholders only.
- Before Codex accesses an external system, it must check that the required
  variables listed in this constitution exist locally.
- If a required variable is missing, Codex must stop and report the missing
  variable name without revealing any values.
- Codex should read `.secrets.local.env` only when the current workflow actually
  needs a listed external system.

Expected local file:

```text
.secrets.local.env
```

Optional tracked example file:

```text
.secrets.example.env
```

## Documentation Sources

Record which documentation Codex should prefer, when to consult it, and how to
cite or apply it.

Default guidance:

- Use project OpenSpec artifacts and top-level ADRs as the primary source of
  truth for planned behavior and durable decisions.
- Use root `ARCHITECTURE.md` as the current architecture snapshot for new chats
  and architecture-sensitive work.
- Use official documentation for external libraries, APIs, CLIs, and services
  when current syntax, options, or compatibility matter.
- Prefer project-local docs over generic assumptions when they answer the
  question.

Project-specific sources:

| Topic | Source | When to use | Notes |
| --- | --- | --- | --- |
| OpenSpec workflow | `openspec/`, `.codex/skills/openspec-*`, `.codex/prompts/opsx-*` | Planning/apply/verify/archive work | Keep OpenSpec as lifecycle engine |
| Current architecture | `ARCHITECTURE.md`, `adr/README.md`, `adr/*.md` | New chats and architecture-sensitive work | Snapshot first, ADRs for rationale |
| Architecture decisions | `adr/*.md` | Design/apply/verify | Follow in-force ADRs |
| Integration target source | `${CODEX_DESKTOP_LINUX_FULL_PATH}` or `/home/as/Документы/AI_PROJECTS/codex-desktop-linux` on this machine | Planning/design/apply for source-overlay compatibility | Read-only unless an OpenSpec task explicitly targets overlay changes; `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full` is a legacy evidence checkout |

## Verification Rules

Record checks Codex must run or consider before claiming work is complete.

Template defaults:

- Run OpenSpec validation for changed OpenSpec artifacts.
- Run `openspec schema validate intent-driven` after schema updates.
- Run `scripts/check-overlay` after overlay/template updates.
- Verify Git status and ensure local secret files are not staged or tracked.
- Do not mark tasks complete until the relevant verification has run or the
  verification limitation is explicitly reported.

Project-specific verification rules:

- Rust changes must pass `make fmt`, `make check`, and `make test` or report the exact blocker.
- `doctor --json` behavior must be validated as machine-readable JSON before tasks depending on it are marked complete.
- X11 window-id normalization must include tests proving equivalent hex forms map to the same `u64`.

## Additional AI Instructions

Record any additional project-wide instructions that Codex must not miss.

Examples:

- Stop and ask before changing project-wide architecture decisions.
- Stop and ask if a request conflicts with this constitution.
- Prefer small implementation groups with visible Git checkpoint boundaries.
- Safe OpenSpec lifecycle checkpoint commits are automatic by default when session git discipline is `auto`; merge, push, archive, destructive git operations, dirty unrelated work, and hard-gate bypasses still require explicit approval.
- Prefer root-level Cargo/Makefile bootstrap until design evidence justifies a more complex workspace layout.
