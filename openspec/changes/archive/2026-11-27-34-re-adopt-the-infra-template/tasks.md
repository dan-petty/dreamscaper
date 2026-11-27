# Tasks

## 1. Re-adopt the template

Kind: other.

- [x] 1.1 Run `copier recopy --skip-answered --defaults --overwrite --vcs-ref v0.25.3` with the answers of this repository
  - Verification: `$ copier recopy --skip-answered --defaults --overwrite --vcs-ref v0.25.3 .` changes no file (`git status --short` prints nothing)
- [x] 1.2 Check the changed files
  - Verification: `$ actionlint` and `$ pre-commit run --from-ref origin/main --to-ref HEAD` exit 0
