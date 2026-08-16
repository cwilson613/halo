# Discord reference assessment — 2026-08-16

## Purpose and method

This report assesses the links and local artifact collected in `discord-misc.md`. It separates direct observation from claims in Discord or upstream documentation and maps each source to bounded project uses.

Evidence gathered on 2026-08-16 included:

- HTTP status checks for every URL;
- GitHub API metadata;
- blob-filtered or shallow local checkouts under `$HALO_RESOURCE_ROOT/tmp/discord-misc-repos`;
- repository tree, README, license, and selected implementation inspection;
- ZIP central-directory and selected text-file inspection without extracting or executing the archive.

Repository snapshots used:

| Source | Snapshot | Last commit observed | License evidence |
|---|---|---:|---|
| OpenSauce mirror | `d200c970c918921d40e9fb376ec685c77797c8b7` | 2022-08-12 | GPL-3.0 text in `license/License.txt`, but repository is an aggregate with component notices; do not infer uniform ownership/terms |
| HaloCEA corpus | `0fd88176fe19df400576d2cbb447d3363080cb52` | 2026-08-13 | no repository license detected |
| c20 | `6ebf90fc134255385e9d10de3d9d74a1f257ae48` | 2026-07-25 | GPL-3.0 (`COPYING`, GitHub metadata) |
| WinterSquire/HaloX | `d24b401c9eb924bcc1d4b50d6a332e8b14116b14` | 2025-11-09 | no license found |
| WinterSquire/AlphaRing | `5b333d9424a77aac28418055d20ef74b76545f66` | 2025-03-22 | no license found |
| WinterSquire/Halo-Tools | `0d80b0b9b6e273f6af06688981d9c3270773a239` | 2024-02-26 | no license found |
| WinterSquire/BlamScript | `b3a6018eebbd7f86b8615a41d8c5fd8b87eef74c` | 2024-08-04 | no license found |

A reachable URL or source file is evidence that a resource exists, not proof that every claim in it is correct.

## Executive assessment

The collection is useful, but its entries have very different evidentiary value:

1. **c20 is the strongest living reference for Halo 1 formats, editions, editing workflows, and community-tool orientation.** Use it as indexed documentation and a source of hypotheses; independently verify compatibility-critical constants and behavior.
2. **OpenSauce is valuable historical evidence for extended Halo CE rendering and tags.** Its directional-lightmap and shader-extension work can inform normalized material/lightmap schemas and visual tests, but it is neither stock Halo behavior nor a modern renderer architecture.
3. **HaloCEA and the local ZIP are broad comparative decompilation corpora.** They can recover names, subsystem boundaries, and candidate behavior, but target an HCEA Xbox 360 prototype descended from the PC branch—not the retail original-Xbox executable. AI-assisted reconstruction and absent licensing require per-claim verification and clean-room discipline.
4. **The examined WinterSquire repositories are specialized MCC/Windows experiments, not a ready Halo 1 runtime dependency.** The Discord wording about “HaloX ... to get halo1.dll (and friends) going without the launcher” is directionally plausible for MCC module hosting, but the inspected HaloX snapshot currently contains a Halo 3 module, depends on `libmcc`, and uses D3D11/Win32. It does not substantiate a complete Halo 1 standalone launcher today.
5. **The private Halospawns development site cannot be assessed without authorized credentials.** A credential was exposed in the raw note and has been removed. It must be rotated.
6. **The ZIP name is misleading.** It is not TinyCC source. It is a 4,608-file reconstructed C corpus with address annotations and a small host/map layer.

None of these sources should silently become canonical truth or copied implementation. They are research inputs whose claims must be promoted through version labeling, provenance, tests, and license review.

## 1. OpenSauce wiki and repository

References:

- https://github.com/MirisWisdom/OpenSauce/wiki/Doc_Halo1_DirectionalLightmaps
- https://github.com/MirisWisdom/OpenSauce/wiki/Doc_Halo1_ShaderExtension
- https://github.com/MirisWisdom/OpenSauce

### What is directly supported

The mirror describes itself as an aggregation of original OpenSauce history, HaloMods branches, and forks. Relevant implementation evidence includes:

- three scenario references named `directional_lightmap_1..3`;
- shader extension usage flags for diffuse and specular directional lightmaps;
- preprocessing validation requiring all three directional maps when any is supplied;
- a lightmap manager that falls back to standard lightmaps;
- environment/model extension code for normal maps, specular maps, tinting, and lighting;
- UI/configuration toggles and D3D9-era shader paths.

An embedded field explanation states that directional lightmaps store accumulated light in three directions in three lightmaps to produce normal-mapped static lighting. OpenSauce's own readme labels diffuse directional lightmaps as an addition and specular directional lightmaps as experimental.

### Interpretation and limits

- These are **OpenSauce extensions**, not evidence of stock Xbox or Custom Edition rendering behavior.
- The three-map convention is a useful interoperability schema candidate, not necessarily the representation the Bevy runtime should use internally.
- The code targets old Direct3D and modifies runtime/tag structures. Porting it mechanically would preserve obsolete constraints.
- The mirror combines components and histories; a GPL file at repository root does not resolve copyright or license provenance for every subtree and bundled resource.

### Recommended project use

- Record OpenSauce extension fields in a versioned optional import namespace.
- Preserve all three directional maps and extension flags losslessly during import even before rendering support exists.
- Build a small redistributable directional-lightmap fixture and compare standard fallback, diffuse normal response, and optional specular response.
- Independently express semantics in project code; do not copy old renderer code into the modern runtime.
- Defer support until after stock Tutorial materials/lightmaps work. It is a compatibility extension, not Phase 0–3 critical path.

Confidence: **high** for repository structure and the three-map/flag behavior; **medium** for broader visual semantics until controlled rendering comparisons exist.

## 2. Private Halospawns development site

Reference:

- https://dev.halospawns.com/

Authenticated inspection of the shared development frontend succeeded. It is a Svelte application backed by `api.dev.halospawns.com` and a separate authentication layer. The frontend models:

- map catalog entries, releases, variants, families, and similarity suggestions;
- GLB/JSON map assets, thumbnails, and screenshot definitions;
- replay-to-map matching, games, participants, players, and statistics;
- positional heatmaps;
- named map regions, region sets, collections, and transforms between related maps;
- region statistics and administrative map reprocessing.

This makes it relevant as design evidence for map identity, coordinate transforms, region authoring, replay analytics, and reproducible processing jobs. It is not evidence for core gameplay or renderer behavior and should not be a runtime dependency.

The website's shared HTTP Basic credential did not authorize its API; API requests returned HTTP 401 because the application uses a separate account/session system. No API payloads or map assets were downloaded.

Confidence: **high** for frontend-exposed concepts; **unknown** for data quality, backend implementation, and available content because API access was not provided.

## 3. WinterSquire repositories and HaloX claim

Reference:

- https://github.com/WinterSquire?tab=repositories

The account exposed twelve public repositories at inspection time. The likely relevant projects were examined separately.

### HaloX

The inspected snapshot is a C++20 Win32 application with D3D11, ImGui, XInput, and `libmcc`. It contains:

- MCC-style game/module management and dynamic `LoadLibraryW` calls;
- interfaces for map/game variants, storage, players, input, and rendering;
- a currently visible `module/halo3` implementation;
- no visible Halo 1 module in the checked snapshot;
- almost no explanatory README;
- no detected license.

Assessment of Discord claim: **partially supported, overstated if read as present functionality**. HaloX demonstrates architecture for hosting MCC game modules outside the stock shell and therefore may eventually illuminate loading `halo1.dll`. The current tree does not establish that Halo 1 works “without the launcher.”

Potential use:

- study MCC host/module interface boundaries if an MCC interoperability track is approved;
- compare launcher responsibilities and module lifecycle;
- do not make it a dependency of the original-Xbox reconstruction or Bevy runtime.

### AlphaRing, Halo-Tools, and BlamScript

- `AlphaRing` is another sparse CMake-based MCC modding experiment with no detected license or useful top-level documentation in the inspected snapshot.
- `Halo-Tools` is a Windows injector plus an internal Halo 3 dolly-camera modification and bundles a DLL. It is unrelated to the immediate Halo 1 runtime/editor path.
- `BlamScript` contains small ManagedBlam C# scripts for Halo 3 scenario/scenery work. It may provide UI/task ideas but no direct Halo 1 implementation value.

Recommended posture: keep these as **MCC interoperability leads**, not dependencies. Revisit HaloX only when work explicitly targets MCC's `halo1.dll` process contract.

Confidence: **high** for inspected repository contents; **medium** for future HaloX intent.

## 4. Claimed C++/Vulkan read-only map renderer

The Discord paragraph describes an unnamed personal project: a C++/Vulkan map opener and renderer compared with read-only SparkEdit/Prometheus. No URL or artifact in the note identifies it.

Therefore its existence and capabilities are **unverified** from the supplied references. Do not attribute it to HaloX: HaloX is D3D11/Win32 and concerns MCC modules, which conflicts with the paragraph's Vulkan/map-viewer description.

Questions needed to assess it:

- repository or immutable archive and revision;
- supported map engine/builds;
- parser provenance and tag coverage;
- BSP, material, lightmap, model, collision, and scenario support;
- license and asset handling;
- screenshots/tests and malformed-input behavior.

Potential value, if obtained: a useful differential renderer/parser for `tutorial.map`, and UI inspiration for read-only map inspection. It should not displace Ringhopper without measured coverage and licensing evidence.

Confidence: **unknown**.

## 5. HaloCEA decompilation corpus

Reference:

- https://github.com/surreptitiousresearch/halocea

The repository identifies its source as an HCEA Xbox 360 June 24, 2011 prototype with verbose symbols and minimal optimization. It reports use of PDB information, IDA-oriented tooling, remill/LLVM/instrumentation, and several Anthropic models with human guidance. At the inspected revision it contained approximately 9,750 `.c` files and 3,071 files under `src/headers`, occupying about 96 MiB in a blob-filtered checkout.

### Value

- recovered function and type names;
- broad subsystem inventory, especially AI;
- candidate call ordering and control flow;
- comparison among PC-derived/HCEA and original-Xbox implementations;
- hypotheses for unknown names in stripped retail binaries.

### Risks and limits

- Platform lineage differs: HCEA Xbox 360 is derived from the Gearbox PC branch and includes Saber integration and later changes.
- Verbose symbols improve naming but do not prove semantic identity with original Xbox.
- AI-assisted reconstructions can be coherent but wrong; comments are especially weak evidence unless tied to disassembly or tests.
- The README describes the corpus as a research artifact but no software license was detected. Public visibility is not permission to copy.
- Thousands of isolated files can create false confidence through apparent completeness.

### Clean-use protocol

For any candidate fact:

1. record corpus commit, exact file/symbol, address, and HCEA prototype identity;
2. label it a hypothesis, not an original-Xbox fact;
3. compare retail Xbox disassembly, current `kb.json`, c20/Ringhopper definitions, and runtime observations;
4. promote only the independently verified fact or behavior;
5. write fresh implementation and tests rather than transplanting unlicensed source.

Do not bulk-import names, structs, or source into this repository.

Confidence: **high** that the corpus is a rich comparative artifact; **low by default** for any unverified original-Xbox claim derived from it.

### Operator-supplied Defiance and Jojo discussion

On 2026-08-16, the operator supplied a Discord transcript containing messages dated from 2026-07-09 through 2026-08-15. This repository does not currently have a Vox Discord connection or an independently queryable export, so the transcript is preserved as **operator-supplied testimonial evidence**, not independently authenticated Discord history. Exact message URLs, channel/server identity, Discord user IDs, attachment hashes, and an immutable export were not supplied.

The transcript supports the following attribution when read with the operator's identification of the opening speaker as Jojo:

- **Defiance** owns or leads the HCEA Blam re-sourcing corpus, performs disassembly-to-C attestation, refines symbols/types/constants, and publishes `surreptitiousresearch/halocea`.
- **Jojo** reports running a compiled Blam core derived from that work against **ReXGlue's “accurate core”**, comparing memory after each deterministic Halo tick, using Codex to localize divergences, and returning proposed bug fixes to Defiance.
- **ReXGlue** supplies the comparison core or execution mechanism. The transcript does not establish its implementation, provenance, or validation level beyond Jojo's description.

The most relevant supplied statements are paraphrased rather than reproduced wholesale:

- Jojo reports per-tick memory diffing between the compiled Blam core and ReXGlue's accurate core; because Halo is deterministic, divergences appear quickly.
- Jojo says Codex is finding many bugs and that fixes will be provided to Defiance.
- Jojo describes aiming for a near-perfect disassembly in the re-sourcing project and plans another particle-system attestation because of complex floating-point behavior.
- Defiance separately reports disassembly/code attestation finding subtle `int`-versus-`float` defects that would affect AI looking behavior.
- Defiance reports an adjudication ledger of 50 real defects, 6 portability hardenings, 12 faithful-as-built or refuted findings, and 4 rejected proposals. This is a self-reported snapshot, not yet tied here to immutable commits or machine-readable reports.

A cautious reconstruction of the validation loop is:

```text
shared initial state and inputs
    ├── compiled reconstructed Blam core
    └── ReXGlue accurate/reference core
              ↓
       execute one Halo tick
              ↓
      compare selected memory/state
              ↓
       localize first divergence
              ↓
       investigate and adjudicate
              ↓
        return accepted fixes
```

The transcript directly supports deterministic per-tick comparison and a feedback loop; it does **not** establish whether all process memory or normalized regions are compared, how pointers/padding/allocators and nondeterministic state are handled, whether ReXGlue executes original machine code or another representation, or how candidate fixes are reviewed before acceptance. Those details require runner source, protocol documentation, or targeted Discord retrieval.

This adds a useful two-layer validation model:

1. **Static attestation:** compare reconstructed C, types, casts, constants, and floating-point expressions with disassembly.
2. **Differential execution:** compare runtime state at deterministic tick boundaries and investigate the earliest meaningful divergence.

For this project, HaloCEA remains comparative evidence rather than original-Xbox authority. Candidate findings should record engine/build lineage, exact source symbol and address, comparison-core identity, compared state regions, first divergent tick, normalization rules, tool/model involvement, human adjudication, and the regression fixture that proves the accepted correction. The modern Bevy architecture is unchanged, but its fixed-step traces and explicit state projections are consistent with this validation strategy.

A future **Vox Discord integration** should be used for targeted searches and provenance completion: resolve stable message links, authors and user IDs, surrounding thread context, attachments, edits, dates/time zones, and later corrections. Until then, do not infer additional speaker identity or runner mechanics beyond this supplied transcript.

Confidence: **high** for what the supplied text literally reports; **medium** for speaker attribution because the opening header is absent from the pasted excerpt and supplied separately by the operator; **unknown** for ReXGlue internals and the correctness/completeness of reported fixes.

## 6. `blam_tinyccompiler_src.zip`

Local input:

```text
/home/wilson/Downloads/blam_tinyccompiler_src.zip
SHA-256 25a1bf1380a0026212c537729ad5db290e4292a45a179bcd39fffa51185407bc
size 4,398,006 bytes
```

A read-only quarantined copy and provenance note now live at:

```text
$HALO_RESOURCE_ROOT/source/community/blam-tinyccompiler-src/
  25a1bf1380a0026212c537729ad5db290e4292a45a179bcd39fffa51185407bc/
```

### Safe structural inspection

- 4,608 ZIP entries, all under `src/`;
- 10,895,481 uncompressed bytes;
- 4,569 `.c`, 3 `.h`, and 36 extensionless directory entries;
- no path traversal, absolute path, or null-byte names;
- no README, license, manifest, CMake file, Makefile, Git metadata, or TinyCC/TCC sources.

The sampled corpus has function-address comments such as `game_initialize_for_new_map @0x83687B78`, many per-function `extern` declarations, reconstructed structures, and explanatory comments. In the first 1,000 C files, 861 contained large hexadecimal address annotations and 519 contained `extern` declarations. Examples cover map initialization order, HaloScript parsing, scenario player starts, rendering, networking, AI, and audio.

Three top-level headers form a small host layer:

- `blam_level.h`: map/cache loading, BSP meshes, models, scenario objects, cameras, cutscene flags, scripts, and player position;
- `host_audio.h`: host audio adaptation;
- `host_clock.h`: host timing adaptation.

### Relationship to HaloCEA

Names, Xbox 360-style addresses, layout, and content strongly suggest the archive is related to the HaloCEA reconstruction effort or a derivative experiment. This is an inference, not established provenance. The archive includes a native initialization grouping and host-facing code not sufficient to establish how it was generated or built.

The Discord filename likely means “Blam source intended for a TinyCC experiment,” not “TinyCC source.” There is no compiler implementation in the archive.

### Handling decision

- Keep quarantined and non-executable.
- Do not compile or redistribute it.
- Do not import any code, addresses, names, or headers until the sender provides author/upstream, source binary/build, generation method, and license.
- If provenance is established, treat it under the same comparative-evidence protocol as HaloCEA.

Confidence: **high** for archive composition; **medium** for relationship to HaloCEA; **unknown** for authorship/license/correctness.

## 7. c20 Reclaimers Library

References:

- https://c20.reclaimers.net/
- https://github.com/Sigmmma/c20

c20 is an active, GPL-3.0 living knowledge base. Its source explicitly distinguishes original Xbox, Gearbox PC, Custom Edition, HCEA/MCC, and related formats. It documents:

- tags and map formats;
- HEK/H1A editing kits and their differences;
- Tool, Guerilla, Sapien, Standalone, and community tools;
- scenario/BSP/material/lightmap/script semantics;
- source formats and Blender workflows;
- known engine limits and edition-specific behavior.

It directly supports important planning assumptions: HCEA derives from the Gearbox PC branch; MCC/H1A adds fixes, fields, functions, and limits; legacy and MCC tags/maps are not automatically bidirectionally compatible.

### Recommended use

- Primary terminology and workflow index.
- Edition/version cross-check before accepting any tag or runtime claim.
- Links and citations in internal research documents.
- Input to fixtures and questions, not a substitute for empirical tests.

Because c20 content is GPL-3.0, avoid copying prose or generated definitions wholesale into a differently licensed deliverable. Independently implement facts and interfaces, preserve citations, and review any reused code/data separately.

Confidence: **high** as maintained community documentation; individual claims still vary and should be triangulated when compatibility-critical.

## Cross-source contradictions and reconciliation

### “Halo 1” is not one target

The sources span at least:

- original Xbox retail;
- Gearbox Halo PC/Custom Edition;
- OpenSauce-extended Custom Edition;
- HCEA Xbox 360 prototype;
- MCC/H1A modules and editing tools;
- Halo 3 MCC tooling in some WinterSquire repositories.

Every imported fact needs an explicit engine/platform/build dimension. Similar names do not imply identical layout or behavior.

### Source completeness is not source authority

HaloCEA and the ZIP look comprehensive because they contain thousands of named files. Their authority remains lower than direct evidence from the target binary and measured behavior. c20 is curated and version-aware, but remains secondary documentation. OpenSauce is primary evidence for OpenSauce behavior only.

### Modern rendering versus compatibility

OpenSauce directional/specular lightmaps and the claimed Vulkan viewer are useful demonstrations, but the Bevy runtime first needs stock Tutorial rendering. Extension fields should be preserved in normalized IR so later support does not require destructive reimport.

## Project decisions and action matrix

| Workstream | Adopt now | Evaluate later | Reject/defer |
|---|---|---|---|
| Original-Xbox reconstruction | use c20 and HaloCEA as labeled hypothesis sources; require per-symbol verification | build comparison tooling for selected functions/types | bulk source/name import from unlicensed corpora |
| Tutorial/map evidence | use Ringhopper plus c20 definitions and operator-supplied HEK artifacts | obtain unnamed Vulkan viewer for differential tests | assuming OpenSauce/HCEA semantics are stock Tutorial semantics |
| Native domain model | include engine/build provenance and lossless unsupported fields | optional namespace for OpenSauce extensions | coupling canonical documents to old C++ structs |
| Renderer | stock BSP/material/lightmap path first | directional lightmaps with project-authored fixture | mechanical D3D9/OpenSauce port |
| Editor | c20/HEK workflows as task evidence | Halospawns map identity, coordinate-transform, region-authoring, and analytics concepts | coupling authoring to a private service |
| MCC interoperability | none on immediate critical path | HaloX/libmcc process boundary spike | treating HaloX as proven Halo 1 standalone support |
| Licensing | cite/link; independently implement verified facts | legal review of specific desired components | copying no-license or ambiguous code |

## Recommended follow-up sequence

1. Ask the ZIP sender for upstream URL, exact HCEA binary/build, generator, authorship, license, and intended TinyCC workflow.
2. Ask for the unnamed Vulkan renderer repository and revision.
3. Obtain API or export documentation only if deeper Halospawns interoperability becomes an approved requirement; the frontend concepts are sufficient for current planning.
4. Add an engine/build field to all future compatibility evidence records.
5. During the Tutorial inventory, use c20 and Ringhopper as complementary references and keep OpenSauce extension support out of the first stock-rendering acceptance gate.
6. Select a handful of high-value functions already present in this repository and compare HaloCEA/ZIP hypotheses against original-Xbox disassembly; measure accuracy before using either corpus broadly.
7. Create a redistributable directional-lightmap fixture only after stock lightmap support works.

## Source links

- OpenSauce directional lightmaps: https://github.com/MirisWisdom/OpenSauce/wiki/Doc_Halo1_DirectionalLightmaps
- OpenSauce shader extensions: https://github.com/MirisWisdom/OpenSauce/wiki/Doc_Halo1_ShaderExtension
- OpenSauce mirror: https://github.com/MirisWisdom/OpenSauce
- Halospawns development origin: https://dev.halospawns.com/
- WinterSquire repositories: https://github.com/WinterSquire?tab=repositories
- HaloCEA: https://github.com/surreptitiousresearch/halocea
- c20 site: https://c20.reclaimers.net/
- c20 source: https://github.com/Sigmmma/c20
