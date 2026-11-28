# Design

## Decisions

- **Add a hook-level `exclude: ^\.devcontainer/devcontainer\.json$` to the `check-json` hook of `.pre-commit-config.yaml`.**: The exclusion touches one hook and one file, adds no dependency, keeps the JSONC comments of the devcontainer file, and `check-json` still checks the other JSON files.
- **Let `end-of-file-fixer` rewrite `scripts/run.sh` and commit the rewrite, so the file ends with exactly one newline.**: The hook holds the rule for one trailing newline, so the file matches the rule that the hook enforces and a later run reports no change.
- **Keep the change to the two files the issue names, `.pre-commit-config.yaml` and `scripts/run.sh`, and add no workflow that runs the hooks.**: The issue names two files and one pull request closes it, so a CI job that runs the hooks is a separate item.

## Rejected alternatives

- **Add a JSONC-aware hook to check `.devcontainer/devcontainer.json`.**: The alternatives block of the issue records 8 searches and no maintained JSONC-aware hook, and the staged config holds 1 repo, at `.pre-commit-config.yaml:7: - repo: https://github.com/pre-commit/pre-commit-hooks`, so a new hook adds a revision to update for one file.
- **Remove the `//` comments from `.devcontainer/devcontainer.json` to make the file strict JSON.**: `.devcontainer/devcontainer.json:10: // Optional: For native host GUI window & GPU passthrough on Windows (WSL2/WSLg) or Linux,` documents the optional WSL and GPU blocks, so the removal loses that guidance for the reader.
- **Add a workflow job that runs `pre-commit run --all-files` in this change.**: `.github/workflows/ci.yml` holds 3 jobs and none runs the hooks, and the issue asks only that the local run exits 0, so the job is a separate item.
