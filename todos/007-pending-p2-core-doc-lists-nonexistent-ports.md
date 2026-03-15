---
status: pending
priority: p2
issue_id: "007"
tags: code-review, documentation, architecture
dependencies: []
---

# P2: Core module doc describes non-existent port files

## Problem Statement

The Core module doc and crate-structure snippet list VisionInterface, AudioInterface, InferenceInterface and files `vision.rs`, `audio.rs`, `inference.rs` under `core/ports`, but only `knowledge.rs` exists in the repo. This overstates what is implemented and can confuse contributors.

## Findings

- **Location:** `docs/src/content/docs/architecture/modules/core.mdx` (crate structure and interfaces table).
- **Evidence:** Doc shows `ports/device_control.rs`, `inference.rs`, `sensor_relay.rs`, `session_config.rs`, `audio.rs`, `vision.rs`, `knowledge.rs`, `peripherals.rs`; repo has only `ports/mod.rs` and `ports/knowledge.rs`.
- **Source:** Architecture-strategist.

## Proposed Solutions

1. **Narrow doc to current state:** Update the crate-structure snippet and interfaces table to list only the ports that exist today (e.g. KnowledgeInterface and knowledge.rs). Add a note: "Other domain interfaces (Vision, Audio, Inference, etc.) will be added when those domains expose core-facing traits." **Pros:** Truthful, no code change. **Cons:** Doc no longer describes final target layout. **Effort:** Small. **Risk:** Low.
2. **Add stub port modules:** Add minimal `vision.rs`, `audio.rs`, `inference.rs`, `peripherals.rs` (and others) with empty or placeholder trait definitions so the doc matches the tree. **Pros:** Doc and tree aligned. **Cons:** Stub files to maintain. **Effort:** Small. **Risk:** Low.
3. **Leave as-is:** Keep doc as target architecture. **Pros:** None. **Cons:** Drift. **Effort:** None. **Risk:** Medium (confusion).

**Recommended Action:** (To be filled during triage.)

## Technical Details

- **Affected files:** `docs/src/content/docs/architecture/modules/core.mdx`; optionally `engine/crates/core/src/ports/*.rs`.
- **Related:** ADR-004 and ADR-008 describe the target; implementation is scaffold-only for several domains.

## Acceptance Criteria

- [ ] Core module doc and crate-structure snippet either match the current repo or are explicitly labeled as target/future state.
- [ ] No misleading implication that vision/audio/inference core ports are implemented.

## Work Log

| Date | Action / Learning |
|------|-------------------|
| (Review) | Finding created from architecture-strategist report. |

## Resources

- Architecture-strategist: "Core module doc describes non-existent ports".
- engine/crates/core/src/ports/ (current contents).
