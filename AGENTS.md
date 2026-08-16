# Halo project directives

## Mission and repository tracks

This repository studies and reimplements Halo: Combat Evolved through two parallel tracks:

1. **C/XBE reconstruction:** verified functions are reimplemented in C and patched into an operator-supplied original Xbox executable.
2. **Modern runtime/toolchain:** a Rust and Bevy runtime, native content model, deterministic compiler, and artist-facing tools reproduce observed behavior on modern systems.

The modern track does not supersede the reconstruction track. Reconstruction evidence informs compatibility fixtures; modern implementation choices do not rewrite historical claims.

Read before modern-runtime work:

- `docs/architecture/modern-runtime-toolchain.md`
- `docs/modern-runtime-roadmap.md`
- `docs/research/bevy-ecosystem-2026-08.md`

## Legal and provenance boundaries

- Never commit original executables, HEK binaries, maps, tags, textures, sounds, or other proprietary Halo content.
- Operator-supplied files and unclear-rights derivatives stay in ignored local resource areas.
- Record source URL/identity, acquisition time, hashes, license evidence, and tool revision for imported resources.
- Public tests and samples use project-authored or clearly redistributable fixtures.
- Do not present interoperability as permission to redistribute content.
- Ringhopper is GPL-3.0-only. Direct linking belongs in separately distributed GPL-compatible tooling, not silently in a permissively licensed modern core.
- `omegon-ringhopper/` is an independent ignored Git checkout, not a submodule and not part of this repository's commits.

## Existing reconstruction track

The root CMake build targets 32-bit Xbox/Windows ABI code and patches the result into `halo-patched/cachebeta.xbe`. Generated declarations and thunks derive from `kb.json`.

Before changing reconstruction code:

- run `python3 tools/check_host.py`;
- preserve `kb.json` as the evidence-backed symbol/type source;
- add C sources to `src/CMakeLists.txt`;
- do not weaken ABI/type checks or freestanding compiler flags to make code compile;
- distinguish confirmed addresses/signatures from inference;
- do not require game files for ordinary unit tests or host feasibility checks.

## Modern runtime architecture invariants

- Canonical scenario/asset documents are versioned project data, not serialized Bevy worlds.
- Persist stable typed IDs, never ECS entity IDs.
- `halo-domain` remains independent of Bevy, UI toolkits, physics engines, Blender add-ons, networking libraries, and Ringhopper.
- GUI editor, CLI, CI, tests, and Omegon integrations call the same domain commands, validation, and compiler libraries.
- UI systems do not mutate canonical documents directly. Edits pass through command transactions with undo/redo semantics.
- Editor-only state and play-mode state remain separate from authored content.
- Source assets and derived runtime artifacts use separate roots and ownership.
- Derived artifacts are deterministic where practical and carry source/dependency hashes plus processor/configuration versions.
- Runtime gameplay uses an explicit fixed simulation schedule. Rendering and asynchronous asset work cannot define authoritative cadence.
- Physics, navigation, audio, VFX, and networking plugins provide mechanisms; project-owned systems define Halo compatibility behavior.
- Avoid speculative abstraction and empty crate scaffolding. Add the smallest vertical slice that produces evidence.

## First environment target

Phase 3 uses the tutorial level shipped with Gearbox's Halo Editing Kit as the first environment/scenario compatibility target.

Treat it as:

- an operator-supplied local reference, with the public HEK path `tags\levels\test\tutorial\tutorial.scenario` and any locally built or supplied `tutorial.map` verified against the acquired package;
- a source for an inspected affordance/dependency matrix;
- an import/compatibility target through explicit adapters;
- a model for a separately authored redistributable analogue used by CI.

Do not assume exact package contents or semantics beyond verified package inspection. Do not copy proprietary content into fixtures.

## Dependency decision policy

Research results identify candidates, not approved dependencies. Before promotion, record:

- exact version or revision;
- license and distribution impact;
- Bevy/Rust/platform compatibility;
- maintenance evidence;
- project-specific fixture results;
- failure and migration strategy;
- reevaluation trigger.

Current prototype baseline is Bevy 0.19. Volatile engine/plugin APIs belong behind narrow integration ownership, not universal abstraction layers.

## Testing requirements

Every non-trivial change needs tests at the narrowest appropriate layer:

- domain documents, migrations, references, commands, and validation;
- deterministic compiler outputs and dependency invalidation;
- fixed-step simulation traces without rendering;
- backend conformance fixtures for physics/import adapters;
- editor command/view-model tests, with a few interaction smoke tests;
- compatibility fixtures that state provenance, confidence, and tolerances.

Label compatibility behavior as observed, inferred, or designed. A passing test without evidence is not proof of original behavior.

## Security and robustness

Treat imported assets, maps, archives, and documents as untrusted input:

- canonicalize and constrain paths to explicit roots;
- reject traversal, escaping symlinks, malformed archives, and unreasonable sizes/counts/depth;
- spawn child processes with argument arrays and piped/ignored stdio, never shell interpolation;
- set timeouts and terminate owned process trees on cancellation;
- write atomically and preserve source originals;
- keep protocol stdout free of logs;
- sanitize user-facing errors while preserving actionable diagnostics.

## Workflow

- Inspect files before editing.
- Use focused branches for non-trivial work.
- Keep Workbench/lifecycle state truthful while executing plans.
- Prefer the smallest reversible step that answers the current spike question.
- Stop researching when a bounded implementation experiment can produce stronger evidence.
- Validate changed files, run relevant tests, check `git diff --check`, and commit completed work with a conventional commit message.
- Do not push unless explicitly requested.
