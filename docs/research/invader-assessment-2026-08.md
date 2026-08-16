# Invader ecosystem assessment — 2026-08

## Scope and snapshot

This assessment covers [SnowyMouse/invader](https://github.com/SnowyMouse/invader) as a candidate research, interoperability, and legacy-content build tool for the original-Xbox reconstruction and the independent Bevy game/toolchain.

Inspected snapshot:

| Field | Value |
|---|---|
| Repository | `https://github.com/SnowyMouse/invader.git` |
| Branch | `master` |
| Commit | `928c17ed728539377c4bd1b19a29aaed0c3d65c3` |
| Commit date | 2026-04-06 |
| Latest release | `0.55.0`, published 2025-10-04 UTC |
| Release commit | `33518109f2d7d1694a41b4ea7e957d244eb1866f` |
| Distance from release | 9 commits |
| License | GPL-3.0-only, with separately identified bundled components |
| Local research checkout | `$HALO_RESOURCE_ROOT/source/community/invader/upstream` |

GitHub reported that the repository was created in 2018, is not archived, and had been pushed in April 2026. The inspected history contains 4,143 commits and continued compatibility fixes after the 0.55.0 release. Windows release archives are published; non-Windows use is source-build oriented.

## Executive assessment

Invader is a mature, broad **legacy Halo content toolchain**, not just a map parser. Its strongest project value is as an external reference implementation and optional operator-invoked builder for HEK-style tags, cache files, resource maps, scripts, bitmaps, sounds, models, and repair/conversion workflows.

It does **not** replace Ringhopper in the current architecture:

- Ringhopper is the better fit for the implemented typed, bounded, read-only Omegon inspection service.
- Invader is substantially broader on authoring, compilation, extraction, repair, and cache generation.
- Both are GPL-3.0-only and therefore remain outside the permissive Bevy domain/runtime build graph.
- Differential results from both tools are evidence, not canonical truth.

The immediate high-value experiment is to build the HEK Tutorial scenario with pinned Invader in a quarantined workspace, record every input and output hash, inspect the result with both Invader and Ringhopper, and compare it with the operator-supplied reference. Invader should not become the native Bevy asset compiler or an ordinary runtime dependency.

## Licensing and distribution boundary

`LICENSE.md` states that Invader itself is licensed under **GPL version 3 only**, not “or later.” It also says that Invader does not impose its license on content created with it, while correctly noting that pre-existing Halo content retains its original rights.

The repository identifies exceptions or bundled components including:

- BSD-licensed ADPCM encoder code;
- MIT-licensed Xbox ADPCM modifications;
- Apache-2.0 Roboto Mono;
- public-domain/dual-licensed STB code.

The build also uses a `riat` Git submodule and system or fetched dependencies. A distributable build therefore needs a complete dependency/license inventory rather than relying only on the root license statement.

Project consequences:

1. Invader executables may be invoked as separate GPL programs.
2. Directly linking `libinvader` into a permissively licensed Bevy executable or library is rejected absent a deliberate licensing change and legal review.
3. Invader source or generated parser code must not be copied into `halo-domain`, the reconstruction sources, or permissive adapters.
4. Project-authored output is not made GPL merely because Invader processed it; source-content rights and provenance still govern redistribution.
5. Any project-distributed Invader wrapper/bundle must independently satisfy GPL and third-party license obligations.

## Architecture and implementation surface

Invader is primarily C++20/C11, built with CMake. Its declared full-build dependencies include Python, zlib, TIFF, Vorbis, libsamplerate, Qt 6, SDL2, Corrosion, and the Rust-based `riat` script compiler; LibArchive and FreeType support optional tools. The default build is therefore materially heavier than Ringhopper's Rust workspace and the current narrow Omegon extension.

The repository has:

- a reusable `invader` library target;
- generated C++ parser code from JSON HEK tag definitions;
- map, tag, resource-map, extraction, compilation, bitmap, sound, model, CRC, and filesystem layers;
- a Qt tag editor;
- 25 declared executable targets, including `invader-lightmap`, `invader-scan`, and `invader-crc` in addition to the 22 programs described in the README.

No repository CI workflow or conventional unit-test/CTest suite was found in the inspected snapshot. The long history, releases, and changelog provide maintenance evidence, but they are not substitutes for fixture-based verification in our environment. Before operational adoption, we need a pinned build, host feasibility record, malformed-input tests around our wrapper, and project-owned golden fixtures.

### Supported engine targets

The documented build and script interfaces name:

- `gbx-custom`;
- `gbx-demo`;
- `gbx-retail`;
- `mcc-cea`;
- `native`;
- `xbox-demo`;
- `xbox-ntsc`;
- `xbox-ntsc-jp`;
- `xbox-ntsc-tw`;
- `xbox-pal`.

This breadth is directly relevant to version-labeled differential testing. It does not prove perfect behavioral fidelity for every target; the changelog repeatedly records corrections made to more closely match `tool.exe`, and the bitmap documentation explicitly warns that some output may differ from HEK output.

## Tool surface and project relevance

| Capability | Invader surface | Relevance |
|---|---|---|
| Cache build | `invader-build` | High for Tutorial and Xbox/PC/MCC differential fixtures |
| Cache inspection | `invader-info`, `invader-index`, `invader-crc` | High as a second implementation beside Ringhopper |
| Tag extraction | `invader-extract` | Useful for operator-directed local research; derived rights remain constrained |
| Tag dependency/archive | `invader-dependency`, `invader-archive` | Useful for provenance manifests and bounded fixture packaging |
| Tag comparison | `invader-compare` | High for source/cache and cross-engine semantic comparisons |
| Validation/repair | `invader-bludgeon`, `invader-scan`, `invader-strip`, `invader-recover` | Research and explicit repair workflows; mutation requires backup/preview policy |
| Conversion/refactoring | `invader-convert`, `invader-refactor` | Useful for controlled legacy migrations, not native document editing |
| Script compilation | `invader-script` with `riat` | High for understanding Tutorial/scenario script behavior |
| Asset compilation | bitmap, model/JMS, sound, font, string, collection tools | Useful HEK-compatible reference and possible local bridge |
| Resource maps | `invader-resource` | Relevant to CE/MCC cache studies, not native Bevy packaging |
| Tag editing | CLI and Qt editors | Workflow reference; not the planned native scenario editor foundation |
| Lightmaps | `invader-lightmap` target | Candidate investigation, but not yet validated or accepted for native rendering |

### Determinism caveats

Invader exposes controls useful for reproducibility, including explicit engine selection, tag-directory precedence, output paths, build strings, compression levels, tag-space settings, resource-map policy, script-source location, and optional tag-order index files. Those controls make deterministic experiments plausible.

However, reproducibility is not established merely by these options. The project must test:

- two clean builds from identical inputs and pinned executable revisions;
- output hashes and normalized semantic comparisons;
- effects of threads, compression, filesystem ordering, paths, locale, and timestamps;
- generated tag ordering with and without `--with-index`;
- diagnostics and exit status stability.

Invader also documents deliberate engine- and path-sensitive tag patches during cache builds. These modifications may be historically correct compatibility behavior, but they mean that “compile” is not always identity-preserving. Every experiment must record target engine, scenario path, options, tool revision, and emitted warnings.

## Invader versus Ringhopper

The projects overlap because both descend conceptually from Halo tag/map tooling, but their best roles here differ.

| Dimension | Ringhopper / `omegon-ringhopper` | Invader |
|---|---|---|
| Primary language | Rust | C++/C with generated definitions and Rust `riat` submodule |
| License | GPL-3.0-only | GPL-3.0-only |
| Current project integration | Typed, bounded read-only Omegon extension | None; machine-local research checkout only |
| Implemented local tools | Map metadata, tag listing, dependencies, verification | Human-oriented CLI suite and Qt editor |
| Best immediate role | Safe structured inspection and verification | Legacy authoring/build/extract/compare/repair reference |
| Protocol fit | Extension-owned structured DTOs | Primarily human/plain-text CLI output; no general JSON contract observed |
| Mutation policy | Current extension is read-only | Many tools write or mutate; requires explicit staging, backup, preview, and output-root policy |
| Build weight | Narrow Rust extension | Broad native dependency stack |
| Native Bevy role | Optional external adapter | Optional external legacy compiler/reference |

### Role assignment

**Keep Ringhopper as:**

- the default structured map/tag inspector exposed to Omegon;
- the bounded verification and dependency-query service;
- the first side of differential parsing tests.

**Use Invader as:**

- the reference cache builder for selected HEK/Xbox/PC/MCC experiments;
- a second independent map/tag interpretation for differential evidence;
- an explicit local extractor, comparator, script compiler, and asset compiler when a fixture requires it;
- a source of hypotheses about target-specific processing and `tool.exe` behavior.

**Do not:**

- replace `halo-domain` with Invader's tag model;
- invoke Invader during normal Bevy startup;
- expose a generic unrestricted argv passthrough to agents;
- parse mutable human output as a permanent interoperability protocol without pinning and contract tests;
- allow both Ringhopper and Invader to silently define canonical semantics when they disagree.

When their results conflict, preserve both outputs with revisions and hashes, identify edition/build differences, and resolve the claim through source artifacts, runtime observation, or a narrower test. Agreement between two community tools raises confidence but is not independent proof if definitions or lineage overlap.

## Repository-specific incorporation

### Original-Xbox reconstruction repository

Invader belongs in the evidence/tooling layer, not in the C/XBE build graph. Appropriate uses are:

- compare Xbox cache metadata, tag ordering, extraction, and compilation assumptions with `kb.json` and observed executable behavior;
- generate candidate behavioral fixtures from operator-supplied content;
- inspect target-specific transformations while labeling claims as observed, inferred, or designed;
- preserve command, revision, input hash, output hash, engine target, and diagnostics with each result.

Invader's support for `xbox-ntsc` does not make its implementation evidence of the retail executable's code or exact algorithms.

### Independent Bevy repository

Invader is a Phase 3/5 external bridge. It may help normalize operator-supplied Tutorial tags, compile a comparison cache, or provide a legacy export target. It does not belong in Phase 0's dependency graph and does not change these invariants:

- canonical documents remain project-owned and versioned;
- `halo-domain` remains Bevy- and legacy-tool-independent;
- native derived artifacts are built by project-owned deterministic processors;
- the editor/runtime operate without Invader installed;
- public CI uses redistributable project-authored fixtures.

If automation is added, prefer a project-owned typed process wrapper with:

- an exact allowed operation rather than arbitrary argv;
- canonicalized input and output roots;
- subprocess timeout and owned-process termination;
- piped stdout/stderr;
- pinned executable hash and version capture;
- clean staging directories and atomic promotion;
- bounded output collection and structured diagnostics;
- no network requirement during execution.

## Risks and unresolved questions

| Risk/question | Current evidence | Required action |
|---|---|---|
| Exact build feasibility on this host | Source declares a broad dependency set; no local build performed | Add a non-mutating dependency/configure probe before the Tutorial experiment |
| Test posture | No conventional repository CI/test suite found | Own fixture and wrapper tests; do not infer correctness from release history |
| Machine-readable protocol | CLI tools primarily print human/plain-text output | Use narrow stable modes or write a GPL-compatible sidecar; avoid generic scraping |
| Parser lineage overlap | Ringhopper exposes “Invader verbs” and ecosystem concepts | Treat agreement as corroboration, not automatically independent implementation |
| Mutation safety | Extraction/edit/refactor/recover tools write files | Operate only in staged copies with explicit collision and backup policy |
| Silent build patches | README documents target/path-specific tag modifications | Capture options/warnings and compare semantic inputs/outputs |
| Submodule/dependency licensing | `riat` and several bundled/system dependencies are involved | Inventory exact revisions and licenses before redistribution |
| Lightmap suitability | Executable target exists but was not evaluated deeply | Defer until stock Tutorial geometry/material import is established |
| Native cache target meaning | Engine list includes `native` | Inspect and test before assuming it maps to any project-native artifact |

## Recommended adoption sequence

1. **Pin, inventory, and build-probe.** Record this commit, initialize `riat`, enumerate dependency versions/licenses, and test CMake configuration without installing globally.
2. **Tutorial build experiment.** In a quarantined resource workspace, run `invader-build -g xbox-ntsc` against the operator-supplied Tutorial scenario using explicit directories and output paths.
3. **Dual inspection.** Capture `invader-info` and `invader-index`; inspect the produced cache through `omegon-ringhopper`; retain structured normalized summaries with revisions and hashes.
4. **Reproducibility check.** Repeat from clean staging directories and compare byte hashes plus semantic inventories.
5. **Reference comparison.** Compare against any operator-supplied/local HEK `tutorial.map`, distinguishing expected build-tool differences from errors.
6. **Bounded adapter decision.** Only after the experiment, decide whether the Bevy project needs a typed Invader process sidecar for one import/export operation. Do not pre-build a universal wrapper.
7. **Defer advanced generators.** Evaluate bitmap/model/sound/script/lightmap paths one at a time when a native vertical slice needs a legacy comparison.

## Decision

**Adopt Invader as a pinned, optional, external legacy build and differential-reference tool.**

**Do not adopt it as a linked dependency, canonical domain model, native asset compiler, runtime requirement, or unrestricted agent shell surface.** Ringhopper retains the default structured read-only inspection role; Invader fills the broader authoring/build/transform role under explicit operator-invoked workflows.
