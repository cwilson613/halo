# Architecture handoff

The architecture and research that authorize this in-tree project live in the monorepo-level documentation:

```text
../../docs/ecosystem.md
../../docs/architecture/modern-runtime-toolchain.md
../../docs/modern-runtime-roadmap.md
../../docs/research/bevy-ecosystem-2026-08.md
../../docs/research/discord-reference-assessment-2026-08.md
```

These are evidence and architecture inputs, not Cargo or Nix build dependencies. Preserve links to evidence and commit identities when promoting decisions into this project.

Initial upstream planning commit:

```text
e405c9a docs: define modern Bevy runtime roadmap
97db377 docs: diagram project ecosystem
```

The primary legacy environment target is the Tutorial level from the original Gearbox Halo Editing Kit. Its source tags and any `tutorial.map` remain operator-supplied under `HALO_RESOURCE_ROOT`; this repository will eventually contain a separately authored analogue for CI.

## Phase 0 host and Nix profile evidence

The Linux window/PBR probe created a Wayland window through Vulkan on an `AMD Radeon RX 9070 XT (RADV GFX1201)` using Mesa 26.1.6 and remained healthy for the 15-second probe.

The repository Nix flake owns package, app, development-shell, formatter, and check outputs for:

- `x86_64-linux`: Wayland-only with Vulkan and host graphics-driver access;
- `aarch64-darwin`: preliminary native Apple Silicon profile with Metal.

The Darwin outputs evaluate from Linux, but they remain unqualified until `nix flake check`, `nix build`, and an actual launch run on Apple Silicon macOS.
