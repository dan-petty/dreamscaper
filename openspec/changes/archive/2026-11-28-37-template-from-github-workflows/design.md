# Design

## Decisions

- **dan-petty/github-workflows is the source of the template and of the reusable workflows.** The owner chose a new public repository for them (dan-petty/infra#217).
- **Merge after dan-petty/infra#235.** Its Renovate presets name dan-petty/github-workflows for the copier update and leave the pin of each template caller to it. Before it, the github-actions manager of Renovate also moves the github-workflows pin of the template callers, so those files get two owners again.

## Rejected alternatives

- **Stay on the template of infra v0.25.3.** After dan-petty/infra#235, Renovate proposes no copier update and no caller update of infra, so the repository keeps v0.25.3. A copier update to a later release of infra writes the root files of infra into the repository (dan-petty/infra#235, renovate/README.md, "Move to dan-petty/github-workflows").
