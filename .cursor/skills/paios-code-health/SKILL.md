---
name: paios-code-health
description: >-
  Run the paiOS quality gate suite: cargo fmt, clippy, tests, Taskmaster
  task validation, and a quick architecture sanity check. Reports what passes,
  what fails, and what to fix. Use for "check code health", "run quality gates",
  or after a batch of implementation tasks.
---

<!-- SSoT: .agents/skills/paios-code-health/SKILL.md — run scripts/dev/sync-paios-skills.sh after edits -->

# Skill: paios-code-health

Run all quality gates and report results. All commands run from `engine/` unless noted.

## Gate 1 — Format

```bash
cd engine && cargo fmt --all -- --check
```

If it fails: run `cargo fmt --all` to fix, then re-check. Report which files were reformatted.

## Gate 2 — Clippy

```bash
cd engine && cargo clippy --all-targets -- -D warnings
```

Report all warnings. Do not suppress with `#[allow(...)]` unless the reason is documented inline.

## Gate 3 — Tests

```bash
cd engine && cargo test
```

Report failing tests with their name and the first 10 lines of output. Do not mark passing if any test fails.

## Gate 4 — Feature profiles

Check that key profiles compile without error:

```bash
cd engine && cargo build --features desktop 2>&1 | tail -3
```

For `rockchip` profile (if aarch64 target is installed):
```bash
cd engine && cargo build --features rockchip --target aarch64-unknown-linux-gnu 2>&1 | tail -3
```

## Gate 5 — Taskmaster task validation

```bash
python3 .taskmaster/scripts/validate_tasks.py .taskmaster/tasks/tasks.json
```

Report any tasks missing required sections (Load context / Acceptance / Verify).

## Gate 6 — Architecture sanity (quick)

Check for cross-crate import violations:

```bash
grep -r "use inference::\|use api::" engine/crates/core/src/ 2>/dev/null && echo "VIOLATION: core imports domain crate" || echo "OK: core has no domain imports"
grep -r "use api::" engine/crates/inference/src/ 2>/dev/null && echo "VIOLATION: inference imports api" || echo "OK"
grep -r "use inference::" engine/crates/api/src/ 2>/dev/null && echo "VIOLATION: api imports inference directly" || echo "OK (should go via core ports)"
```

## Summary format

```
Gate 1 fmt:       PASS / FAIL (N files reformatted)
Gate 2 clippy:    PASS / FAIL (N warnings)
Gate 3 tests:     PASS / FAIL (N/M tests, N failing)
Gate 4 desktop:   PASS / FAIL
Gate 5 tasks:     PASS / FAIL (N issues)
Gate 6 arch:      PASS / FAIL (N violations)

Overall: PASS (all green) | FAIL (fix gates above before next task)
```
