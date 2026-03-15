---
status: pending
priority: p1
issue_id: "001"
tags: code-review, security, knowledge
dependencies: []
---

# P1: Full metadata written to audit log (PII risk)

## Problem Statement

`KnowledgeManager::ingest_document` builds an `AuditEvent` with `metadata: metadata.clone()`. Any PII or sensitive data in `Metadata` (e.g. user_id, filenames, tags) is stored in the append-only audit log with no redaction or minimization. ADR-010 defers "privacy-preserving audit payload policy"; current code makes the problem concrete and can cause compliance (e.g. DSGVO) issues.

## Findings

- **Location:** `engine/crates/knowledge/src/domain/mod.rs` (lines 35-39).
- **Evidence:** `AuditEvent { ..., metadata: metadata.clone() }` passes through full caller metadata into the audit log.
- **Source:** Security-sentinel review.

## Proposed Solutions

1. **Minimize at source:** Do not pass full `metadata` into `AuditEvent`; pass a minimized or allowlisted subset (e.g. document_id, action, actor, timestamp only). Add a helper that strips PII from metadata before appending. **Pros:** Clear contract. **Cons:** Requires defining allowlist. **Effort:** Small. **Risk:** Low.
2. **Redact in adapter:** Keep port as-is; document that adapters must redact/minimize before persisting. **Pros:** Flexible. **Cons:** Every adapter must implement correctly. **Effort:** Small for doc, Medium for each adapter. **Risk:** Medium (easy to forget).
3. **New audit payload type:** Introduce `AuditPayload` with only safe fields (action, actor, resource_id, timestamp) and use it in `AuditLog::append_event`. **Pros:** Type-level safety. **Cons:** Port and all adapters change. **Effort:** Medium. **Risk:** Low.

**Recommended Action:** (To be filled during triage.)

## Technical Details

- **Affected files:** `engine/crates/knowledge/src/domain/mod.rs`, `engine/crates/knowledge/src/ports/mod.rs` (AuditEvent type).
- **Downstream:** Any adapter implementing `AuditLog`; compliance and retention policy.

## Acceptance Criteria

- [ ] Audit log never persists full arbitrary metadata from callers.
- [ ] Document or type enforces minimized/allowlisted audit fields.
- [ ] ADR or knowledge module doc updated with audit payload policy.

## Work Log

| Date | Action / Learning |
|------|-------------------|
| (Review) | Finding created from security-sentinel report. |

## Resources

- Security-sentinel review (Critical #1).
- ADR-010 deferred item: "Privacy-preserving audit payload policy".
