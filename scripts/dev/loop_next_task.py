#!/usr/bin/env python3
"""loop_next_task.py — Pick the next eligible task for the autonomous m0 loop.

The autonomous loop only works an explicit allow-list of "loop-friendly" tasks
(clear write-set, deterministic `cargo test` verify, no human decision needed).
Hard / decision-heavy tasks (llama.cpp build, gRPC routing security, HF Hub
download, NPU bindings, on-device E2E) are deliberately excluded so the loop
never walks into a decision only a human should make.

A task is *eligible* when:
  - its id is in the active allow-list, AND
  - its status is `pending`, AND
  - every dependency task has status `done`.

The script is the single source of truth for the loop's allow-list and the
model/effort policy (kept in sync with .taskmaster/loop-prompt.md).

Usage:
  python3 scripts/dev/loop_next_task.py                 # print next eligible id, or NONE
  python3 scripts/dev/loop_next_task.py --list          # table of all eligible tasks
  python3 scripts/dev/loop_next_task.py --plan          # every allow-list task + status
  python3 scripts/dev/loop_next_task.py --wave 2        # extend allow-list with wave 2
  python3 scripts/dev/loop_next_task.py --allow 4,5,6   # explicit allow-list override
  python3 scripts/dev/loop_next_task.py --json          # machine-readable next task

Exit 0 always (advisory tool). Prints NONE when nothing is eligible.
"""

import argparse
import json
import sys

TASKS_PATH = ".taskmaster/tasks/tasks.json"
TAG = "m0"

# Allow-list waves (see .taskmaster/loop-prompt.md).
WAVE_1 = [4, 5, 6, 11, 15, 19]          # fully autonomous, zero decisions
WAVE_2 = [8, 9, 10, 16]                  # needs the "wire mock adapter" OK (--wave 2)
# Excluded on purpose (decision/hardware/network backlog): 7, 12, 13, 14, 17, 18, 20


def model_for(complexity, role):
    """Return (model, effort) for a task complexity and subagent role.

    Single source of truth for the loop's model policy. Roles:
      'context'    — Explore subagent that digests issue/ADR/module docs
      'implement'  — implementer subagent (paios-implement)
      'review'     — adversarial reviewer subagent (paios-review)
    """
    cx = complexity or 5
    if role == "context":
        return ("sonnet", "low")
    if role == "implement":
        if cx <= 4:
            return ("sonnet", "medium")
        if cx <= 6:
            return ("sonnet", "high")
        # security/most-complex tasks get the top effort tier
        return ("opus", "xhigh" if cx >= 8 else "high")
    if role == "review":
        if cx <= 6:
            return ("sonnet", "high")
        return ("opus", "high")
    return ("sonnet", "medium")


def load_tasks(path):
    with open(path) as f:
        data = json.load(f)
    return data.get(TAG, {}).get("tasks", [])


def eligible(task, by_id):
    if task.get("status") != "pending":
        return False
    for dep in task.get("dependencies", []):
        dep_t = by_id.get(int(dep))
        if dep_t is None or dep_t.get("status") != "done":
            return False
    return True


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--path", default=TASKS_PATH)
    ap.add_argument("--wave", type=int, default=1, choices=[1, 2])
    ap.add_argument("--allow", default=None, help="comma-separated task ids, overrides --wave")
    ap.add_argument("--list", action="store_true", help="table of all eligible allow-list tasks")
    ap.add_argument("--plan", action="store_true", help="every allow-list task with status")
    ap.add_argument("--json", action="store_true", help="machine-readable next task")
    args = ap.parse_args()

    if args.allow:
        allow = [int(x) for x in args.allow.split(",") if x.strip()]
    else:
        allow = WAVE_1 + (WAVE_2 if args.wave >= 2 else [])

    try:
        tasks = load_tasks(args.path)
    except Exception as e:  # noqa: BLE001 - advisory tool, report and exit clean
        print(f"NONE  # cannot load {args.path}: {e}", file=sys.stderr)
        print("NONE")
        return 0

    by_id = {int(t["id"]): t for t in tasks}
    allow_tasks = [by_id[i] for i in allow if i in by_id]
    allow_tasks.sort(key=lambda t: int(t["id"]))

    if args.plan:
        for t in allow_tasks:
            elig = "READY" if eligible(t, by_id) else t.get("status", "?")
            m, e = model_for(t.get("complexity"), "implement")
            print(f"{t['id']:>3}  cx={t.get('complexity')}  {elig:<10} {m}/{e}  {t['title']}")
        return 0

    eligibles = [t for t in allow_tasks if eligible(t, by_id)]

    if args.list:
        if not eligibles:
            print("NONE")
            return 0
        for t in eligibles:
            m, e = model_for(t.get("complexity"), "implement")
            print(f"{t['id']:>3}  cx={t.get('complexity')}  {m}/{e}  {t['title']}")
        return 0

    nxt = eligibles[0] if eligibles else None

    if args.json:
        if nxt is None:
            print(json.dumps({"id": None}))
            return 0
        cx = nxt.get("complexity")
        print(json.dumps({
            "id": int(nxt["id"]),
            "complexity": cx,
            "title": nxt["title"],
            "verify": None,
            "models": {
                "context": model_for(cx, "context"),
                "implement": model_for(cx, "implement"),
                "review": model_for(cx, "review"),
            },
        }))
        return 0

    print(int(nxt["id"]) if nxt else "NONE")
    return 0


if __name__ == "__main__":
    sys.exit(main())
