# HCAA SDL host package assessment — 2026-08-16

## Scope

This note records a safe structural assessment of the operator-supplied archive `BLAM_EXTRACTED_HCAA-clean.zip`, explains why it resolves the earlier Jojo compilation blockers, and gives a bounded macOS trial path. The archive was inspected as ZIP metadata and selected text/source entries. Bundled executables were not run during this assessment.

## Acquisition identity

Machine-local original:

```text
$HALO_RESOURCE_ROOT/acquisition/downloads/BLAM_EXTRACTED_HCAA-clean.zip
size: 51,357,182 bytes
SHA-256: 296c38b1c478bf54a1a7aa5efd88691487900a6ccab78647d84541d0ca0b1f18
acquired: 2026-08-16 (operator-supplied download; exact public URL/message identity not recorded)
```

Archive safety inventory:

- 11,151 entries;
- 143,462,764 uncompressed bytes;
- compression ratio approximately 2.94:1;
- no encrypted entries;
- no ZIP symlinks;
- no observed absolute paths, `..` traversal components, or null-byte paths;
- largest entry 29,634,560 bytes;
- includes Windows EXEs and DLLs and must remain untrusted until qualified in isolation.

The archive remains outside Git. Do not commit or redistribute it or its recovered/proprietary payloads.

## Declared source identity

The included `SOURCE_REVISION` records:

```text
repository: https://github.com/surreptitiousresearch/halocea.git
commit: aa635a70b2c3285ade3f53c2f3f94de759ce7cd4
blam-version: 01.00.01.0563
```

The included README describes the package as a 32-bit Windows SDL 1.2 host for that recovered HCAA Blam build. This establishes the package's declared lineage but does not independently establish source correctness, permission to redistribute recovered code, or compatibility with original-Xbox, Custom Edition, or unrelated cache files.

No root license covering the recovered corpus was observed. Bundled dependencies have their own notices, which do not resolve the recovered-source license. Treat the package as an unclear-rights local research artifact.

## Composition

The archive contains approximately:

- 4,864 C files;
- 1,753 headers;
- host entry point and platform adapters;
- cache, level, HaloScript, input, observer, audio, and diagnostics boundaries;
- a curated recovered-source subset under `src/blam`;
- `build.ps1`, `verify.ps1`, launchers, and `blam_host.ini`;
- TinyCC 0.9.27 and SDL 1.2.15 development/runtime files;
- a current 32-bit Windows build;
- an optional separate Python/SDL2/OpenGL/ImGui debugger.

The package documentation reports that the host can initialize the recovered shell, open a 640×480 SDL software surface, validate and relocate 1,411 `ui.map` tags, run menu camera script state, load campaign maps, and mix supported native sounds. Those are package claims pending local execution evidence.

## Resolution of the earlier Jojo build blocker

The earlier `blam_tinyccompiler_src.zip` contained 4,569 C files, only three headers, and no build graph, host entry point, compiler configuration, or platform dependency closure. Adding public HaloCEA headers still left an undifferentiated function corpus rather than a runnable program.

The clean package supplies the missing **port composition**:

```text
matching selected source and headers
    + explicit translation-unit list/exclusions
    + host entry point
    + platform/cache/render/input/audio adapters
    + compiler and SDL boundary
    + runtime configuration
    + headless verification surface
```

`build.ps1` is especially important. It selects host units, recursively includes the curated recovered tree, excludes conflicting or incomplete native initialization units, and explicitly adds dependency-complete initialization/gameplay functions. That policy cannot be reconstructed safely by compiling every recovered C file.

Therefore:

- the clean package resolves the source/header/build-composition blockers from the earlier attempt;
- it should replace the old dump as the object of a build trial;
- it should **not** be overlaid onto `jojoc/blam_tinyccompiler_src.zip`;
- its declared source revision, selected headers, host patches, and build script must be treated as one versioned unit.

## Runtime data contract

The package does not include the prototype map set it expects. `blam_host.ini` provides:

```ini
[paths]
prototype_root=
```

The configured root must contain `maps\ui.map`; it must point at the prototype root, not directly at `maps`. Documented path precedence is:

1. explicit command-line option such as `--proto-root`;
2. `HCEA_PROTO_ROOT`;
3. `[paths] prototype_root` in the selected INI.

The separate downloaded `beavercreek.map` must not be assumed compatible. This host targets HCAA build `01.00.01.0563`; Beavercreek's cache engine/build must be identified independently before any load attempt.

## macOS trial plan

### Hard constraint

The supplied host is a **32-bit Windows x86 executable** built with Windows TinyCC and SDL 1.2. A native macOS launch is not supplied. Apple Silicon cannot run 32-bit Windows binaries directly, and ordinary modern Wine on macOS may not support this executable/runtime combination.

The most reliable first trial on an Apple Silicon Mac is an isolated x86 Windows virtual machine. CrossOver or Whisky/Game Porting Toolkit may work, but that is unverified and 32-bit support is the central risk. Do not begin by rewriting the build for macOS; first establish that the supplied Windows composition works.

### Recommended qualification order

1. Keep the ZIP and prototype data outside the source repository.
2. Create a disposable Windows 10/11 x86-capable VM or isolated compatibility prefix.
3. Disable network access for the first run unless a documented dependency requires it; the normal build is declared offline.
4. Extract the ZIP into a normal writable directory inside the isolated environment.
5. Scan the extracted tree with the host platform's malware scanner.
6. Do not use `bootstrap-toolchain.ps1`; the archive already claims to include the pinned toolchain, and bootstrap performs downloads.
7. Keep prototype maps in a separate read-only directory and configure `blam_host.ini`:

   ```ini
   [paths]
   prototype_root=D:\path\to\HCEA_EXTRACTED_PROTO
   ```

8. Run the documented headless verifier before opening a window:

   ```powershell
   powershell -NoProfile -ExecutionPolicy Bypass -File .\verify.ps1
   ```

9. Record the complete command, VM/compatibility product and version, CPU architecture, Windows version, exit code, and log. A claimed successful reference result is `ui.map: 1411/1411 tags` and exit code zero.
10. Only after headless success, run the menu launcher or an explicit known-compatible prototype campaign map.
11. Do not point the host at `beavercreek.map` until its cache identity and compatibility are established.

### Rebuild only if needed

The archive includes a current build. If rebuilding is necessary, use its own build script and source tree as supplied:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\build.ps1
```

Expected outputs are documented as:

```text
build\blam_host.exe
build\SDL.dll
build\blam_host.ini
```

Record hashes before and after rebuilding. A differing executable hash is not automatically a failure—tool/environment metadata must accompany it—but unexplained source or generated-file changes invalidate comparison with the supplied build.

### Stop conditions

Stop rather than weakening isolation if:

- the archive or prototype provenance is unclear enough to create a redistribution risk;
- the compatibility environment requires disabling broad host security controls;
- the executable attempts unexpected network access or writes outside its project/configured output roots;
- the headless verifier crashes before reporting the map identity/tag count;
- only map-specific patches make initialization pass;
- the supplied HCAA build is asked to consume an unidentified Beavercreek cache.

## Project use

Classify this package as an optional local **differential oracle and lifecycle probe**. It may provide bounded observations for initialization order, cache relocation, player starts, cameras, objects, collision BSP state, scripts, and deterministic tick traces.

It is not:

- a dependency of `halo-domain` or the Bevy runtime;
- a public fixture;
- authority for retail original-Xbox or Custom Edition behavior;
- permission to copy recovered source into this repository;
- a substitute for version-aware Ringhopper/Invader inspection or project-owned normalization.

Promote only independently verified behavior into project-authored records and fixtures, with source build, symbol/trace identity, first divergent tick where applicable, and explicit cross-version limits.

## Remaining evidence needed

- immutable sender/message or public release identity for the clean ZIP;
- license/redistribution statement for recovered source and bundled native assets;
- exact prototype package identity and hashes;
- a successful isolated headless verification record;
- Beavercreek cache engine/build identification;
- documentation of the debugger shared-memory schema before consuming it from project tooling.
