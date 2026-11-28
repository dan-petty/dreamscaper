# Proposal

## Why

The issue says `pre-commit run --all-files` fails on the default branch: the `check-json` hook cannot parse the JSONC comments of `.devcontainer/devcontainer.json`, and `end-of-file-fixer` rewrites the extra trailing newline of `scripts/run.sh`. No workflow runs the hooks, so CI never shows the failure, and a commit that touches one of the two files fails its hook. #36

## What Changes

- Exclude the JSONC devcontainer file from the `check-json` hook of `.pre-commit-config.yaml`, so the strict JSON parser of that hook stops reading the file and the other JSON files stay checked.
- Remove the extra trailing newline of `scripts/run.sh`, so the file ends with exactly one newline and `end-of-file-fixer` reports no rewrite.
- Leave the 2 workflow files unchanged, so the change holds no CI job that runs the hooks.
- Record the hook rules of the repository as the new capability `commit-hooks` in the spec delta of the change.
- The change fits one pull request.

## Capabilities

### New Capabilities

- `commit-hooks`: Exclude the JSONC devcontainer file from check-json, Keep exactly one trailing newline in scripts/run.sh, Exit zero on the full pre-commit run

### Modified Capabilities

None.

## Impact

- Code: `.pre-commit-config.yaml`, `scripts/run.sh`

## Feasibility

- `read_file .pre-commit-config.yaml` showed the 22 lines of the hook list, with `.pre-commit-config.yaml:15: - id: check-json` holding no `exclude` key and `.pre-commit-config.yaml:8: rev: v6.0.0` pinning the hook repository, so the premise of the issue holds at the staged commit.
- `read_file .devcontainer/devcontainer.json` (lines 1 to 14) showed `.devcontainer/devcontainer.json:10: // Optional: For native host GUI window & GPU passthrough on Windows (WSL2/WSLg) or Linux,`, so the file holds `//` comments and the strict JSON parser of `check-json` cannot parse it.
- `read_file scripts/run.sh` (lines 44 to 52) showed `scripts/run.sh:51:esac` as the last line that holds text, with line 52 empty, so the file ends with two newlines and `end-of-file-fixer` rewrites it.
- The probes confirm the three paths word for word: "The path `.devcontainer/devcontainer.json` is a file of the staging commit." and "The path `scripts/run.sh` is a file of the staging commit." and "The path `.pre-commit-config.yaml` is a file of the staging commit."
- The probe line "The path `run.sh` is no file at the root of the staging commit. These files of the commit are candidates: `scripts/run.sh`. No commit in the history deleted or renamed it." resolves the `run.sh` of the issue title to `scripts/run.sh`, so the issue is built as written.
- `search_code pre-commit` matched no file under `.github/workflows/`, and `list_files .github/workflows` showed 2 files, `.github/workflows/ci.yml` and `.github/workflows/pr-title.yml`, so neither workflow runs the hooks and CI never shows the failure, as the issue says.
- `list_files openspec/specs` showed only `openspec/specs/.gitkeep`, so no capability exists yet and the change adds the new capability `commit-hooks` with no `modified`, `renamed` or `removed` entry.
- The implementation runs these checks against the real system, each needing a checkout of the repository: `pre-commit run check-json --files .devcontainer/devcontainer.json`, then `pre-commit run end-of-file-fixer --files scripts/run.sh`, then the done-when command `pre-commit run --all-files`.
- The planning run made no `pre-commit` call, so the hook behavior is not checked here and the three commands above stay the work of the implementation.
- The first hook run needs a network on a cold cache to build the environment of `.pre-commit-config.yaml:8: rev: v6.0.0`; the verify command `uv run devops ci --check --only pre-commit` needs a checkout of the repository and the `devops` tool of the runner image; the plan calls no other function whose arguments need a checkout, a client or a network.
- Runner capability: `pre-commit`, `uv`, `devops` and `git` are image tools of the runner, so no program of the plan is NOT_IN_RUNNER; the failure class of a failing hook is `lint`, and the pod of the Job has a read-only root and no cluster token, so the hooks check the checkout only.
- The issue names two files, `.pre-commit-config.yaml` and `scripts/run.sh`; the root `list_files` and the probes give a count of 2, and each gets its decision: the config takes the `exclude` key on `check-json`, and the script loses its extra newline. The plan leaves `.devcontainer/devcontainer.json` and the 2 workflow files unchanged, so no group lists them in `changes` or `creates`.

## Existing Tool Considered

pre-commit 4.7.0 with the pre-commit-hooks revision `v6.0.0` of `.pre-commit-config.yaml:8: rev: v6.0.0` (MIT, maintained) covers the need, and the repository already runs it, so the change adds one `exclude` key and deletes one blank line and no internal code.

## Dependencies

None.
