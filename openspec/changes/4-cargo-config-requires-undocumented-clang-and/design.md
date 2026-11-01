# Design

## Decisions

- **Delete the x86_64-unknown-linux-gnu linker block from .cargo/config.toml.**: Rust 1.90+ ships rust-lld as the default linker on x86_64-unknown-linux-gnu. The project uses the stable channel (rust-toolchain.toml). The override only adds a system dependency on clang and lld with no benefit.
- **Keep the aarch64-unknown-linux-gnu linker block.**: aarch64-unknown-linux-gnu does not default to rust-lld. The explicit clang and lld configuration is required for linking on that target.
- **Delete the [env] block that sets BEVY_ASSET_ROOT.**: Bevy falls back to CARGO_MANIFEST_DIR, which resolves to the repository root and is the correct base path. The variable was misconfigured (producing assets/assets) and is unrelated to hot reloading.
- **Remove lld and clang from both CI apt install lines.**: CI only builds x86_64-unknown-linux-gnu on Linux. With the x86_64 block deleted, these packages are not needed.
- **Remove mold from the dev container package list.**: No file in the repository references mold as a linker or tool. It adds install time and image size with no benefit.
- **Add clang and lld to the README with an aarch64-only note.**: The aarch64 block still requires them. Documenting them prevents build failures on aarch64 hosts while not confusing x86_64 users.

## Rejected alternatives

- **Keep the x86_64 linker block.**: Rust 1.90+ ships rust-lld as the default linker on x86_64-unknown-linux-gnu (rust-lang/rust#140525). The override forces a clang and lld install that the toolchain already provides.
- **Delete the aarch64 linker block.**: aarch64-unknown-linux-gnu does not have rust-lld as the default linker. Deleting the block would break linking on aarch64 Linux hosts.
- **Fix BEVY_ASSET_ROOT to an empty string or dot.**: This would work but adds a confusing empty or dot value. Deleting the block is cleaner and relies on the correct CARGO_MANIFEST_DIR default.
- **Keep mold in the dev container.**: No project file references mold. Keeping it only increases the container image size.
- **Add clang and lld to the README unconditionally.**: This would mislead x86_64 users into installing packages they do not need. The x86_64 target does not require them.
