# Custom Halo game

A clean-room, project-owned Halo-inspired game and content toolchain built with Rust and Bevy.

This repository is independent from the original-Xbox reconstruction repository. The reconstruction produces evidence; it is not a source dependency. Original Halo executables, maps, tags, textures, sounds, and other proprietary content are not part of this repository.

## Current phase

Phase 0 proves the minimum foundation:

- Bevy 0.19.1 application and PBR fixture;
- Bevy-independent domain documents with stable authored identities;
- domain-command mutation boundary;
- deterministic fixed-step trace tests;
- only one domain crate and one consuming application.

Run:

```bash
cargo test --workspace
cargo run -p halo-game
```

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
