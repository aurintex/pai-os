---
status: pending
priority: p2
issue_id: "008"
tags: code-review, architecture, knowledge, ci
dependencies: []
---

# P2: Knowledge adapters stub and missing "test" profile

## Problem Statement

(1) `knowledge/src/adapters/mod.rs` is a stub; there are no feature-gated adapter modules (e.g. sqlite.rs, mock.rs) or concrete types implementing the knowledge ports. (2) The workspace doc and feature matrix describe a "test" profile that enables `knowledge_mock` (and other *_mock), but `pai-engine/Cargo.toml` has no `test` feature, so CI cannot rely on a single test profile.

## Findings

- **Location:** `engine/crates/knowledge/src/adapters/mod.rs` (empty beyond comment); `engine/pai-engine/Cargo.toml` (no `test` feature).
- **Evidence:** adapters/mod.rs has no `#[cfg(feature = "knowledge_sqlite")]` or `knowledge_mock` modules; pai-engine features are default, desktop, rockchip, and re-exports.
- **Source:** Architecture-strategist.

## Proposed Solutions

1. **Add mock adapter and test profile:** Add `adapters/mock.rs` gated by `knowledge_mock` with a type implementing DocumentStore, ConversationMemory, AuditLog (and optionally stub VectorSearch/FullTextSearch). Add `test = ["vision_mock", "audio_mock", "infer_mock", "knowledge_mock", "api_mock", "periph_mock"]` to pai-engine and use it in CI. **Pros:** Real adapters boundary and runnable test build. **Cons:** Some implementation work. **Effort:** Medium. **Risk:** Low.
2. **Test profile only:** Add the `test` feature to pai-engine so docs match; leave adapters as stub. **Pros:** Doc/code alignment. **Cons:** knowledge_mock still has no impl. **Effort:** Small. **Risk:** Low.
3. **Document as conceptual:** In workspace-and-build and feature matrix, state that "test" profile is conceptual and tests must enable needed *_mock features explicitly until a profile is added. **Pros:** No code change. **Cons:** Inconvenient for CI. **Effort:** Trivial. **Risk:** Medium.

**Recommended Action:** (To be filled during triage.)

## Technical Details

- **Affected files:** `engine/crates/knowledge/src/adapters/mod.rs` and new `mock.rs`; `engine/pai-engine/Cargo.toml`; optionally CI workflow.
- **Related:** Wiring KnowledgeInterface (003) will need at least one concrete adapter (mock or SQLite).

## Acceptance Criteria

- [ ] Either (a) at least one feature-gated adapter module exists (e.g. mock) with a type implementing the ports, or (b) the stub is explicitly documented as intentional for this phase.
- [ ] Either (a) pai-engine has a `test` feature that aggregates *_mock and is used in CI, or (b) docs state that test profile is conceptual and how to run tests with mocks.

## Work Log

| Date | Action / Learning |
|------|-------------------|
| (Review) | Finding created from architecture-strategist report. |

## Resources

- Architecture-strategist: "Adapters boundary is still a stub", "test profile is documented but not defined".
- Workspace and Build: Feature Flag Matrix.
