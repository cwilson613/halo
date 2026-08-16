# Project directives

## Mission

Build a clean-room Halo-inspired game, runtime, and artist-facing content toolchain with Rust and Bevy. Compatibility behavior comes from documented observations and lawful fixtures, not copied proprietary or unlicensed source.

## Boundaries

- This is an independent Git repository, not a submodule of the original-Xbox reconstruction.
- Never commit Halo executables, maps, tags, textures, sounds, HEK packages, or unclear-rights derivatives.
- Large and restricted resources belong under `HALO_RESOURCE_ROOT` (default `$HOME/.local/share/halo-re`).
- Public tests and examples use project-authored redistributable fixtures.
- Ringhopper and other GPL tooling remain in separate repositories/processes unless this repository's licensing is deliberately changed after review.

## Architecture invariants

- Canonical documents are versioned project data, not serialized Bevy worlds.
- Persist stable typed IDs, never ECS entity IDs.
- `halo-domain` stays independent of Bevy, UI, physics, DCC tooling, networking, and legacy-format libraries.
- UI and runtime projections invoke project-owned domain commands rather than mutating authored documents directly.
- Source assets and generated artifacts have separate ownership.
- Simulation uses an explicit fixed timestep; rendering does not define authoritative cadence.
- Plugins provide mechanisms. Project code owns gameplay behavior and compatibility semantics.
- Add vertical slices, not speculative empty crates.

## Initial delivery order

1. Phase 0: Bevy foundation, canonical document spike, fixed-step trace, transform/inspector evidence.
2. Phase 1: native scenario editor with validation, undo/redo, save/recovery, and isolated play mode.
3. Phase 2: deterministic Blender/glTF interchange and asset processing.
4. Phase 3: operator-supplied HEK Tutorial reference plus a redistributable public analogue.

## Quality gates

For non-trivial changes, run:

```bash
cargo fmt --all --check
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

Tests should target domain behavior and deterministic simulation without requiring a window. Renderer/editor smoke tests are additive and may require a graphics-capable host.

Treat imported documents and assets as untrusted: constrain paths, bound allocations and recursion, avoid shell interpolation, time out subprocesses, and write atomically.
