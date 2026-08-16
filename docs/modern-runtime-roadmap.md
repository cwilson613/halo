# Modern runtime delivery plan

This plan turns [`architecture/modern-runtime-toolchain.md`](architecture/modern-runtime-toolchain.md) into evidence-producing increments. Dates are intentionally absent: phases advance on exit evidence, not optimism.

The existing C/XBE reconstruction remains operational throughout. Modern work begins in `modern/` only after Phase 0 approves the workspace baseline.

## Delivery rules

- Each phase produces a runnable or inspectable artifact.
- Spikes are time-boxed and disposable unless their exit review explicitly promotes them.
- A dependency is not “chosen” because a demo starts; it graduates through a decision gate.
- Artist-facing acceptance is required for authoring milestones.
- Compatibility claims cite observed fixtures and uncertainty.
- No proprietary game content is needed for CI.
- Prefer one vertical slice over parallel skeleton crates.

## Phase 0 — Foundation and decision harness

### Objective

Prove that Bevy 0.19 and the proposed Rust boundaries can support a small modern runtime/editor loop on the current development hosts.

### Deliverables

- minimal `modern/` Cargo workspace with only crates needed by the slice;
- pinned Rust and dependency policy;
- a Bevy application that opens a PBR scene;
- fixed-step simulation resource and deterministic test clock;
- project-authored fixture asset loaded through Bevy;
- transform-gizmo and editor-camera experiment;
- headless unit/integration test path;
- measured clean/incremental build and startup times;
- architecture decision records for promoted dependencies.

### Spikes

1. Bevy default feature audit and compile-time profile.
2. Native Bevy transform gizmo in a simple viewport.
3. egui/`bevy-inspector-egui` prototype behind an adapter-free view model.
4. Canonical document serialization comparison with merge and migration fixtures.

### Exit criteria

- Linux development build and tests pass reproducibly.
- A project-authored object can be loaded, selected, moved, and inspected.
- A fixed-step simulation test produces the same trace across repeated runs.
- Canonical document format is selected with deterministic output and migration proof.
- Known build/iteration budgets and top bottlenecks are recorded.
- No crate exists without an immediate consumer.

### Explicit exclusions

No original assets, complete editor, networking, AI, or general asset compiler.

## Phase 1 — Native scenario document and editor shell

### Objective

Allow a non-programmer to create, edit, validate, save, reload, and play a small project-authored scenario.

### Deliverables

- versioned `ScenarioDocument`, `PrefabDocument`, typed stable IDs, and references;
- command transactions with undo/redo;
- hierarchy, palette, selection, transform, and inspector panels;
- atomic save, dirty-state handling, external-change detection, and recovery file;
- validation diagnostics linked to objects/properties;
- isolated play-in-editor snapshot;
- CLI commands for inspect, validate, format, and migrate;
- small legal fixture scenario under version control.

### Artist workflow acceptance

An artist can, without changing Rust:

1. open the fixture project;
2. place scenery and a spawn point from a palette;
3. transform and configure them;
4. create a trigger volume;
5. undo/redo the operations;
6. resolve a deliberately introduced validation error;
7. save, restart, and recover the same content;
8. enter and leave play mode without modifying the document.

### Exit criteria

- GUI and CLI apply the same domain commands and validation rules.
- Stable IDs survive rename, reorder, save/reload, and Blender reimport tests.
- Undo/redo and crash-safe saves have automated tests.
- No persisted ECS entity IDs or UI types appear in documents.

## Phase 2 — Art interchange and deterministic asset build

### Objective

Create a reliable Blender-to-runtime path with explicit provenance, reimport, dependencies, and hot reload.

### Spikes

Compare:

- plain glTF/GLB plus project metadata;
- Skein;
- Blenvy.

Optional: test TrenchBroom import separately for greyboxing.

### Comparison fixture

The same authored room must contain:

- static render geometry and materials;
- collision proxy;
- socket/marker;
- reusable prefab/blueprint instance;
- stable component metadata;
- one animation if supported;
- re-export after rename and hierarchy change.

### Deliverables

- selected Blender workflow and documented add-on/export setup;
- headless deterministic export in CI or a documented evidence-backed exception;
- import adapter to normalized project IR;
- source/derived asset manifests and content hashes;
- dependency invalidation and incremental rebuild;
- editor asset browser, import status, diagnostics, and reimport;
- hot reload into editor and runtime;
- optional TrenchBroom decision record.

### Exit criteria

- clean build from source assets succeeds without interactive GUI steps.
- repeated unchanged builds produce identical artifact hashes.
- changing one source invalidates only its dependent artifacts.
- reimport preserves scenario references and reports destructive changes.
- licensing/provenance is visible for every fixture and dependency.

## Phase 3 — Playable compatibility slice

### Objective

Deliver a small first-person combat sandbox that exercises the compatibility architecture end to end.

### Primary reference target: HEK Tutorial

Use the `tutorial` level distributed with the original Gearbox Halo Editing Kit as the first environment/map/scenario compatibility target. Public references identify its source scenario as `tags\levels\test\tutorial\tutorial.scenario`; an operator may also compile or supply `tutorial.map` for cache-level comparison.

This is a strong first target because it was intentionally shipped as a compact, documented example of Halo Custom Edition's end-to-end level workflow. Its associated tutorial and focused tag set provide a bounded reference for environment construction, materials, collision, portals, object placement, spawn metadata, scenery, equipment, and basic playtesting. The exact coverage must be established by inventorying the acquired HEK version rather than assumed from memory.

#### Input and legal boundary

- The original HEK installer, tags, data, tutorial documentation, and compiled map are operator-supplied external resources and remain outside Git.
- Acquisition records use the repository's provenance conventions and identify the exact distributor, package version, hashes, and relevant file paths.
- Do not redistribute HEK content or derived payloads through this repository.
- CI and public tests use a separately authored redistributable analogue containing only the mechanics and affordances needed by the test matrix.
- Treat `tutorial.scenario` and its source tags as the authoring-semantic reference; use `tutorial.map` as a compiled-cache comparison artifact. A map cache alone is not a substitute for source-authoring evidence.

#### Tutorial evidence package

Before implementing the environment, produce a machine-readable inventory that records:

- source and compiled artifact hashes and provenance;
- scenario type and engine/build identity;
- referenced tag groups and dependency graph;
- BSP counts and geometry/material/collision statistics;
- object palettes and placed instances by category;
- player/netgame starts, equipment, scenery, devices, decals, lights, and sound placements where present;
- trigger volumes, scripts, encounters, AI, navigation, and other affordances as present or absent;
- coordinate, unit, orientation, and naming observations;
- screenshots or traces needed to establish expected rendering and collision behavior without committing proprietary payloads.

Every inventory field distinguishes observed, derived, absent, unsupported, and not-yet-inspected states.

#### Acceptance matrix

Track the native implementation by subsystem rather than a single “loads Tutorial” checkbox:

| Capability | Source evidence | Native acceptance |
|---|---|---|
| BSP/render geometry | source tags plus compiled cache | topology/material assignment and representative views match within documented limits |
| Collision | BSP collision and play traces | floor, wall, slope, stair, and boundary contacts pass the movement course |
| Materials/textures | shader/bitmap references | normalized material mapping is documented and visually reviewed |
| Scenario objects | palettes and placements | stable native IDs preserve type, transform, and source identity |
| Player starts | scenario spawn metadata | deterministic spawn and respawn behavior is testable |
| Equipment/scenery | scenario placements | supported objects compile, appear, and participate in interaction/collision as applicable |
| Volumes/devices/scripts | present source features | each is either implemented, represented losslessly for later work, or reported unsupported |
| Audio/lighting | source references and observations | initial approximation and known semantic gaps are explicit |
| Build determinism | normalized IR and artifacts | unchanged inputs produce identical native artifact hashes |

The public analogue must exercise the same accepted native code paths so CI does not silently depend on local HEK resources.

### Scope

- one native reconstruction of the operator-supplied HEK Tutorial reference, paired with a project-authored public analogue;
- one player spawn;
- walking, looking, jumping, crouching, slopes, stairs, and moving platform;
- shields/health/damage/death/respawn;
- one hitscan weapon, one projectile weapon, melee, and grenade;
- one device/trigger interaction;
- HUD and basic audio;
- deterministic trace capture.

### Physics gate

Implement the shared movement/collision test course against Avian first and Rapier where needed for comparison. Measure:

- ground contact stability;
- step and slope behavior;
- high-speed collision;
- triggers and moving platforms;
- query ergonomics;
- debug visualization;
- simulation repeatability and performance.

Promote one backend; document why and when to reconsider.

### Exit criteria

- the entire slice is authored through native documents and the editor.
- gameplay runs at an explicit fixed simulation rate.
- project-owned movement and weapon tests pass without rendering.
- backend-specific code remains in the Bevy integration boundary.
- observed-vs-designed behavior is labeled in the compatibility report.
- representative frame/simulation budgets are recorded.

## Phase 4 — Scenario systems and AI sandbox

### Objective

Prove that Halo-like scenario semantics can be authored and simulated above generic navigation.

### Deliverables

- encounters, squads, starting locations, orders, trigger references, and spawn conditions;
- visual overlays and dedicated inspector affordances;
- navmesh generation/import spike, initially evaluating Oxidized Navigation;
- AI state and order execution for a small encounter;
- path/debug visualization;
- compiler validation for unreachable references, invalid ordering, and missing navigation;
- deterministic AI fixture tests where feasible.

### Exit criteria

- an artist authors and debugs a multi-wave encounter without Rust changes.
- scenario semantics do not leak into the selected pathfinding crate.
- navigation data is reproducibly generated or imported.
- AI diagnostics identify scenario objects and source properties.

## Phase 5 — Legacy interoperability vertical slice

### Objective

Demonstrate explicit, lawful, provenance-aware import from operator-supplied Halo content into the native model.

### Deliverables

- completed independent `omegon-ringhopper` read-only integration;
- one pinned Invader Tutorial build/differential experiment, kept as an optional external GPL process rather than a linked dependency;
- documented process/file contracts between GPL tooling and permissive project components, subject to license review;
- one bounded import adapter for selected metadata or one asset class;
- normalized output with source hashes, tool revisions, and diagnostics;
- differential fixture comparing Ringhopper observations with project parsing/IR;
- no hidden dependency on local Halo installation paths.

### Exit criteria

- native editor/runtime start and operate without Ringhopper.
- import is operator-invoked and auditable.
- no proprietary source or derived payload is committed.
- round-trip or semantic-loss report clearly identifies unsupported data.

## Phase 6 — Packaging and creator preview

### Objective

Make the modern toolchain usable by a small external creator cohort with project-authored content.

### Deliverables

- reproducible editor/runtime/CLI packages for supported desktop platforms;
- project template and sample scenario;
- first-run dependency and GPU diagnostics;
- crash recovery and actionable logs;
- content build caching;
- compatibility/migration policy for documents;
- creator documentation and structured feedback loop.

### Exit criteria

- a fresh machine can install tools, open the sample, edit, build, and run it from documented steps.
- project files survive a schema migration rehearsal.
- failures do not corrupt source documents or derived caches.
- feedback identifies no workflow requiring undocumented Rust edits for the supported slice.

## Phase 7 — Networking decision and multiplayer slice

### Objective

Select and prove a network architecture only after the simulation and content model are stable enough to test.

### Leading candidate

Lightyear, because current evidence shows server-authoritative replication, prediction, interpolation, authority metadata, visibility, hierarchy propagation, and Bevy 0.19 support.

### Required spike

- dedicated server and two clients;
- authoritative movement and firing;
- client prediction/reconciliation;
- remote interpolation;
- projectile/hitscan authority and lag handling;
- object spawn/despawn and hierarchy;
- bandwidth, latency, packet-loss, and correction metrics;
- replayable input/state traces.

### Exit criteria

- library choice has measured project evidence, not ecosystem popularity alone.
- gameplay code classifies authority/prediction/presentation explicitly.
- content documents remain transport-agnostic.
- offline simulation tests remain valid.

## Cross-cutting workstreams

### Compatibility evidence

Continuously add small lawful fixtures and traces. Every result records provenance, confidence, platform/version, and tolerances.

### Tool UX

At each content milestone, test with someone who did not implement it. Capture task completion, errors, and iteration latency. Generic inspectors are replaced where they obstruct frequent workflows.

### Security and robustness

Treat imported documents and assets as untrusted:

- validate paths and archive entries;
- cap sizes/counts/depth;
- avoid shell interpolation;
- time out child processes;
- isolate writes to project/cache roots;
- preserve originals and use atomic replacement;
- fuzz parsers and migration boundaries where practical.

### Dependency governance

Pin versions, review licenses and maintenance, minimize Bevy feature sets, run update spikes on branches, and keep adapters around volatile plugins.

### Performance

Establish representative scenes early. Track compile time, startup, editor latency, import/rebuild latency, simulation time, render frame time, memory, and artifact size.

## First backlog after documentation approval

1. Create a design record for the Bevy foundation spike.
2. Establish `modern/` with one application and one domain library only.
3. Add project-authored cube/room fixture and provenance.
4. Implement fixed-step trace test.
5. Exercise transform gizmo and editor camera.
6. Compare canonical document serialization with realistic diffs and migration.
7. Prototype a UI shell without placing domain mutations in UI systems.
8. Record build/startup measurements and decide whether Phase 0 exits.

Everything beyond this list remains planned, not implicitly authorized implementation.
