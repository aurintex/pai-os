---
status: pending
priority: p2
issue_id: "003"
tags: code-review, architecture, knowledge, agent-native
dependencies: []
---

# P2: No KnowledgeInterface implementation or runtime wiring

## Problem Statement

Core defines `KnowledgeInterface`, but no type implements it and the composition root does not build or inject a knowledge adapter. The knowledge domain is scaffold-only at runtime: core cannot call ingest, retrieve, memory, or audit. Agent-native parity is blocked until the same operations are exposed and wired.

## Findings

- **Location:** `engine/crates/core/src/ports/knowledge.rs` (trait); `engine/crates/knowledge/` (no impl); `engine/pai-engine/src/main.rs` (no injection).
- **Evidence:** No `impl KnowledgeInterface for ...` in repo; main.rs has TODO for wiring.
- **Source:** Agent-native-reviewer, architecture-strategist.

## Proposed Solutions

1. **Bridge in knowledge crate:** Add a type in `knowledge` (e.g. `KnowledgeBridge` or `KnowledgeAdapter`) that holds `KnowledgeManager` (and future VectorSearch/FullTextSearch), implement `KnowledgeInterface` for it, and have knowledge depend on `core` only for the trait. Composition root constructs it when `knowledge` feature is enabled and injects into orchestrator. **Pros:** Single place for mapping. **Cons:** knowledge crate gains dependency on core (trait only). **Effort:** Medium. **Risk:** Low.
2. **Bridge in pai-engine:** Implement `KnowledgeInterface` in pai-engine in a module that wraps `KnowledgeManager`; knowledge crate stays free of core dependency. **Pros:** Core and knowledge stay independent. **Cons:** Mapping logic lives in composition root. **Effort:** Medium. **Risk:** Low.
3. **Stub impl for tests first:** Add `knowledge_mock` adapter that implements the knowledge ports and a thin wrapper that implements `KnowledgeInterface`; wire in main when feature is enabled. **Pros:** Unblocks tests and CI. **Cons:** Still need real impl later. **Effort:** Medium. **Risk:** Low.

**Recommended Action:** (To be filled during triage.)

## Technical Details

- **Affected files:** `engine/crates/knowledge/src/` (new bridge type or impl module), `engine/pai-engine/Cargo.toml` (optional core dep if bridge in knowledge), `engine/pai-engine/src/main.rs`.
- **Related:** retrieve_context requires VectorSearch/FullTextSearch in KnowledgeManager (separate finding); document delete/list not on KnowledgeInterface (agent-native).

## Acceptance Criteria

- [ ] Some type implements `KnowledgeInterface` and delegates to KnowledgeManager (and retrieval when available).
- [ ] Composition root instantiates and injects it when knowledge feature is enabled.
- [ ] Core/orchestrator can call ingest_document, append_memory, read_memory, write_audit_entry (and retrieve_context once retrieval is implemented).

## Work Log

| Date | Action / Learning |
|------|-------------------|
| (Review) | Finding created from agent-native and architecture-strategist reports. |

## Resources

- Agent-native-reviewer: "Implement and wire KnowledgeInterface".
- Architecture-strategist: "Knowledge is not wired into the runtime".
- ADR-010: Deferred follow-up.
