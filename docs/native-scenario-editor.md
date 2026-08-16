---
id: native-scenario-editor
title: "Phase 1 — Native scenario document and editor shell spike"
status: exploring
tags: [modern-runtime, phase-1, editor, scenario, spike]
open_questions:
  - "Which minimum native scenario object/property set is sufficient to complete the documented non-programmer workflow without prematurely modeling the full HEK scenario schema?"
  - "What atomic-save, external-change, and recovery protocol preserves user work while keeping GUI and CLI operations consistent?"
  - "How should play-in-editor snapshot and teardown isolate simulation mutations from the canonical document and undo history?"
dependencies:
  - modern-runtime-foundation
related:
  - deterministic-asset-pipeline
priority: 2
---

# Phase 1 — Native scenario document and editor shell spike

## Overview

Prove that a non-programmer can create, edit, validate, safely save/reload, and play a small native scenario through shared domain commands used by the GUI and CLI. Establish stable IDs, undo/redo, diagnostics, recovery, and play-mode isolation.

For the next vertical slice, implement only Gate A of [`beavercreek-compatibility-plan.md`](beavercreek-compatibility-plan.md). The minimum model is an environment reference, scenery placement, player start, project coordinate convention, validation, command transactions, deterministic save/reload, and an isolated play snapshot. Full HEK scenario coverage and a polished editor are not prerequisites for the Tutorial calibration or Beavercreek walkthrough.

## Assumptions to verify

- [assumption] Snapshot-backed undo remains adequate for the first realistic imported/project-authored scenario size.
- [assumption] Strict canonical JSON can remain the Phase 1 source format while migrations and diagnostics are added.
- [assumption] A single in-process play snapshot is sufficient to prove isolation before separate editor/runtime applications exist.

## Open Questions

- Which minimum native scenario object/property set is sufficient to complete the documented non-programmer workflow without prematurely modeling the full HEK scenario schema?
- What atomic-save, external-change, and recovery protocol preserves user work while keeping GUI and CLI operations consistent?
- How should play-in-editor snapshot and teardown isolate simulation mutations from the canonical document and undo history?
