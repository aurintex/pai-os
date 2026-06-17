---
name: paios-loop
description: >-
  Drive the autonomous paiOS M0 loop: repeatedly select the next eligible
  loop-friendly Taskmaster task, delegate context-load / implement / review to
  subagents at complexity-matched models, run the quality gate, commit per task,
  and mark it done. Skips the decision-heavy backlog. Use for "start the loop",
  "run the autonomous loop", "arbeite die m0-Tasks autonom ab".
---

<!-- SSoT: .agents/skills/paios-loop/SKILL.md — run scripts/dev/sync-paios-skills.sh after edits -->

# Skill: paios-loop

You are the **driver** of the autonomous paiOS M0 loop. The full operating
instructions live in `.taskmaster/loop-prompt.md` — read that file first and
follow it exactly. This skill is the entry point and the guardrails.

## Argument

`/paios-loop` runs **Wave 1**. `/paios-loop wave2` (argument `$ARGUMENTS` contains `wave2`)
is the human's explicit OK to also run **Wave 2** — tasks 8, 9, 10, 16, with task 8 wired to
the mock adapter as the desktop default. Never enable Wave 2 without that argument.

## Before you start (the agent does all of this — no manual setup needed)

1. **Pin the model:** run on Sonnet (`/model sonnet`). The loop spawns Opus only for cx≥7
   tasks; the allow-list has none, so the run is Sonnet-only.
2. **Dedicated branch:** `git checkout -b chore/m0-autoloop 2>/dev/null || git checkout
   chore/m0-autoloop`. Task commits must never land on `main`/release branches.
3. **Clean tree:** `git status --porcelain` must be empty. If dirty, stop and report — do not
   auto-commit unknown WIP.
4. **Pre-flight:** `gh auth status` (issues readable) and `python3
   scripts/dev/loop_next_task.py` (must print a task id, e.g. `4`).

This skill is built to run unattended (e.g. overnight with `--dangerously-skip-permissions`):
it stays inside the allow-list and each task's write-set, commits to a dedicated branch, and
stops cleanly when no eligible task remains.

## Scope

- Default = **Wave 1**: tasks 4, 5, 6, 11, 15, 19 (fully autonomous, zero decisions).
- **Wave 2** (only with the `wave2` argument): + 8, 9, 10, 16; pass `--wave 2` to the selector.
- **Never touch** the backlog 7, 12, 13, 14, 17, 18, 20 (external build / routing-security /
  HF-network / NPU-hardware / on-device-E2E — human decisions).

## Run

Follow `.taskmaster/loop-prompt.md`'s per-task lifecycle for each eligible task until the
selector prints `NONE`:

1. `python3 scripts/dev/loop_next_task.py` → next task id (or `NONE` → stop with a report).
2. **Context-load** subagent (Explore, sonnet/low) → compact digest of issue + ADR + module doc.
3. **Implement** subagent runs `paios-implement <id>` within the write-set; model/effort from
   `loop_next_task.py --json` (the policy SSoT).
4. **Gate (you run it):** the task verify command + `cargo fmt --all -- --check` +
   `cargo clippy --all-targets -- -D warnings` + `cargo test`. All green.
5. **Adversarial review** subagent runs `paios-review` on the diff; one fix round max for real
   gaps, then re-gate.
6. **Commit** one Conventional Commit (title = task title), then
   `task-master set-status --id <id> --status done --tag m0`.

## Stop conditions

- **DONE-ALL**: selector prints `NONE` → final report (tasks done, commits, m0 `N/20`, untouched
  backlog) and stop.
- **BLOCKED** (decision needed / write-set too narrow): mark the task `blocked` with a one-line
  reason and continue to the next eligible task — do not halt the whole loop.
- **FAILED** (3× same verify error): mark `blocked` with the error and continue.

The loop is resumable: progress is durable via per-task commits + Taskmaster status. Re-running
the skill resumes from the next eligible task.
