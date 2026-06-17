#!/usr/bin/env python3
"""validate_tasks.py — Validate paiOS Taskmaster task JSON for the m0 tag.

Checks:
- Required fields present (title, description, details, status, dependencies, complexity)
- details contains required sections: 'Load context', 'Acceptance', 'Verify'
- details contains at least one GitHub issue link (#NNN) or docs link
- dependencies reference existing task IDs
- No dependency cycles
- complexity in range 1-9

Exit 0 if valid, non-zero if any check fails (advisory: called from hooks).
"""

import json
import sys
import re

REQUIRED_FIELDS = ["id", "title", "description", "details", "status", "dependencies", "complexity"]
# Subtasks have a lighter schema — no complexity, no full template required
REQUIRED_SUBTASK_FIELDS = ["id", "title", "status"]
DETAILS_REQUIRED_SECTIONS = ["Load context", "Acceptance", "Verify"]
GITHUB_LINK_RE = re.compile(r"#\d+|github\.com/aurintex/pai-os/issues/\d+")
DOCS_LINK_RE = re.compile(r"docs/src/content/docs/|ADR-\d+|\.mdx")


def fail(msg: str) -> None:
    print(f"[validate_tasks] FAIL: {msg}", file=sys.stderr)


def normalize_id(task_id) -> str:
    return str(task_id)


def check_cycles(tasks: list) -> bool:
    """Return True if no cycles, False if cycles detected."""
    id_map = {normalize_id(t["id"]): t for t in tasks}
    visited = set()
    in_stack = set()

    def dfs(tid: str) -> bool:
        if tid in in_stack:
            fail(f"dependency cycle detected involving task {tid}")
            return False
        if tid in visited:
            return True
        visited.add(tid)
        in_stack.add(tid)
        for dep in id_map.get(tid, {}).get("dependencies", []):
            if not dfs(normalize_id(dep)):
                return False
        in_stack.remove(tid)
        return True

    for tid in id_map:
        if tid not in visited:
            if not dfs(tid):
                return False
    return True


def check_task(task: dict, all_ids: set, is_subtask: bool = False) -> list:
    errors = []
    tid = task.get("id", "?")

    required = REQUIRED_SUBTASK_FIELDS if is_subtask else REQUIRED_FIELDS
    for field in required:
        if field not in task:
            errors.append(f"{'subtask' if is_subtask else 'task'} {tid}: missing field '{field}'")

    # Full template sections are only required for top-level tasks
    if not is_subtask:
        details = task.get("details", "")
        if details:
            for section in DETAILS_REQUIRED_SECTIONS:
                if section.lower() not in details.lower():
                    errors.append(f"task {tid}: details missing required section '{section}'")
            has_link = GITHUB_LINK_RE.search(details) or DOCS_LINK_RE.search(details)
            if not has_link:
                errors.append(f"task {tid}: details has no GitHub issue link or docs link")

        complexity = task.get("complexity")
        if complexity is not None and not (1 <= int(complexity) <= 9):
            errors.append(f"task {tid}: complexity {complexity} out of range 1-9")

    for dep in task.get("dependencies", []):
        if normalize_id(dep) not in all_ids:
            errors.append(f"{'subtask' if is_subtask else 'task'} {tid}: dependency '{dep}' not found in task list")

    return errors


def check_archived_tag(data: dict) -> None:
    for tag in data:
        if tag == "archive":
            print("[validate_tasks] INFO: 'archive' tag found (ok)")


def main() -> int:
    path = sys.argv[1] if len(sys.argv) > 1 else ".taskmaster/tasks/tasks.json"
    try:
        with open(path) as f:
            data = json.load(f)
    except Exception as e:
        fail(f"cannot load {path}: {e}")
        return 1

    check_archived_tag(data)

    tag = "m0"
    if tag not in data:
        print(f"[validate_tasks] INFO: no '{tag}' tag found, nothing to validate")
        return 0

    tasks = data[tag].get("tasks", [])
    all_ids = {normalize_id(t["id"]) for t in tasks}

    all_errors = []
    for task in tasks:
        all_errors.extend(check_task(task, all_ids, is_subtask=False))
        for sub in task.get("subtasks", []):
            all_errors.extend(check_task(sub, all_ids, is_subtask=True))

    if not check_cycles(tasks):
        all_errors.append("dependency cycle detected (see above)")

    if all_errors:
        for e in all_errors:
            fail(e)
        print(f"[validate_tasks] {len(all_errors)} error(s) found in '{tag}' tag", file=sys.stderr)
        return 1

    print(f"[validate_tasks] OK: {len(tasks)} tasks in '{tag}' tag validated")
    return 0


if __name__ == "__main__":
    sys.exit(main())
