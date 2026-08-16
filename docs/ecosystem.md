# Halo reconstruction and modern-tooling ecosystem

This is a compact visual index of the plans in:

- [`architecture/modern-runtime-toolchain.md`](architecture/modern-runtime-toolchain.md)
- [`modern-runtime-roadmap.md`](modern-runtime-roadmap.md)
- [`research/discord-reference-assessment-2026-08.md`](research/discord-reference-assessment-2026-08.md)

The diagrams deliberately show ownership and handoff boundaries rather than every crate, tag group, or community project.

## 1. Project tracks and evidence flow

```mermaid
flowchart LR
    Original[Original Xbox executable] --> Recon[C / XBE reconstruction]
    Recon --> Evidence[Verified behavior and ABI evidence]

    HEK[Operator-supplied HEK content] --> Legacy[Legacy inspection and import]
    Community[c20, OpenSauce, HaloCEA,<br/>other research leads] -. hypotheses .-> Evidence
    Legacy --> Evidence

    Evidence --> Native[Native domain model and tests]
    Native --> Tools[Scenario editor and CLI]
    Native --> Runtime[Bevy runtime]

    Public[Project-authored public fixtures] --> Native
```

Key boundary: community corpora can generate hypotheses; only verified observations become compatibility evidence. The modern runtime consumes evidence, not reconstruction source code.

## 2. Content and build flow

```mermaid
flowchart LR
    subgraph Sources[Authored and supplied sources]
        Blender[Blender / glTF]
        Scenario[Native scenario documents]
        Legacy[HEK tags and maps]
    end

    subgraph Domain[Project-owned content core]
        Import[Explicit import adapters]
        Model[Versioned documents<br/>stable IDs and commands]
        Validate[Validation and diagnostics]
        Compile[Deterministic compiler and bakers]
    end

    subgraph Products[Consumers]
        Editor[Bevy scenario editor]
        CLI[Headless CLI and CI]
        Artifacts[Content-addressed runtime artifacts]
        Game[Bevy runtime]
    end

    Blender --> Import
    Legacy --> Import
    Import --> Model
    Scenario <--> Model
    Model --> Validate
    Model --> Compile
    Validate --> Editor
    Validate --> CLI
    Compile --> Artifacts --> Game
    Model --> Editor
    Model --> CLI
```

Source assets and canonical documents are preserved. Generated meshes, colliders, lightmaps, navigation, and packages are derived outputs—not authoring truth.

## 3. Local storage and repository boundaries

```mermaid
flowchart TB
    Git[halo Git repository<br/>code, docs, public fixtures]
    RingRepo[Independent omegon-ringhopper repository]

    subgraph Local[HALO_RESOURCE_ROOT<br/>machine-local and outside Git]
        Download[Acquisition originals]
        Restricted[HEK and restricted source]
        Derived[Derived imports and builds]
        Cache[Disposable caches]
        Research[Quarantined research artifacts]
    end

    Download --> Restricted
    Restricted --> Derived
    Research -. inspected evidence .-> Git
    Restricted -. explicit adapters .-> RingRepo
    RingRepo -. structured read-only results .-> Git
    Git --> Derived
    Derived --> Cache
```

Large or restricted resources live under `~/.local/share/halo-re`. There is no repository symlink and the Ringhopper extension remains a separate checkout rather than a submodule.

## 4. Planned delivery ladder

```mermaid
flowchart LR
    P0[Phase 0<br/>Bevy foundation] --> P1[Phase 1<br/>Scenario editor]
    P1 --> P2[Phase 2<br/>Asset pipeline]
    P2 --> P3[Phase 3<br/>HEK Tutorial slice]
    P3 --> P4[Phase 4<br/>Scenario AI]
    P4 --> P5[Phase 5<br/>Legacy interoperability]
    P5 --> P6[Phase 6<br/>Creator preview]
    P6 --> P7[Phase 7<br/>Networking decision]
```

Each transition is evidence-gated. A later phase does not authorize speculative scaffolding in an earlier one.

## 5. HEK Tutorial compatibility ladder

```mermaid
flowchart LR
    Source[tutorial.scenario<br/>source tags and documentation]
    Cache[tutorial.map<br/>compiled-cache evidence]
    Inventory[Provenance and affordance inventory]
    IR[Normalized native representation]
    Slice[Playable native Tutorial slice]
    Analogue[Redistributable public analogue]
    Tests[Shared compatibility tests]

    Source --> Inventory
    Cache --> Inventory
    Inventory --> IR --> Slice
    Analogue --> Tests
    Slice --> Tests
```

The operator-supplied Tutorial artifacts stay outside Git. The public analogue exercises the same native import-independent runtime paths in CI.

## 6. External ecosystem posture

```mermaid
flowchart TB
    c20[c20<br/>formats and workflows] --> Questions[Research questions and fixtures]
    Sauce[OpenSauce<br/>CE extension semantics] --> Questions
    HCEA[HaloCEA and related corpus<br/>comparative hypotheses] --> Questions
    HaloX[HaloX / MCC experiments] --> Future[MCC interoperability backlog]
    Spawns[Halospawns<br/>map/replay analytics] --> Future
    Ring[Ringhopper<br/>typed map and tag inspection] --> Adapter[Optional read-only adapter]
    Invader[Invader<br/>legacy build and transformation] --> Legacy[Optional build/reference workflow]

    Questions --> Verify[Project-owned verification]
    Verify --> Decisions[Compatibility decisions]
    Adapter --> Decisions
    Legacy --> Decisions
```

External projects are not a universal dependency graph. They occupy bounded roles:

- **c20:** terminology, format, and workflow index;
- **OpenSauce:** historical Custom Edition extension semantics;
- **HaloCEA/corpora:** version-labeled hypotheses requiring independent verification;
- **HaloX:** future MCC module-hosting research;
- **Halospawns:** map catalog, replay, heatmap, region, and analytics concepts;
- **Ringhopper:** explicit structured inspection at a separate GPL process/repository boundary;
- **Invader:** optional pinned legacy tag/cache build, extraction, comparison, and transformation reference, also behind a GPL process boundary. See the [dated assessment](research/invader-assessment-2026-08.md).

## Halospawns relevance

Authenticated inspection of the shared development site showed a Svelte application backed by a separate API. The frontend exposes concepts including:

- map catalog, releases, variants, families, and similarity suggestions;
- GLB/JSON map assets and generated thumbnails;
- replay-to-map matching;
- player and game statistics;
- positional heatmaps;
- named map regions, region sets, transforms between related maps, and region statistics;
- map reprocessing and screenshot regeneration workflows.

This is useful as evidence for **analytics and map-coordinate tooling**, not as a core runtime dependency. The strongest ideas to carry forward are:

1. explicit map release/build/family identity;
2. coordinate-space metadata and transforms between related map versions;
3. authored named regions layered over map geometry;
4. derived heatmaps and replay observations kept separate from canonical level content;
5. reproducible map-processing and screenshot jobs.

The Basic-auth credential gates the website only. The API uses a separate authentication system, so the shared credential did not authorize API data access. No API payloads or proprietary map assets were downloaded.
