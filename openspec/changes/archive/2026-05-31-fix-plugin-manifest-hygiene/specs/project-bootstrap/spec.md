## ADDED Requirements

### Requirement: Repository backup artifact hygiene
The repository MUST keep generated timestamped backup files out of tracked source control while preserving canonical OpenSpec configuration files.

#### Scenario: Ignore future timestamped backup files
- **GIVEN** a tool or editor creates a timestamped backup file whose name matches `*.bak.*`
- **WHEN** a developer checks repository status
- **THEN** Git ignores that backup artifact by default
- **AND** canonical files such as `openspec/config.yaml` remain trackable

#### Scenario: Remove accidental OpenSpec config backups from tracked files
- **GIVEN** the repository previously tracked timestamped `openspec/config.yaml.bak.*` files
- **WHEN** the hygiene change is applied
- **THEN** those backup files are no longer tracked source files
- **AND** no canonical OpenSpec configuration content is removed
