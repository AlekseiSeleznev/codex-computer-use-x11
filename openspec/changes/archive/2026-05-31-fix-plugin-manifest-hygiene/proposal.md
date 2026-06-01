## Why

A review found that the repository has two tracked OpenSpec backup artifacts and that the generated standalone plugin manifest has drifted from the actual repository/tool surface. These small hygiene issues make the project look less trustworthy even though the code and verification suite are otherwise clean.

## What Changes

- Remove accidentally tracked `openspec/config.yaml.bak.*` backup files from version control and ignore future timestamped backup files.
- Correct the standalone plugin manifest `homepage` URL to the actual GitHub remote owner/name.
- Update the generated standalone plugin manifest long description and default prompts so they cover the full `x11_*` MCP tool surface exposed by `src/mcp.rs` and documented in `README.md`.
- No breaking changes.

## Capabilities

- Modify `project-bootstrap` to require repository hygiene for generated backup artifacts.
- Modify `standalone-codex-mcp-plugin` to require accurate generated plugin marketplace metadata for repository URL and exposed tool descriptions.

## Impact

- Affected files: `scripts/install-codex-plugin.sh`, `.gitignore`, tracked OpenSpec backup files, and tests around installer manifest generation if needed.
- Public CLI/MCP behavior is unchanged; only repository hygiene and generated plugin metadata change.
- Verification follows `CONSTITUTION.md`: run OpenSpec validation for changed artifacts and project checks (`make fmt`, `make check`, `make test`) or report blockers.
- No external systems or secret values are needed.
