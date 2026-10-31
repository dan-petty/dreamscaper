# Design

## Decisions

- **Use `cargo update -p windows@0.58.0 --precise 0.62.2` for a targeted lockfile update.**: Both gpu-allocator and gilrs-core accept 0.62.x. This is the minimal change that resolves the conflict without modifying any source or manifest files.
- **Target windows 0.62.2 specifically.**: wgpu-hal 29.0.4, accesskit_windows 0.32.1, and cpal 0.17.3 already use 0.62.2. Unifying on this version eliminates the type mismatch with zero additional dependency changes.

## Rejected alternatives

- **Pin windows to 0.58.0 in Cargo.toml.**: wgpu-hal 29.0.4 requires windows 0.62.2. Downgrading would break wgpu-hal and the entire Bevy 0.19.1 rendering stack.
- **Regenerate Cargo.lock from scratch with a full `cargo update`.**: A full update would change many unrelated dependency versions, increasing the risk of new breakage across all platforms.
