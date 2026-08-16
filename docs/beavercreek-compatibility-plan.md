---
id: beavercreek-compatibility-playtest
title: "Beavercreek compatibility load and playtest plan"
status: decided
tags: [modern-runtime, bevy, compatibility, beavercreek, tutorial, reconstruction]
open_questions: []
dependencies:
  - modern-runtime-foundation
  - native-scenario-editor
related:
  - deterministic-asset-pipeline
priority: 3
---

# Beavercreek compatibility load and playtest plan

## Decision

Use the operator-supplied HEK Tutorial as the **format and import calibration target**, then use operator-supplied Beavercreek content as the first recognizable **environment load and movement playtest** target.

The content-import lane and reconstruction-evidence lane proceed in parallel:

- content inspection and import establish what bytes and authored records mean;
- reconstructed C, headers, disassembly, and differential traces establish runtime lifecycle and behavior;
- only independently verified observations cross into the native domain model, compiler, fixtures, or Bevy gameplay systems.

Do not wait for a complete reconstructed C corpus before beginning read-only content inventory and import. Conversely, do not infer original runtime behavior from imported data structures alone.

## Definition of the first Beavercreek playtest

The first milestone is intentionally narrower than a complete Halo multiplayer match. It is complete when a local, operator-supplied Beavercreek reference can be inspected, transformed into native derived artifacts, and run in Bevy with:

- one source-derived player start;
- BSP render geometry;
- documented basic material and lightmap mapping;
- BSP collision sufficient to traverse representative floors, ramps, walls, ledges, and boundaries;
- fixed-step walking, looking, jumping, and crouching;
- deterministic restart to the same selected spawn under the same inputs;
- a machine-readable coverage and semantic-loss report;
- no proprietary source or derived payload committed to Git;
- a project-authored redistributable analogue exercising the same importer-normalization, compiler, loading, collision, spawn, and movement interfaces in CI.

The following are explicitly outside this first milestone: networking, complete multiplayer rules, vehicles, AI, weapons, exact shader fidelity, full audio, all devices/scripts, and byte-identical cache reproduction.

## System boundary

```text
operator-supplied HEK tags or compatible cache map
    -> pinned external read-only inspector
    -> versioned observation inventory
    -> project-owned normalized environment IR
    -> deterministic compiler
    -> derived mesh/material/collider artifacts + runtime manifest
    -> native ScenarioDocument references and placements
    -> Bevy projection
    -> fixed-step play session
```

Ringhopper and Invader remain external GPL-compatible processes. They communicate through documented files or process output; neither is linked into `halo-domain`, the permissive compiler core, or the runtime. The native editor and runtime must start and operate on native/project-authored data without either tool installed.

## Evidence classes

Every imported or compatibility-relevant field carries, directly or through its containing record:

- source engine, platform, build, and content kind;
- source artifact hash and local provenance-record identity;
- inspection tool and immutable revision;
- state: `observed`, `derived`, `absent`, `unsupported`, or `not_inspected`;
- confidence and derivation notes where interpretation is not direct;
- stable source identity sufficient to correlate reimports;
- diagnostics for truncation, unsupported semantics, and destructive changes.

A field being present in HCEA, Gearbox PC/Custom Edition, retail Xbox, or a community corpus does not establish that it has the same meaning in another build.

## Work lanes

### Lane A — Native scenario and play-session substrate

Owner: `halo-domain` and its consuming application until another crate has an immediate consumer.

Minimum model:

- typed `ScenarioId`, `ObjectId`, `AssetId`, and `PlayerStartId`;
- environment/BSP asset reference;
- scenery placement with stable source identity;
- player start with position, orientation, team/type metadata when observed, and explicit unsupported fields;
- project coordinate/unit convention;
- validation diagnostics with document/object/property identity;
- command transactions and snapshot-backed undo/redo;
- deterministic canonical save/reload;
- isolated play snapshot that cannot mutate the authored document or undo history.

Gate A acceptance:

1. A project-authored room document can be saved, reopened, edited, undone/redone, and validated through shared domain APIs.
2. Entering play mode consumes a snapshot; leaving play mode restores the unchanged authoring state.
3. Headless tests prove stable IDs, deterministic serialization, transaction behavior, and play isolation.
4. No Bevy, UI, physics, or legacy-format types enter `halo-domain`.

### Lane B — Read-only observation contract

Define a bounded, versioned interchange document before writing a broad importer. It records inventory and normalized observations, not proprietary payload bytes.

Minimum inventory:

- scenario identity and type;
- engine/build identity;
- tag dependency graph with group/class and source identity;
- BSP count and bounds;
- render geometry, material-slot, lightmap, and collision statistics;
- player/netgame starts;
- scenery/equipment/device/volume placement counts and transforms;
- coordinate, unit, handedness, and orientation observations;
- unsupported and not-inspected sections;
- source/tool hashes and revisions.

Security limits:

- accept paths only beneath an approved local resource root;
- reject traversal, unexpected absolute paths, and escaping symlinks;
- bound file size, record count, nesting depth, string length, and output size;
- invoke tools with argument arrays, captured output, and timeouts;
- write inventories and derived outputs atomically beneath dedicated local derived/cache roots;
- never copy original tag/map payloads into tracked fixtures.

Gate B acceptance:

1. Repeated inspection of unchanged Tutorial inputs produces semantically and byte-stable inventory output.
2. Malformed and oversized synthetic fixtures fail with bounded diagnostics.
3. Ringhopper and, where useful, Invader observations can be compared without either output silently becoming canonical truth.
4. The same schema can represent Tutorial and Beavercreek without map-name-specific fields.

### Lane C — Tutorial calibration slice

Tutorial calibrates the pipeline because source tags and documentation provide bounded authoring evidence. A compiled `tutorial.map`, if supplied, is complementary cache evidence.

Implementation order:

1. inventory and provenance;
2. coordinate/unit transform proof using named points and bounds;
3. one BSP's render topology and material slots;
4. lightmap association and documented fallback material mapping;
5. collision surfaces and material classes needed by the movement course;
6. scenario player start and a bounded subset of placements;
7. deterministic artifact manifest and rebuild comparison;
8. representative screenshot and collision-trace review.

Gate C acceptance:

- topology/count differences are explained rather than hidden;
- representative transforms round-trip within stated tolerances;
- the player can traverse a project-defined Tutorial movement course;
- unchanged inputs produce identical normalized IR, manifest, and derived-artifact hashes where the chosen encodings permit;
- unsupported source features survive in the observation/semantic-loss report;
- a redistributable authored analogue exercises every accepted code path in CI.

Tutorial passing authorizes applying the importer to Beavercreek; it does not establish general Halo map support.

### Lane D — Beavercreek environment load

Apply the calibrated contracts without adding Beavercreek-specific parser branches.

Implementation order:

1. acquire/identify the exact operator-supplied source and record provenance outside Git;
2. emit the common inventory and compare its feature set with Tutorial;
3. stop and extend normalized schemas only for genuinely new semantic classes, never for map names;
4. compile BSP geometry, basic materials/lightmaps, collision, and player starts;
5. load through the ordinary runtime manifest;
6. select a deterministic spawn policy;
7. run visual, collision, and restart traces;
8. record coverage, performance, and unsupported features.

Gate D acceptance:

- no Beavercreek name/path condition exists in parsing, normalization, compilation, or runtime loading;
- the map is loaded only from local derived artifacts associated with recorded source hashes;
- representative geometry/material views pass documented visual review;
- collision probes and the player movement route do not escape the intended playable volume;
- spawn selection and restart are deterministic;
- headless movement traces pass without rendering;
- frame time, fixed-step time, memory, artifact size, and load time are recorded;
- deleting local proprietary resources does not break public CI or native project-authored examples.

### Lane E — Reconstruction evidence promotion

Incoming C files and headers are indexed by source module, symbol/address, target build, provenance, and verification state. The operator-supplied clean HCAA SDL host package is an optional local differential oracle and lifecycle probe; its pinned identity, unresolved rights boundary, and isolated qualification procedure are recorded in [`research/hcaa-sdl-host-assessment-2026-08.md`](research/hcaa-sdl-host-assessment-2026-08.md). It is not linked into or copied into the native runtime.

Prioritize modules that constrain the playtest:

1. scenario/cache/tag loading and disposal;
2. structures/BSP initialization;
3. object placement and player creation;
4. player control and game time;
5. collision/point physics;
6. director, observer, and render cameras;
7. rasterizer/render material and visibility behavior.

For each candidate observation:

1. identify exact source module, symbol/address, target build, and provenance;
2. classify it as confirmed, inferred, comparative-only, or conflicting;
3. compare against retail-Xbox evidence, `kb.json`, lawful runtime traces, and version-aware content observations;
4. express the promoted result as a small testable compatibility record;
5. add a project-authored fixture or trace before changing native behavior;
6. implement behavior freshly in project-owned Rust or C as appropriate;
7. retain tolerances and known cross-version differences.

High-value fixture sequence:

- map lifecycle state/order trace;
- player-start filtering and deterministic selection;
- coordinate/orientation conversion points;
- fixed-tick input-to-motion trace;
- grounding, slope, step, and boundary contacts;
- observer/camera transform trace;
- material/lightmap classification cases.

C/header availability is never itself acceptance evidence. Differential execution or another independent check raises confidence; HCEA-derived material remains comparative-only until reconciled with the target build.

## Dependency and promotion gates

```text
Phase 0 closure
    -> Gate A native scenario/play substrate
    -> Gate B observation contract
    -> Gate C Tutorial calibration
    -> Gate D Beavercreek load/playtest

Lane E reconstruction evidence runs continuously and may tighten any gate,
but only a demonstrated contradiction blocks forward progress.
```

A gate may proceed with explicitly unsupported features. It may not proceed with silent data loss, unknown provenance, nondeterministic outputs, map-specific hacks, or proprietary fixtures in Git.

## Test matrix

| Layer | Public CI fixture | Local reference evidence |
|---|---|---|
| Domain | authored room/scenario | normalized inventories only |
| Inspector contract | synthetic valid/malformed inventories | Tutorial and Beavercreek tags/maps |
| Compiler | authored geometry/material/collision bundle | locally derived Tutorial/Beavercreek artifacts |
| Runtime load | authored analogue manifest | local Beavercreek manifest |
| Collision/movement | authored ramps, stairs, ledges, boundaries | selected Tutorial and Beavercreek routes |
| Spawn/restart | authored start set | observed source player starts |
| Visual review | authored golden views where stable | non-redistributed local captures/measurements |
| Reconstruction fixtures | project-authored state/trace cases | lawful target-build or differential observations |

## Milestones and commit-sized increments

1. **P0-close:** reconcile Phase 0 design status and evidence; no new engine scope.
2. **Scenario-v2-minimum:** typed asset/player-start references, coordinates, validation, migration tests.
3. **Transactions:** shared command transaction, snapshot undo/redo, deterministic save/reload.
4. **Play snapshot:** isolated headless play lifecycle and unchanged-document tests.
5. **Observation-schema:** versioned inventory structs/schema, strict parser, resource bounds, synthetic fixtures.
6. **Inspector-adapter:** one pinned external read-only process contract with provenance and timeout tests.
7. **Tutorial-inventory:** local inventory plus tracked redacted/statistical evidence record.
8. **Environment-IR:** BSP/material/collision/player-start normalized IR with deterministic serialization.
9. **Tutorial-render:** one BSP render path and authored analogue.
10. **Tutorial-collision:** collision artifact and fixed-step movement course.
11. **Beavercreek-inventory:** common-schema feature-gap report.
12. **Beavercreek-load:** ordinary manifest load with geometry/material/collision/spawn.
13. **Beavercreek-playtest:** deterministic movement/restart traces and performance report.
14. **Evidence-tightening:** promote incoming reconstruction observations as focused fixtures and corrections, independently of broad corpus completeness.

Each non-trivial increment includes tests and is committed independently after the repository quality gates pass.

## Stop conditions

Stop and resolve the evidence or interface boundary before proceeding if:

- content identity, engine version, or rights/provenance is unclear;
- an external tool would need to be linked into the permissive core;
- source and derived asset ownership becomes ambiguous;
- an importer requires a map-name-specific branch;
- coordinate conversion cannot be demonstrated with independent points/bounds;
- malformed input can escape roots or create unbounded work;
- a reconstructed-source claim conflicts with observed target-build behavior;
- play mode can mutate canonical authored state.

## Immediate next action

Close Phase 0 formally, then implement Gate A as the next vertical slice. In parallel, specify the Lane B observation document using only synthetic/project-authored fixtures; local Tutorial acquisition and inspection begins only after that contract and its safety limits exist.
