---
name: paios-review
description: >-
  Review a paiOS code change (diff, PR, or named files) against the project's
  architecture standards: hexagonal ports, feature-flag discipline, Rust style,
  ADR compliance, and test coverage. Returns a prioritised finding list.
  Use for "review this change", "check this PR", or "review task N output".
---

<!-- SSoT: .agents/skills/paios-review/SKILL.md — run scripts/dev/sync-paios-skills.sh after edits -->

# Skill: paios-review

You are reviewing paiOS Rust code. Load the relevant standards first, then assess the diff or files provided.

## Context to load before reviewing

- [Rust Style](docs/src/content/docs/guides/contributing/rust-style.mdx) — stack vs heap, generics, error types
- [ADR-004 Architecture](docs/src/content/docs/architecture/adr/004-system-architecture.mdx) — hexagonal, ports, feature flags, YAGNI
- [Standards](docs/src/content/docs/guides/contributing/standards.mdx) — code style, test requirements
- [Workflow](docs/src/content/docs/guides/contributing/workflow.mdx) — PR/branch/commit conventions
- [Definition of Done](.cursor/rules/definition-of-done.mdc)

## Review dimensions (check all)

**Architecture compliance**
- Domain crates (`common`, `core`, `inference`, `api`) must not import each other — only `core` is allowed to import `common`.
- New functionality behind a declared feature flag from workspace-and-build.mdx.
- Ports defined in `pai-core`, adapters in domain crates — no business logic leaking into adapters.
- No new direct dependencies in `pai-engine` that belong in a domain crate.

**Rust correctness**
- No `unwrap()` or `expect()` in library crates (`common`, `core`, `inference`, `api`).
- Errors: `thiserror` enums in libs, `anyhow` only in `pai-engine` binary.
- Async correctness: no blocking calls inside `tokio::spawn` without `spawn_blocking`.
- Cancellation: long-running operations check `CancellationToken` or select on shutdown signal.
- No `clippy::unwrap_used` violations (check with `cargo clippy --all-targets -- -D warnings`).

**Test coverage**
- New public functions have at least one unit test.
- Adapter integrations have at least one integration test (can use mock port).
- Tests don't rely on real hardware or network (use feature-gated mocks).

**Documentation**
- No comments that explain WHAT the code does — only WHY (non-obvious invariants, workarounds).
- Public types/functions have doc comments if they're part of a port trait.
- No em-dashes in doc comments.

**Conventional commits**
- Branch name: `type(scope)/short-description` per workflow.mdx.
- Commit messages: `type(scope): short description` — present tense, under 72 chars.

## Output format

Report findings grouped by severity:

**Critical** (blocks merge): correctness bugs, security issues, architecture violations, missing tests for new ports.

**Major** (should fix): unwrap in lib, missing feature gate, cross-crate import violation.

**Minor** (nice to have): style, naming, doc comment gaps.

For each finding: file:line, what the issue is, suggested fix. Skip findings where you're less than 70% confident.
