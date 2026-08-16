# Custom Halo game

A clean-room, project-owned Halo-inspired game and content toolchain built with Rust and Bevy.

The modern game is tracked at `halo-bevy/` inside the overarching Halo monorepo. It remains architecturally isolated from the original-Xbox reconstruction build: reconstruction produces evidence and compatibility fixtures, not a source or link dependency. Original Halo executables, maps, tags, textures, sounds, and other proprietary content are not part of the tracked project.

## Platform baseline

## Nix profiles

The flake exposes the same development, build, and run interface on Linux and preliminary Apple Silicon macOS support:

```bash
nix develop                 # Rust toolchain and platform dependencies
nix build .#halo-game       # installable game package
nix run .#halo-game         # build and start the game
nix flake check             # package build and Cargo tests
```

Supported flake systems:

- `x86_64-linux`: validated Wayland/Vulkan profile;
- `aarch64-darwin`: preliminary native Apple Silicon/Metal profile, evaluated from Linux but requiring validation on macOS hardware.

Intel macOS is deliberately omitted because the pinned unstable Nixpkgs revision has dropped `x86_64-darwin`. Supporting it would require a second pinned Nixpkgs input and a real Intel Mac validation target.

The Linux shell supplies `pkg-config`, Wayland client development/runtime libraries, `libxkbcommon`, and the Vulkan loader, and selects Wayland for winit. The Darwin shell supplies the Apple frameworks required by winit/wgpu and selects Metal.

## Current phase

Phase 0 proves the minimum foundation:

- Bevy 0.19.1 application and PBR fixture;
- Bevy-independent domain documents with stable authored identities;
- domain-command mutation boundary;
- deterministic fixed-step trace tests;
- only one domain crate and one consuming application.

Run with Cargo inside the development profile:

```bash
nix develop
cargo test --workspace
cargo run -p halo-game
```

Build or run the Nix package directly:

```bash
nix build .#halo-game
nix run .#halo-game
```

The initial flake profiles support `x86_64-linux` with Wayland/Vulkan and `aarch64-darwin` with Metal. The macOS profile currently evaluates but still needs a native Apple Silicon build and launch qualification before it is considered proven.

## Layout

```text
apps/halo-game/         Bevy application
crates/halo-domain/     Bevy-independent authored data and commands
assets/fixtures/        Redistributable project-authored fixtures
docs/                   Architecture and handoff material
```

## Machine-local assets

Large, restricted, or operator-supplied resources live outside Git:

```bash
export HALO_RESOURCE_ROOT="${HALO_RESOURCE_ROOT:-$HOME/.local/share/halo-re}"
```

Do not symlink that tree into this repository. Public tests and examples must use redistributable project-authored fixtures.

## License

New project code is intended to be dual-licensed under MIT or Apache-2.0. Third-party dependencies and imported resources retain their own licenses. GPL tools such as Ringhopper remain separate processes/repositories rather than linked dependencies here.
