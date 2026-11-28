# Tasks

## 1. Exclude the JSONC devcontainer file and fix the run.sh newline

Kind: other. Reads: `.pre-commit-config.yaml`, `.devcontainer/devcontainer.json`, `scripts/run.sh`. Changes: `.pre-commit-config.yaml`, `scripts/run.sh`.

- [x] 1.1 Add `exclude: ^\.devcontainer/devcontainer\.json$` under the `check-json` hook of `.pre-commit-config.yaml`, note that exclusion in the header comment of that file, and run `pre-commit run end-of-file-fixer --files scripts/run.sh` so `scripts/run.sh` ends with one newline. Keep the `//` comments of the devcontainer file, which no group changes
  - Verification: `$ uv run devops uv run pre-commit run --all-files`

## Amendments

- Group 1, group: Both verification commands of group 1 cannot run in the runner image: `pre-commit` is no row of `devops ci --only`, and the `docs` row fails inside the image's own devops_cli, so no change to the group's two files can make either text pass.. Evidence: `uv run devops ci --check --only pre-commit` exits 2 with `Invalid value for '--only': Unknown check row: 'pre-commit'` and lists its rows (test, lint, ..., devcontainer, openspec, git, ...), which hold none that runs the repository's hooks. `uv run devops ci docs` exits 1 with `RuntimeError: Failed to import module 'devops_cli.grafana.service_dashboard' ... No module named 'grafana_foundation_sdk'` raised at /app/.venv/lib/python3.14/site-packages/devops_cli/docs/generator.py:977 before any repository file is read. Both fail the same way in a clean `git worktree` of HEAD without the changes of this step. The replacement `uv run devops uv run pre-commit run --all-files` exits 0 with every hook Passed and starts with `uv run devops`, as `is_valid_verify` requires.. `Exclude the JSONC devcontainer file and fix the run.sh newline` -> `Exclude the JSONC devcontainer file and fix the run.sh newline`
