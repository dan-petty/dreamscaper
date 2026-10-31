# Tasks

## 1. Update Cargo.lock to unify windows crate on 0.62.2

Reads: none. Changes: `Cargo.lock`.

- [x] 1.1 Run `cargo tree --target x86_64-pc-windows-msvc -i windows@0.58.0`. This command currently succeeds and lists gpu-allocator and gilrs-core as dependents. After the fix, it MUST report that no such package is in the graph, and verify that `cargo tree --target x86_64-pc-windows-msvc -i windows` fails on the base
- [x] 1.2 Run `cargo update -p windows@0.58.0 --precise 0.62.2`. This rewrites only Cargo.lock. It changes the windows dependency of gpu-allocator 0.28.0 and gilrs-core 0.6.8 from 0.58.0 to 0.62.2. No Cargo.toml or source files are touched, and verify that `cargo tree --target x86_64-pc-windows-msvc -i windows` passes
- [x] 1.3 Change no document, run `uv run devops docs generate --sync-readme`, and verify that `uv run devops ci docs` passes

## Workflow follow-up

- Pending a person: review the diff in tuicr (tuicr -r <base>..HEAD).
