# Proposal

## Why

The reusable workflows and the repository template moved from dan-petty/infra to dan-petty/github-workflows (dan-petty/infra#217). A release of infra after the move holds neither, and dan-petty/infra#235 turns off the Renovate github-actions and copier managers for each form of the name of infra (#37). This repository rendered the template of infra v0.25.3 and called its workflows.

## What Changes

- Set `_src_path: https://github.com/dan-petty/github-workflows.git` in `.copier-answers.yml` and run `copier recopy --skip-answered --defaults --overwrite --vcs-ref v0.1.0`. The answers stay, and `.copier-answers.yml` names `_commit: v0.1.0`.
- The template callers `.github/workflows/pr-title.yml` call dan-petty/github-workflows at the commit of the tag v0.1.0, `39f92fb644e550f84a5797cce46a80a891231d4a`, with `# v0.1.0` in the comment.
- The comments of `github/repository.yaml` and `openspec/config.yaml` name dan-petty/github-workflows. The template adds the empty files `openspec/changes/archive/.gitkeep` and `openspec/specs/.gitkeep`.
- Copier keeps `.github/renovate.json`, `.pre-commit-config.yaml`, `.devops/project.toml`, `version.txt` and `.release-please-manifest.json` of the repository (`_skip_if_exists`). The Renovate presets stay in dan-petty/infra.
- The change alters no behavior that a consumer observes: it changes CI workflow callers, comments and settings files. It sets `skip_specs: true`.
- The change fits one pull request.

## Feasibility

- The tag v0.1.0 of dan-petty/github-workflows is the commit `39f92fb644e550f84a5797cce46a80a891231d4a` (`git rev-parse v0.1.0^{commit}`).
- `pr-title.yml`, `release.yml` and `k8s-validate.yml` of dan-petty/github-workflows at v0.1.0 run the same steps as those of dan-petty/infra at v0.25.3. Only names, the checkout folder and the concurrency group prefix `github-workflows-` differ, and no caller of this repository uses a group with that prefix.
- `copier recopy` of Copier 9.19.1 at v0.1.0 changes only the files above, and a second recopy changes no file.

## Existing Tool Considered

Copier (MIT, maintained) renders the template from its new source, and Renovate (AGPL-3.0, maintained) proposes the next copier update and moves the other caller pins. The change builds no code and deletes no internal code.
