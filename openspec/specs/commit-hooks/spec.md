# commit-hooks Specification

## Purpose

The pre-commit hook configuration of the repository and the files that the hooks check.

## Requirements

### Requirement: Exclude the JSONC devcontainer file from check-json
The `check-json` hook of `.pre-commit-config.yaml` SHALL exclude `.devcontainer/devcontainer.json`, so the JSONC comments of that file stay in place.

#### Scenario: check-json skips the JSONC devcontainer file
- **WHEN** `uv run devops ci --check --only pre-commit` runs over the repository
- **THEN** the command exits 0 and the `check-json` hook reports no failing file

### Requirement: Keep exactly one trailing newline in scripts/run.sh
`scripts/run.sh` SHALL end with exactly one newline.

#### Scenario: end-of-file-fixer leaves scripts/run.sh unchanged
- **WHEN** `uv run devops ci --check --only pre-commit` runs over the repository
- **THEN** the command exits 0 and the `end-of-file-fixer` hook reports no file it rewrote

### Requirement: Exit zero on the full pre-commit run
`pre-commit run --all-files` SHALL exit 0 in a clone of the repository.

#### Scenario: The full pre-commit run exits 0
- **WHEN** `uv run devops ci --check --only pre-commit` runs over the repository on the branch of the change
- **THEN** the command exits 0 and reports no failing hook
