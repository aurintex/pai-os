---
name: paios-implement
description: >-
  Execute a paiOS Taskmaster m0 task end-to-end: load context from the
  linked GitHub issue and docs, implement strictly within the declared
  write-set, run the verify command, and mark the task done. Use whenever
  the user says "arbeite an Task N" or "implement task N".
---

<!-- SSoT: .agents/skills/paios-implement/SKILL.md — run scripts/dev/sync-paios-skills.sh after edits -->

# Skill: paios-implement

You are executing a paiOS Taskmaster m0 task. Follow the Task Lifecycle from AGENTS.md exactly.

## Step 1 — Load the task

```bash
task-master show <ID> --tag m0
```

Read the full `details` field before touching any file. If `details` is missing required sections (Role / Goal Prompt / Load context / Acceptance / Verify), stop and report BLOCKED.

## Step 2 — Fetch context (links only, never from memory)

Follow every link in the "Load context" section of the task details:
- Open the GitHub issue (`https://github.com/aurintex/pai-os/issues/<NN>`) and read Goal, Scope, Done-when.
- Read the referenced ADR(s) in `docs/src/content/docs/architecture/adr/`.
- Read the module doc in `docs/src/content/docs/architecture/modules/<crate>.mdx`.
- Read feature flags in `docs/src/content/docs/architecture/workspace-and-build.mdx`.

## Step 3 — Check dependencies

```bash
task-master list --tag m0 | grep -E "pending|in_progress"
```

If any dependency task is not `done`, mark this task BLOCKED with a comment listing the blocking task IDs. Do not implement.

## Step 4 — Check write-set

The task `details` declares a **Write-set**. Only modify those files. If you need to touch a file outside the write-set, surface this as a scope question — do not silently widen scope.

## Step 5 — Implement

- Follow [Rust Style](docs/src/content/docs/guides/contributing/rust-style.mdx): prefer stack, generics over `Box<dyn Trait>`, `thiserror` in libs, `anyhow` only in `pai-engine`.
- No `unwrap()` in library crates.
- Feature-gate all new code behind the correct feature flag (see workspace-and-build.mdx).
- Write unit/integration tests for all new behavior.
- No comments that describe WHAT the code does — only WHY when non-obvious.

## Step 6 — Verify

Run the exact command from the task's **Verify** section. Also run:

```bash
cd engine && cargo fmt --all -- --check
cd engine && cargo clippy --all-targets -- -D warnings
cd engine && cargo test
```

All three must pass before marking done.

## Step 7 — Mark done

```bash
task-master set-status --id <ID> --status done --tag m0
```

Only after Step 6 passes. Then report: task ID, what was implemented, verify output summary.

## Terminal states

- **DONE**: verify passes, task marked done.
- **BLOCKED**: dependency not done, missing context, or scope question. Leave a comment explaining why.
- **FAILED**: three consecutive verify attempts failed with the same error. Leave a comment and stop.
