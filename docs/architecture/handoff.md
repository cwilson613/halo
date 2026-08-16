# Architecture handoff

The architecture and research that authorized this repository currently live in the adjacent reconstruction checkout:

```text
/home/wilson/workspace/pig/halo/docs/ecosystem.md
/home/wilson/workspace/pig/halo/docs/architecture/modern-runtime-toolchain.md
/home/wilson/workspace/pig/halo/docs/modern-runtime-roadmap.md
/home/wilson/workspace/pig/halo/docs/research/bevy-ecosystem-2026-08.md
/home/wilson/workspace/pig/halo/docs/research/discord-reference-assessment-2026-08.md
```

Those paths are provenance references, not build dependencies. Before this repository is moved or published, copy the stable architecture decisions needed here while preserving links to upstream evidence and commit identities.

Initial upstream planning commit:

```text
e405c9a docs: define modern Bevy runtime roadmap
97db377 docs: diagram project ecosystem
```

The primary legacy environment target is the Tutorial level from the original Gearbox Halo Editing Kit. Its source tags and any `tutorial.map` remain operator-supplied under `HALO_RESOURCE_ROOT`; this repository will eventually contain a separately authored analogue for CI.
