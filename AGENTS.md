# dreamscaper — AI Agent Instructions

dreamscaper workspace tooling and development environment.

> **Canonical Source**: This file is the single source of truth for AI agent instructions. [CLAUDE.md](./CLAUDE.md), [GEMINI.md](./GEMINI.md) and [.github/copilot-instructions.md](./.github/copilot-instructions.md) are thin pointers to this file. It holds the rules for every task and sends each kind of task to the document that owns it.

## Rules for every task

1. **One issue, one pull request.** Work on a topic branch. Topic PRs target the default branch. The PR title is a Conventional Commit, and the PR body closes its one issue (`Closes #<issue>`). Plan it as the OpenSpec change `openspec/changes/<issue>-<slug>/`, and archive the change in the last commit with `devops openspec archive <change>`.
2. **Check the premise first.** Confirm it against the real system: upstream documents, library source or command output. Never return a tracker: work its open sub-issues or related issues. If a leaf item cannot be built as written, write no compensating code: comment the evidence on the issue and stop. Close issues as rejected if their functionality has been superseded by newer issues or code changes.
3. **Use an open tool before you build.** Name the tool or library that covers the need.
4. **Test new behavior first.** Write the failing test before the code. Prove any other change with a command.
5. **Pass the checks.** Push only when the checks pass. Never lower a threshold unless the owner allows it.
6. **Open every pull request as a draft.** Agents open every pull request with `devops pr create`, which always opens a draft. Watch it with `devops pr monitor <pr_number>`. Always mark it ready with `devops pr ready <pr_number>` when the checks pass and no agent work remains. Reply to each review comment in its thread. Copilot reviews a pull request only when a person requests it.
7. **Use `devops gh` and `devops pr`.** Never run bare `gh`: `devops` paces each request against the rate limit. Run one `devops ai` call at a time.
8. **Write as merged.** Make no commit that only updates tracking. If your change makes a document wrong, fix that document in the same pull request.
9. **Keep private data out of the repository.** Never commit a secret, the content of a gitignored file, an internal hostname or a private IP address. Use `example.com` and RFC 5737 addresses.
10. **Fix the cause.** If an outside constraint blocks the fix, add a check that fails when the problem returns.
11. **Keep your own files out of git.** Put scratch files in a gitignored folder. Back up a file outside the workspace before you edit it.
12. **Keep tool output small.** Search before you read, and read only the lines you need. Ask each command for less output. Keep each fact that the reader needs: error lines, `file:line`, failed test IDs.

Only a person merges pull requests.

## Where to look

| When you | Read |
| :--- | :--- |
| plan and record each item as an OpenSpec change | [openspec/config.yaml](openspec/config.yaml) |
| learn a `devops` command | `devops <command> --help` |
