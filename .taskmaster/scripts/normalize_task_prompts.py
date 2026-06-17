#!/usr/bin/env python3
"""normalize_task_prompts.py — Normalize paiOS Taskmaster task details to the standard template.

Checks that each task's `details` field contains the standard sections.
In --fix mode, prepends a template stub for any missing section.

Usage:
  python3 .taskmaster/scripts/normalize_task_prompts.py [--fix] [--tag m0] [path/to/tasks.json]

Exit 0 = all normalized (or no issues found), 1 = issues found (in check mode).
"""

import json
import sys
import re
import argparse

TEMPLATE_SECTIONS = [
    "**Role:**",
    "**Goal Prompt:**",
    "**Load context (links only, never copy):**",
    "**Non-goals:**",
    "**Write-set:**",
    "**Acceptance:**",
    "**Verify:**",
    "**Stop conditions / terminal states:**",
]

REQUIRED_FOR_VALIDATION = ["Load context", "Acceptance", "Verify"]


def trim_kontext(details: str) -> str:
    """Remove trailing whitespace from each line, collapse 3+ blank lines to 2."""
    lines = [l.rstrip() for l in details.splitlines()]
    result = []
    blank_count = 0
    for line in lines:
        if line == "":
            blank_count += 1
            if blank_count <= 2:
                result.append(line)
        else:
            blank_count = 0
            result.append(line)
    return "\n".join(result).strip()


def normalize_task(task: dict, fix: bool = False) -> list:
    """Return list of missing sections. If fix=True, prepend stubs to details."""
    tid = task.get("id", "?")
    details = task.get("details", "")
    missing = []

    for section in REQUIRED_FOR_VALIDATION:
        if section.lower() not in details.lower():
            missing.append(section)

    if fix and missing:
        stub_lines = []
        for section in TEMPLATE_SECTIONS:
            if not any(s.lower() in details.lower() for s in [section]):
                key = section.strip("*").strip(":").strip()
                stub_lines.append(f"{section} [TODO: fill in for task {tid}]")
        if stub_lines:
            task["details"] = "\n".join(stub_lines) + "\n\n" + trim_kontext(details)

    return missing


def main() -> int:
    parser = argparse.ArgumentParser(description="Normalize task prompt details")
    parser.add_argument("path", nargs="?", default=".taskmaster/tasks/tasks.json")
    parser.add_argument("--fix", action="store_true", help="Write stub sections for missing ones")
    parser.add_argument("--tag", default="m0", help="Tag to normalize (default: m0)")
    args = parser.parse_args()

    try:
        with open(args.path) as f:
            data = json.load(f)
    except Exception as e:
        print(f"[normalize] ERROR: cannot load {args.path}: {e}", file=sys.stderr)
        return 1

    tag = args.tag
    if tag not in data:
        print(f"[normalize] INFO: no '{tag}' tag found, nothing to normalize")
        return 0

    tasks = data[tag].get("tasks", [])
    total_issues = 0

    for task in tasks:
        missing = normalize_task(task, fix=args.fix)
        if missing:
            print(f"[normalize] task {task.get('id')}: missing sections: {missing}")
            total_issues += len(missing)
        for sub in task.get("subtasks", []):
            sub_missing = normalize_task(sub, fix=args.fix)
            if sub_missing:
                print(f"[normalize] task {task.get('id')}.{sub.get('id')}: missing: {sub_missing}")
                total_issues += len(sub_missing)

    if args.fix and total_issues > 0:
        with open(args.path, "w") as f:
            json.dump(data, f, indent=2, ensure_ascii=False)
            f.write("\n")
        print(f"[normalize] Wrote stubs for {total_issues} missing sections to {args.path}")
        return 0

    if total_issues > 0:
        print(f"[normalize] {total_issues} missing section(s) found (run with --fix to add stubs)")
        return 1

    print(f"[normalize] OK: all tasks in '{tag}' tag have required sections")
    return 0


if __name__ == "__main__":
    sys.exit(main())
