---
status: pending
priority: p3
issue_id: "009"
tags: code-review, security, knowledge, quality
dependencies: []
---

# P3: Audit gaps, ID validation, and port contracts (documentation / future)

## Problem Statement

Several important-but-not-critical items: (1) Conversation memory appends are not audited (audit gap). (2) DocumentId/ChunkId are unconstrained strings; future adapters need validation/normalization to avoid injection or path issues. (3) write_audit_entry summary is free-form (PII risk if callers pass user content). (4) AuditLog port does not enforce append-only or integrity at the type level. (5) Metadata type allows arbitrary key/value (PII in metadata). Document or enforce limits and contracts so future adapters and callers follow safe patterns.

## Findings

- **Locations:** `engine/crates/knowledge/src/domain/mod.rs` (remember_message does not call audit); `engine/crates/knowledge/src/ports/mod.rs` (DocumentId, ChunkId, Metadata, AuditLog); `engine/crates/core/src/ports/knowledge.rs` (write_audit_entry summary).
- **Source:** Security-sentinel (Important #2, #3, #4, #7; Nice #5, #6, #9).

## Proposed Solutions

1. **Document in port/module docs:** Add doc comments and a short "Security and compliance" subsection in the knowledge module doc: audit only non-PII/summarized data; IDs must be safe for storage and logging; summary must be minimized; metadata must not contain PII; conversation memory audit optional but recommended for compliance. **Pros:** Clear contract. **Cons:** No enforcement. **Effort:** Small. **Risk:** Low.
2. **Add optional memory audit:** In KnowledgeManager::remember_message, call audit.append_event with action/session_id/timestamp only (no content). **Pros:** Closes audit gap. **Cons:** More events. **Effort:** Small. **Risk:** Low.
3. **Restrict types later:** Introduce validated ID types and AuditPayload with only safe fields in a future iteration. **Pros:** Type safety. **Cons:** Breaking change. **Effort:** Medium. **Risk:** Medium.

**Recommended Action:** (To be filled during triage.)

## Technical Details

- **Affected files:** `engine/crates/knowledge/src/ports/mod.rs`, `engine/crates/knowledge/src/domain/mod.rs`, `docs/src/content/docs/architecture/modules/knowledge.mdx`.
- **Related:** 001 (audit PII via metadata); 002/006 (systemd).

## Acceptance Criteria

- [ ] Knowledge ports and Core interface document safe use of IDs, metadata, and audit summary.
- [ ] Optionally: conversation memory appends produce an audit event (minimal payload).
- [ ] No breaking API change unless triaged as required.

## Work Log

| Date | Action / Learning |
|------|-------------------|
| (Review) | Finding created from security-sentinel report (consolidated P2/P3 items). |

## Resources

- Security-sentinel: #2, #3, #4, #5, #6, #7, #8, #9.
- ADR-010: deferred "privacy-preserving audit payload policy".
