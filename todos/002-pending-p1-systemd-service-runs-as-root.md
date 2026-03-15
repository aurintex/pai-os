---
status: pending
priority: p1
issue_id: "002"
tags: code-review, security, os, systemd
dependencies: []
---

# P1: pai-engine systemd service runs as root

## Problem Statement

The systemd unit for pai-engine does not set `User=` or `Group=`. The service runs as root, increasing impact of any compromise (e.g. of the engine process or a dependency). For a service that may handle user content and inference, least privilege is important.

## Findings

- **Location:** `os/overlays/app/pai-engine/etc/systemd/system/pai-engine.service`.
- **Evidence:** No `User=` or `Group=` directives; `[Service]` only has Type, ExecStart, Restart.
- **Source:** Security-sentinel review (Critical #10).

## Proposed Solutions

1. **Dedicated user/group:** Create a `pai-engine` user and group (e.g. in overlay or package), set `User=pai-engine` and `Group=pai-engine` in the unit. Ensure data/workspace dirs are owned by that user. **Pros:** Standard practice. **Cons:** Need to create user in image/recipe. **Effort:** Small. **Risk:** Low.
2. **Use existing system user:** If the OS recipe already has a suitable unprivileged user, use it. **Pros:** No new user. **Cons:** May not exist. **Effort:** Small. **Risk:** Low.
3. **Defer to post-install script:** Create user in a recipe step and set `User=`/`Group=` in the unit. **Pros:** Flexible. **Cons:** Must keep in sync. **Effort:** Small. **Risk:** Low.

**Recommended Action:** (To be filled during triage.)

## Technical Details

- **Affected files:** `os/overlays/app/pai-engine/etc/systemd/system/pai-engine.service`; possibly recipe/overlay to add user/group.
- **Related:** Security-sentinel also recommends systemd hardening (PrivateTmp, NoNewPrivileges, ProtectSystem, ReadWritePaths, etc.) as P2.

## Acceptance Criteria

- [ ] Service runs as a non-root user.
- [ ] Data/workspace directories (if any) are owned by that user.
- [ ] No regression in engine startup or file access.

## Work Log

| Date | Action / Learning |
|------|-------------------|
| (Review) | Finding created from security-sentinel report. |

## Resources

- Security-sentinel review (Critical #10).
- systemd documentation: User=, Group=.
