---
status: pending
priority: p2
issue_id: "004"
tags: code-review, architecture, knowledge
dependencies: ["003"]
---

# P2: retrieve_context has no implementation path (VectorSearch/FullTextSearch not in KnowledgeManager)

## Problem Statement

`KnowledgeInterface::retrieve_context` cannot be implemented by the current facade because `KnowledgeManager<S, M, A>` only has `DocumentStore`, `ConversationMemory`, and `AuditLog`; it does not have `VectorSearch` or `FullTextSearch`. The trait method is dead until the manager (or the bridge) has access to retrieval ports.

## Findings

- **Location:** `engine/crates/knowledge/src/domain/mod.rs`: `KnowledgeManager` has three type parameters (S, M, A) for store, memory, audit; no V, F for vector/FTS.
- **Evidence:** `retrieve_context(query, top_k) -> Vec<String>` requires embedding the query (via inference) and running hybrid search; neither is possible without VectorSearch/FullTextSearch (and inference dependency for embedding).
- **Source:** Agent-native-reviewer.

## Proposed Solutions

1. **Extend KnowledgeManager:** Add `V: VectorSearch` and `F: FullTextSearch` (or a single hybrid port) to `KnowledgeManager`, implement a `retrieve_context` method that takes an embedding and runs hybrid search (or FTS candidate + vector rerank). The `KnowledgeInterface` impl then calls this; embedding of the query can be done in the bridge by calling into inference (core orchestrates). **Pros:** Single place for retrieval logic. **Cons:** Manager grows; may need inference in knowledge or bridge. **Effort:** Medium–Large. **Risk:** Medium (dependency on inference for embeddings).
2. **Retrieval in bridge only:** Keep manager as-is; the bridge that implements `KnowledgeInterface` holds VectorSearch/FullTextSearch (and optionally inference for embedding) and implements `retrieve_context` there. **Pros:** Manager stays minimal. **Cons:** Bridge does more than delegation. **Effort:** Medium. **Risk:** Low.
3. **Defer retrieve_context:** Implement only ingest, memory, audit in the first wiring; document that `retrieve_context` will return empty or panic until retrieval is implemented. **Pros:** Unblocks wiring. **Cons:** Incomplete interface. **Effort:** Small. **Risk:** Low (if documented).

**Recommended Action:** (To be filled during triage.)

## Technical Details

- **Affected files:** `engine/crates/knowledge/src/domain/mod.rs`, bridge/adapter that implements `KnowledgeInterface`; possibly inference crate for embedding.
- **Doc:** knowledge module and ADR-010 already describe hybrid search as deferred.

## Acceptance Criteria

- [ ] Either KnowledgeManager or the KnowledgeInterface impl has access to VectorSearch (and optionally FullTextSearch).
- [ ] retrieve_context returns context strings (e.g. from hybrid search) or is explicitly stubbed and documented.
- [ ] No breaking change to KnowledgeInterface signature.

## Work Log

| Date | Action / Learning |
|------|-------------------|
| (Review) | Finding created from agent-native-reviewer report. |

## Resources

- Agent-native-reviewer: "Add retrieval to the knowledge facade".
- Knowledge module doc: hybrid search, FTS + vector rerank.
