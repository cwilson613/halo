# Bevy ecosystem evidence survey — 2026-08-16

This is the first evidence pass for the proposed modern Halo runtime and artist-facing toolchain. It records what exists today, what is mature enough to evaluate, and what must remain project-owned. It is research, not a dependency lockfile or adoption decision.

## Executive findings

1. **Target Bevy 0.19 for prototypes, but isolate engine-facing code.** Bevy 0.19 is the current release. It adds next-generation code-defined scenes, built-in transform gizmos, asset saving, and substantial renderer work. Its `.bsn` file loader and editor write-back workflow are not finished, so BSN cannot yet be our only authored level format.
2. **There is no finished first-party Bevy editor to adopt.** The official `bevy_editor_prototypes` repository was archived in April 2026. Foundational work moved into Bevy; the community Jackdaw editor is now the recommended experiment, but describes itself as very early.
3. **Use Blender immediately for art and authored 3D layouts.** glTF/GLB is Bevy's established interchange path. Skein and Blenvy prove reflected Bevy component metadata can be authored in Blender. We should evaluate them, not make either format our canonical gameplay schema without a spike.
4. **Use a project-owned scenario document and compiler.** Halo-specific encounters, object palettes, trigger volumes, device wiring, spawn rules, AI orders, netgame flags, and validation need a stable schema independent of any one editor frontend or Bevy scene version.
5. **A project-owned in-engine scenario editor is justified.** Bevy 0.19 supplies picking-adjacent primitives, transform gizmos, reflection, UI/tooling foundations, and asset saving, but not the complete workflow. Artists need viewport placement, palettes, property inspectors, undo/redo, validation, and play-in-editor without editing Rust.
6. **TrenchBroom is a valuable greybox option, not the universal level editor.** `bevy_trenchbroom` provides maintained `.map`/`.bsp` integration and Avian collider support. It is strong for brush blocking and FPS iteration but weak for Halo terrain, final art, and rich scenario semantics.
7. **Physics and movement require behavioral evaluation.** Avian is Bevy-native and current; Rapier is older and more mature. `bevy_fps_controller` and Bevy Ahoy demonstrate FPS movement, but neither establishes Halo-compatible movement. We should use them as prototypes/reference, then own the controller and tune it against evidence.
8. **Do not choose netcode in the editor milestone.** Lightyear is the strongest current candidate for server-authoritative replication, prediction, interpolation, and rollback, but gameplay determinism and Halo compatibility need their own decision record.

## Baseline: Bevy itself

### Current release and editor foundations

Bevy 0.19 was released June 19, 2026. Relevant additions and caveats:

- next-generation scenes and the `bsn!` macro;
- composable, patchable, dependency-aware scene definitions in code;
- built-in transform gizmo support;
- `AssetSaver`-backed runtime/editor asset saving through `save_using_saver`;
- continued glTF support and renderer improvements;
- **no first-party `.bsn` asset loader in 0.19**;
- `.bsn` loading/saving and editor-produced assets remain future work.

Evidence:

- [Bevy 0.19 release notes](https://bevy.org/news/bevy-0-19)
- [Bevy glTF module documentation](https://dev-docs.bevy.org/bevy/gltf/index.html)
- [Bevy examples](https://bevy.org/examples)
- [Bevy asset processor documentation](https://docs.rs/bevy/latest/bevy/asset/processor/index.html)

**Consequence:** use Bevy scenes internally where useful, but retain a project-owned serializable authoring model until first-party scene assets and write-back stabilize. The compiler may emit Bevy-native runtime artifacts later.

### Official editor status

The official prototype repository says it was archived on April 16, 2026. Its notice says editor foundations continue in Bevy itself through BSN, UI, and camera/tooling work, while active experimentation continues in Jackdaw.

Evidence:

- [Archived Bevy Editor Prototypes](https://github.com/bevyengine/bevy_editor_prototypes)
- [Historical editor roadmap](https://bevyengine.github.io/bevy_editor_prototypes/roadmap.html)
- [Jackdaw](https://github.com/jbuehler23/jackdaw)
- [BSN editor infrastructure/write-back issue](https://github.com/bevyengine/bevy/issues/23637)

**Consequence:** do not wait for or depend on a complete upstream editor. Reuse Bevy primitives and study Jackdaw, but keep our editor narrow and Halo-domain-specific.

## Artist-facing editor and inspection building blocks

| Candidate | Evidence-backed capability | Current role in our plan | Main risk |
|---|---|---|---|
| Bevy transform gizmo | Built into Bevy 0.19 | Adopt for translate/rotate/scale spike | New API; selection/undo/document model still ours |
| Bevy Feathers | First-party tooling widget set introduced experimentally in 0.17 | Evaluate for native tool chrome | Experimental and still evolving |
| `bevy-inspector-egui` | Reflection-based entity/resource/asset inspection; Bevy 0.19 compatibility | Fast prototype and debugging inspector | Egui stack may not be the long-term product UI |
| Jackdaw | 3D hierarchy/inspector/viewport editor experiment; BSN work | Study architecture and contribute reusable fixes where sensible | Explicitly early and unstable |
| `space_editor` | Scene/prefab prototyping | Reference only | Unclear durability/version cadence |
| YOLECK | In-game, game-specific level editor kit | Study document/edit-mode patterns | Historically stronger for custom/game-specific workflows than full 3D DCC replacement |

Sources:

- [Bevy 0.17: Feathers tooling widgets](https://bevy.org/news/bevy-0-17)
- [`bevy-inspector-egui`](https://github.com/jakobhellermann/bevy-inspector-egui)
- [Jackdaw](https://github.com/jbuehler23/jackdaw)
- [`space_editor`](https://github.com/rewin123/space_editor)
- [YOLECK](https://github.com/idanarye/bevy-yoleck)

### Editor UX conclusion

The practical near-term stack is:

- Bevy viewport and runtime simulation;
- built-in transform gizmos where sufficient;
- `bevy-inspector-egui` for an expendable prototype inspector;
- a project-owned command/document layer for selection, undo/redo, dirty state, validation, save, and migrations;
- replacement of prototype UI panels with first-party Bevy UI/Feathers only when those APIs satisfy the workflow.

The command/document layer must not depend on egui. That preserves a migration path and enables headless compilation, CLI tooling, automated tests, and Omegon operations.

## Geometry and map-authoring ecosystem

### Blender + glTF/GLB

Bevy directly loads glTF scenes, meshes, materials, animations, skins, and extras. Two notable workflows add reflected Bevy component metadata:

- **Skein:** Blender extension plus Bevy plugin; reflected components are stored in glTF extras/extensions and instantiated in Bevy. Its compatibility table includes Bevy 0.19.
- **Blenvy:** Blender-centric levels, blueprints/prefabs, component editing, asset setup, and hot reload.

Evidence:

- [Skein repository](https://github.com/rust-adventure/skein)
- [Skein documentation](https://bevyskein.dev)
- [Blenvy repository](https://github.com/kaosat-dev/Blenvy)

**Recommendation for spike:** use Blender for meshes, materials, skeletons, animation, collision proxies, markers, and coarse layout. Compare plain glTF extras, Skein, and Blenvy against these requirements:

- deterministic export;
- stable object identity across re-export;
- prefab/blueprint references;
- component schema migration;
- source-control diff behavior;
- headless CI export;
- artist installation/upgrades;
- separation of visual geometry from scenario gameplay data.

Do not store all scenario semantics only inside a `.blend` file. The editable source may be Blender, but canonical gameplay data must be inspectable, migratable, and compilable without opening Blender.

### TrenchBroom and brush maps

TrenchBroom is a mature GPLv3+ cross-platform brush level editor. `bevy_trenchbroom` integrates TrenchBroom `.map`/`.bsp` data with Bevy and supports Avian collider creation.

Evidence:

- [TrenchBroom](https://trenchbroom.github.io)
- [TrenchBroom manual](https://trenchbroom.github.io/manual/latest)
- [`bevy_trenchbroom`](https://github.com/Noxmore/bevy_trenchbroom)
- [Foxtrot example game](https://github.com/janhohenheim/foxtrot)

**Recommendation:** support an optional TrenchBroom greybox importer if a two-day spike demonstrates fast iteration and robust conversion. Treat `.map` as an import/source format, not the final canonical world representation. Halo-style BSP spaces, terrain, portals, visibility, collision materials, and scenario objects need a compiler-owned intermediate representation.

### Terrain

`bevy_terrain` and other heightmap/procedural plugins exist, but the ecosystem does not show a stable, full artist-facing terrain editor comparable to mature commercial engines.

Evidence:

- [`bevy_terrain`](https://github.com/kurtkuehnert/bevy_terrain)
- [Bevy 0.19 release notes](https://bevy.org/news/bevy-0-19)

**Consequence:** terrain editing is a later dedicated workstream. Initially import terrain from Blender or heightmaps and focus the scenario editor on placement, volumes, paths, gameplay metadata, and validation.

## Physics, collision, navigation, and FPS movement

### Physics engines

| Candidate | Evidence | Assessment |
|---|---|---|
| Avian | ECS-driven Bevy-native 2D/3D physics; Avian 0.7 supports Bevy 0.19 | Preferred prototype because components/reflection/editor integration align with Bevy |
| Rapier / `bevy_rapier` | Mature, feature-rich Rust physics with official Bevy plugin | Keep as comparison/fallback; likely strongest maturity baseline |

Sources:

- [Avian](https://github.com/avianphysics/avian)
- [`bevy_rapier`](https://github.com/dimforge/bevy_rapier)

**Decision deferred:** build the same movement/collision test course against both where practical. Compare stepping, slopes, contact stability, continuous collision, triggers, moving platforms, determinism, query APIs, debug rendering, and editor collider visualization.

### Character/FPS controllers

- [`bevy_fps_controller`](https://github.com/qhdwight/bevy_fps_controller) supports Source-inspired movement and both Rapier/Avian backends.
- [Bevy Ahoy](https://github.com/janhohenheim/bevy_ahoy) provides a Bevy 0.19/Avian kinematic controller with ramps, ground snapping, steps, pushing, and Quake/Source movement behaviors.
- [Bevy's first-person view-model example](https://bevy.org/examples/camera/first-person-view-model) demonstrates separate view/world rendering layers.
- [`smooth-bevy-cameras`](https://github.com/bonsairobo/smooth-bevy-cameras) and Bevy's camera controller work are useful for editor navigation, not gameplay authority.

**Conclusion:** borrow architecture and test cases, not feel. Halo movement, aim, acceleration, crouch, jumping, vehicle transitions, melee, weapon timing, and networking must be project-owned and evidence-tuned.

### Navigation

- [`oxidized_navigation`](https://github.com/TheGrimsey/oxidized_navigation) asynchronously generates tiled navmeshes from Rapier, Avian, or custom Parry colliders.
- [`vleue_navigator`](https://github.com/vleue/vleue_navigator) provides Polyanya-based navigation and glTF mesh loading.

**Recommendation:** evaluate `oxidized_navigation` for generated runtime/editor preview navmeshes. Persist compiler inputs and settings, not opaque runtime-only state. Halo AI encounter/order semantics remain above pathfinding and belong in the scenario schema.

## Rendering, lighting, effects, and audio

### Lighting

Bevy supports imported lightmaps; Bevy 0.19 adds asset saving useful for editor/bake outputs. [`bevy-baked-gi`](https://github.com/pcwalton/bevy-baked-gi) demonstrates lightmapping, irradiance volumes, and related baked-GI workflows.

**Recommendation:** begin with Bevy PBR plus imported/baked lightmaps. Define bake products as derived artifacts with provenance. Do not make a custom lightmapper a phase-one dependency.

### VFX

[`bevy_hanabi`](https://github.com/djeedai/bevy_hanabi) is a maintained GPU particle system with Bevy 0.19 support. Community Hanabi editors/loaders exist, but there is no single canonical first-party VFX editor.

**Recommendation:** evaluate Hanabi for runtime effects, while designing effect assets behind a project-owned reference and validation layer. A specialized graph/editor panel can follow after the scene/scenario workflow works.

### Audio

Bevy's built-in audio covers basic playback and stereo spatial audio. [`bevy_kira_audio`](https://github.com/NiklasEi/bevy_kira_audio) offers richer playback controls.

**Recommendation:** defer backend selection. The authoring schema should describe emitters, gain/range, looping, classes, and references without binding scenarios to a particular Bevy audio plugin.

## Asset pipeline

Bevy's asset processor is explicitly a build system for artist-authored assets and supports loaders, transformers, savers, metadata, dependencies, and hot reload. This aligns with our desired source-to-runtime pipeline.

Proposed conceptual stages:

```text
DCC/source assets
  -> import adapters
  -> normalized project IR
  -> validation + dependency graph
  -> deterministic processors/bakers
  -> Bevy runtime assets + manifests
  -> hot-reloadable runtime/editor world
```

Candidate helpers:

- Bevy built-in asset processor and `AssetSaver`;
- [`bevy_asset_loader`](https://crates.io/crates/bevy_asset_loader) for loading-state ergonomics;
- [`bevy_common_assets`](https://github.com/NiklasEi/bevy_common_assets) for RON/JSON/YAML/TOML where appropriate.

**Rule:** source assets and derived artifacts are separate. Every derived output needs source identity, processor version/configuration, dependency hashes, and diagnostics. CI must be able to rebuild without GUI interaction.

## Networking evidence (not yet an adoption decision)

[Lightyear](https://github.com/cBournhonesque/lightyear) supports Bevy 0.19 and provides server-authoritative replication, prediction, interpolation, authority metadata, visibility, hierarchy propagation, pre-spawning, and deterministic input replication modes.

It is the leading candidate for a later networking spike. It must not shape the editor document model prematurely. Scenario data should be network-agnostic, while runtime components explicitly classify authority and replication.

## Proposed tool affordances

The original Halo CE toolkit separated concerns among a world/scenario editor, structured tag editor, and command-line build tools. We need comparable affordances without copying their UI or internal architecture:

| Affordance | Proposed tool surface |
|---|---|
| Spatial world/scenario editing | Bevy-native editor app with viewport, hierarchy, palettes, transforms, volumes, links, AI encounter/order overlays, nav/collision diagnostics, and play-in-editor |
| Structured asset/component editing | Reflection/schema-driven inspector panels plus dedicated editors for complex assets; Blender integration for art-centric data |
| Deterministic import/build/verify | Headless CLI and library operating on the same document/command/compiler APIs as the GUI |
| Legacy Halo interoperability | Optional Omegon-Ringhopper tools and explicit import/export adapters, never hidden runtime dependencies |
| Greyboxing | Blender and optional TrenchBroom importer |
| Runtime | Bevy app consuming compiled artifacts, not editor-only state |

The shared domain core is more important than matching the old executable split. GUI, CLI, CI, and Omegon should all call the same operations.

## Risks and validation spikes

| Spike | Question answered | Exit evidence |
|---|---|---|
| Bevy 0.19 foundation | Can we maintain a small runtime/editor workspace with fast iteration? | Window, PBR scene, hot reload, transform gizmo, tests, measured compile/run loop |
| Blender interchange | Plain glTF vs Skein vs Blenvy? | Round-trip scene with stable IDs, prefab, collider, marker, reflected component, headless export |
| Scenario document | Can GUI and CLI edit/validate the same domain model? | Versioned fixture, migration, deterministic serialization, undoable commands, diagnostics |
| Editor shell | Can an artist place and edit objects without Rust? | Viewport/hierarchy/palette/inspector/save/reload/play workflow |
| Physics course | Avian or Rapier for Halo-like behavior? | Repeatable slope/step/platform/projectile/trigger benchmarks and tuning report |
| TrenchBroom | Is brush greyboxing worth supporting? | Imported test map with materials, entities, collision, re-import behavior |
| Ringhopper bridge | Can legacy metadata/assets enter through explicit provenance-aware adapters? | Read-only map/tag query and one normalized import fixture |

## Adoption posture

- **Adopt now for prototyping:** Bevy 0.19, glTF/GLB, Blender, Bevy transform gizmos, Bevy asset APIs.
- **Evaluate with bounded spikes:** Avian, Rapier, Skein, Blenvy, TrenchBroom integration, `bevy-inspector-egui`, Jackdaw patterns, Oxidized Navigation, Hanabi.
- **Own:** scenario schema, stable IDs, command/undo model, compiler IR, validation, provenance, runtime gameplay, Halo-feel movement, editor workflows, migration policy.
- **Defer:** final networking library, terrain editor, full VFX/animation graph tools, custom lightmapper, general-purpose Bevy IDE.

## Evidence quality notes

- Bevy release notes and Bevy API documentation are authoritative for first-party capabilities.
- Project repositories and their compatibility tables are authoritative for stated support, not proof of production fitness.
- Search results and showcases establish existence, not quality. Every candidate dependency still requires a pinned-source review, license check, maintenance/activity check, minimal build, and project-specific spike.
- No dependency version in this document is an approved lock. Compatibility must be rechecked when implementation begins.
