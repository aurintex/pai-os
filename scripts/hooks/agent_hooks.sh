#!/usr/bin/env bash
# scripts/hooks/agent_hooks.sh — AI agent hook dispatcher for paiOS
#
# Subcommands:
#   format-rust       Run cargo fmt (advisory, non-blocking)
#   verify-rust       Run cargo clippy + build (advisory, timeout 120s)
#   validate-tasks    Validate .taskmaster/tasks/tasks.json m0 tag
#
# Called by .claude/settings.json PostToolUse and Stop hooks.
# All subcommands are advisory: they log but never fail the agent turn.

set -euo pipefail

SUBCOMMAND="${1:-help}"
REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
ENGINE_DIR="$REPO_ROOT/engine"

case "$SUBCOMMAND" in
  format-rust)
    # Only run if a .rs file was modified (passed via stdin JSON from Claude Code)
    if command -v cargo >/dev/null 2>&1 && [ -d "$ENGINE_DIR" ]; then
      (cd "$ENGINE_DIR" && cargo fmt --all 2>/dev/null) \
        && echo "[hooks] format-rust: done" \
        || echo "[hooks] format-rust: cargo fmt failed (advisory)" >&2
    fi
    ;;

  verify-rust)
    if command -v cargo >/dev/null 2>&1 && [ -d "$ENGINE_DIR" ]; then
      timeout 120 bash -c "cd '$ENGINE_DIR' && cargo clippy --all-targets -- -D warnings 2>&1 | tail -5" \
        && echo "[hooks] verify-rust: clean" \
        || echo "[hooks] verify-rust: issues found (advisory, check manually)" >&2
    fi
    ;;

  validate-tasks)
    TASKS_JSON="$REPO_ROOT/.taskmaster/tasks/tasks.json"
    VALIDATOR="$REPO_ROOT/.taskmaster/scripts/validate_tasks.py"
    if [ -f "$VALIDATOR" ] && [ -f "$TASKS_JSON" ]; then
      python3 "$VALIDATOR" "$TASKS_JSON" \
        && echo "[hooks] validate-tasks: ok" \
        || echo "[hooks] validate-tasks: issues found (advisory)" >&2
    fi
    ;;

  help|*)
    echo "Usage: agent_hooks.sh <format-rust|verify-rust|validate-tasks>"
    ;;
esac
