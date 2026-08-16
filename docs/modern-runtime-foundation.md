---
id: modern-runtime-foundation
title: "Phase 0 — Modern runtime foundation spike"
status: implemented
tags: [modern-runtime, phase-0, bevy, spike, foundation]
open_questions: []
dependencies: []
related:
  - native-scenario-editor
priority: 1
---

# Phase 0 — Modern runtime foundation spike

## Overview

Prove the minimum Bevy 0.19 runtime/editor architecture with a project-authored fixture, Bevy-independent canonical document model, command-driven transforms, deterministic serialization, fixed-step headless traces, and measured iteration costs. This is an evidence-producing spike, not a general editor implementation.

## Research

### Linux Wayland/Vulkan launch qualification

On 2026-08-16, the pinned Nix development profile supplied the required Wayland, xkbcommon, and Vulkan dependencies. The Bevy 0.19.1 application built and launched on the host's Wayland/Vulkan stack and rendered the project-authored PBR fixture. This resolves the Linux renderer/window feasibility question. Apple Silicon macOS remains a separate, explicitly unqualified platform target.

### Canonical scenario document probe

Phase 0 uses strict, pretty-printed JSON with a trailing newline as its canonical textual representation. The Bevy-independent `halo-domain` model owns schema versioning, stable typed object IDs, project-owned transforms, validation, and serialization. Golden-equivalence tests prove byte-stable repeated output and parse/serialize round trips; unknown fields, unsupported versions, duplicate IDs, and non-finite transforms are rejected. This resolves the Phase 0 format choice without claiming that schema version 1 is the final production scenario schema.

### Host and Bevy 0.19 feasibility probe

On 2026-08-16, the host exposed rustc 1.97.1 and cargo 1.97.0. `/dev/dri/renderD128` exists and is world-readable. A machine-local probe under `/home/wilson/.local/share/halo-re/tmp/bevy-019-probe` resolved Bevy 0.19.1 from crates.io with `default-features = false` and completed `cargo check` successfully in 27.59s after dependency download. The later Nix-backed launch qualification above supersedes the probe's original renderer uncertainty.

## Decisions

### Use Bevy 0.19.1 for the Phase 0 implementation probe

**Status:** accepted

**Rationale:** The approved research baseline is Bevy 0.19, crates.io currently resolves 0.19.1, and a minimal no-default-features crate compiles with the host Rust 1.97.1 toolchain. Pin the lockfile for the spike; this does not yet promote Bevy 0.19.1 beyond Phase 0 because renderer/editor evidence is still open.

### Keep Phase 0 to one domain crate and one consuming Bevy application

**Status:** accepted

**Rationale:** This is the smallest vertical slice that can verify a Bevy-independent canonical document, domain-command editing, fixed-step simulation, and runtime projection without creating speculative empty crates.

### Accept the Phase 0 editor interaction evidence

**Status:** accepted

**Rationale:** The Bevy application selects the project-authored fixture, presents object identity and transform state, visualizes transform axes, and routes movement through `DomainCommand::MoveObject`. Tests prove that the authored document and ECS projection remain synchronized without exposing Bevy or UI types through `halo-domain`. A production drag gizmo and specialized inspector remain Phase 1 UX work rather than Phase 0 blockers.

## Resolution

Phase 0 is implemented. The Linux Wayland/Vulkan renderer launch, deterministic canonical document tests, fixed-step headless trace, command-driven editor interaction, projection tests, and recorded build/startup evidence satisfy the foundation gate. Apple Silicon runtime qualification remains a platform task and does not reopen this Linux foundation decision.

The next authorized vertical slice is Gate A in [`beavercreek-compatibility-plan.md`](beavercreek-compatibility-plan.md): the minimum native scenario transaction, save/reload, validation, and isolated play-session substrate.
