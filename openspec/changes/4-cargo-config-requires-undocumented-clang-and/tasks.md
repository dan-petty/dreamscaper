# Tasks

## 1. Remove x86_64 linker block and env block from .cargo/config.toml

Reads: none. Changes: `.cargo/config.toml`.

- [ ] 1.1 Run `grep -c 'x86_64-unknown-linux-gnu' .cargo/config.toml` and confirm the output is 0. Run `grep -c 'BEVY_ASSET_ROOT' .cargo/config.toml` and confirm the output is 0. Run `grep -c 'aarch64-unknown-linux-gnu' .cargo/config.toml` and confirm the output is 1, and verify that `grep -c 'x86_64-unknown-linux-gnu' .cargo/config.toml; grep -c 'BEVY_ASSET_ROOT' .cargo/config.toml; grep -c 'aarch64-unknown-linux-gnu' .cargo/config.toml` fails on the base
- [ ] 1.2 Delete the [target.x86_64-unknown-linux-gnu] block (lines 1-3). Delete the [env] block that sets BEVY_ASSET_ROOT (lines 19-21). Keep the [target.aarch64-unknown-linux-gnu] block unchanged, and verify that `grep -c 'x86_64-unknown-linux-gnu' .cargo/config.toml; grep -c 'BEVY_ASSET_ROOT' .cargo/config.toml; grep -c 'aarch64-unknown-linux-gnu' .cargo/config.toml` passes
- [ ] 1.3 Change no document, run `uv run devops docs generate --sync-readme`, and verify that `uv run devops ci docs` passes

## 2. Remove lld and clang from CI apt install lines

Reads: none. Changes: `.github/workflows/ci.yml`.

- [ ] 2.1 Run `grep -c 'lld\|clang' .github/workflows/ci.yml` and confirm the output is 0, and verify that `grep -c 'lld\|clang' .github/workflows/ci.yml` fails on the base
- [ ] 2.2 In the check job apt install line (line 23), remove `lld clang` from the package list. In the build-desktop job apt install line (line 56), remove `lld clang` from the package list, and verify that `grep -c 'lld\|clang' .github/workflows/ci.yml` passes
- [ ] 2.3 Change no document, run `uv run devops docs generate --sync-readme`, and verify that `uv run devops ci docs` passes

## 3. Remove mold from dev container package list

Reads: none. Changes: `.devcontainer/devcontainer.json`.

- [ ] 3.1 Run `grep -c 'mold' .devcontainer/devcontainer.json` and confirm the output is 0, and verify that `grep -c 'mold' .devcontainer/devcontainer.json` fails on the base
- [ ] 3.2 In the postCreateCommand string (line 9), remove `mold` from the apt-get install package list, and verify that `grep -c 'mold' .devcontainer/devcontainer.json` passes
- [ ] 3.3 Change no document, run `uv run devops docs generate --sync-readme`, and verify that `uv run devops ci docs` passes

## 4. Add aarch64-only clang and lld note to README

Reads: none. Changes: `README.md`.

- [ ] 4.1 Run `grep -c 'aarch64' README.md` and confirm the output is at least 1. Run `grep 'clang' README.md` and confirm the output mentions aarch64, and verify that `grep 'aarch64' README.md && grep 'clang' README.md` fails on the base
- [ ] 4.2 In the Linux Packages section (around lines 137-151), add a note after the apt-get install block: `# clang and lld are required on aarch64 Linux hosts only`. Add `clang lld` to the apt-get install line with a comment marking them as aarch64-only, and verify that `grep 'aarch64' README.md && grep 'clang' README.md` passes
- [ ] 4.3 README.md, run `uv run devops docs generate --sync-readme`, and verify that `uv run devops ci docs` passes

## Workflow follow-up

- Pending a person: review the diff in tuicr (tuicr -r <base>..HEAD).
