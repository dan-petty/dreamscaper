# Tasks

## 1. Move to the template of dan-petty/github-workflows

Kind: other.

- [x] 1.1 Set the new `_src_path` and run `copier recopy --skip-answered --defaults --overwrite --vcs-ref v0.1.0`
  - Verification: `$ copier recopy --skip-answered --defaults --overwrite --vcs-ref v0.1.0 .` changes no file (`git status --short` prints nothing)
- [x] 1.2 Check the changed files
  - Verification: `$ actionlint` and `$ pre-commit run --from-ref origin/main --to-ref HEAD` exit 0
