---
status: pending
priority: p2
issue_id: "005"
tags: code-review, knowledge, agent-native, api
dependencies: ["003"]
---

# P2: Document lifecycle not exposed on KnowledgeInterface (delete, get chunks, list)

## Problem Statement

Core's `KnowledgeInterface` only has `ingest_document`; there is no delete or read path. `DocumentStore` has `delete_document` and `get_document_chunks`, but they are not exposed to the orchestrator or future agents. CRUD is incomplete for documents.

## Findings

- **Location:** `engine/crates/core/src/ports/knowledge.rs` (only ingest_document, retrieve_context, append_memory, read_memory, write_audit_entry).
- **Evidence:** DocumentStore defines delete_document and get_document_chunks; KnowledgeInterface does not.
- **Source:** Agent-native-reviewer.

## Proposed Solutions

1. **Add to KnowledgeInterface:** Add `delete_document(&self, document_id: &str)` and either `get_document_chunks(&self, document_id: &str) -> Vec<...>` or a small "document info" type. Optionally `list_document_ids(&self) -> Vec<String>`. Implement in the bridge. **Pros:** Full CRUD at core boundary. **Cons:** More surface. **Effort:** Small. **Risk:** Low.
2. **Delete only:** Add only `delete_document` to keep interface minimal; get/list can follow later. **Pros:** Addresses most critical gap. **Cons:** Read/list still missing. **Effort:** Small. **Risk:** Low.
3. **Leave as-is for scaffold:** Document in ADR/module that document delete and list are deferred; agents will get them when API is added. **Pros:** No code change now. **Cons:** CRUD remains incomplete. **Effort:** None. **Risk:** Medium (discovery and cleanup harder).

**Recommended Action:** (To be filled during triage.)

## Technical Details

- **Affected files:** `engine/crates/core/src/ports/knowledge.rs`, any type that implements KnowledgeInterface.
- **Related:** When adding gRPC/MCP, expose these primitives for agent parity (agent-native #4).

## Acceptance Criteria

- [ ] KnowledgeInterface includes at least delete_document (and optionally get_document_chunks or list_document_ids).
- [ ] Bridge/adapter implements the new methods.
- [ ] Docs updated to reflect document lifecycle at core boundary.

## Work Log

| Date | Action / Learning |
|------|-------------------|
| (Review) | Finding created from agent-native-reviewer report. |

## Resources

- Agent-native-reviewer: "Extend KnowledgeInterface for document CRUD".
- CRUD completeness audit (knowledge document: 3/4 with update missing).
