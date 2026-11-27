# Proposal

## Why

The Renovate copier update of the infra template fails in this repository with merge conflicts, so `renovate/artifacts` fails and no update merges (#34). Two owners write one template file: Renovate moves the infra pin of the workflow callers, and the repository changed template files by hand. dan-petty/infra#225 gives each template file one owner, in the infra release v0.25.3.

## What Changes

- Run `copier recopy --skip-answered --defaults --overwrite --vcs-ref v0.25.3` with the answers `scopes`, `scope_descriptions` and `required_checks` of this repository. `.copier-answers.yml` names `_commit: v0.25.3`.
- The template writes `.github/labels.yml`, `.github/CODEOWNERS`, `github/repository.yaml` and `.github/workflows/pr-title.yml` from the answers.
- `.github/labels.yml` keeps only the labels of this repository, and gets the template scopes `ci`, `deps` and `docs`. The workflow labels (`type/*`, `priority/*`, `status/*`, `release`, `needs-split`, `source/agent`, `budget/borrowed`) come from devops-cli.
- Copier keeps `.github/renovate.json`, `.pre-commit-config.yaml`, `.devops/project.toml`, `version.txt` and `.release-please-manifest.json` of the repository (`_skip_if_exists`).
- The change alters no behavior that a consumer observes: it changes label data, repository settings and CI workflow callers. It sets `skip_specs: true`.
- The change fits one pull request.

## Feasibility

- `copier recopy` of Copier 9.19.1 at the tag v0.25.3 changes only the files above. A run with the same answers against the branch of dan-petty/infra#225 gave the same diff.

## Existing Tool Considered

Copier (MIT, maintained) renders and updates the template, and its `_skip_if_exists` keeps the files of the repository. Renovate (AGPL-3.0, maintained) proposes the next copier update. The change builds no code and deletes no internal code.
