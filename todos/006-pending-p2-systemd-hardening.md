---
status: pending
priority: p2
issue_id: "006"
tags: code-review, security, os, systemd
dependencies: ["002"]
---

# P2: No systemd hardening on pai-engine service

## Problem Statement

The systemd unit does not use common hardening options (PrivateTmp, NoNewPrivileges, ProtectSystem, ProtectHome, ReadWritePaths, CapabilityBoundingSet, etc.). This leaves a larger attack surface and privilege retention if the process is compromised.

## Findings

- **Location:** `os/overlays/app/pai-engine/etc/systemd/system/pai-engine.service`.
- **Evidence:** Only Type, ExecStart, Restart; no hardening directives.
- **Source:** Security-sentinel (Important #11).

## Proposed Solutions

1. **Add recommended hardening:** Set `PrivateTmp=yes`, `NoNewPrivileges=yes`, `ProtectSystem=strict`, `ProtectHome=read-only`, `ReadWritePaths=` to engine data/workspace dirs, `CapabilityBoundingSet=`, `RestrictAddressFamilies=`, `RestrictSUIDSGID=`, `ProtectKernelTunables=yes`. **Pros:** Standard practice. **Cons:** Must align ReadWritePaths with actual paths. **Effort:** Small. **Risk:** Low (test startup and file access).
2. **Minimal set first:** Add only PrivateTmp, NoNewPrivileges, ProtectSystem=strict; expand later. **Pros:** Quick win. **Cons:** Still some surface. **Effort:** Small. **Risk:** Low.
3. **Document only:** Add a comment or doc that hardening should be applied before production. **Pros:** No change to unit. **Cons:** No actual hardening. **Effort:** Trivial. **Risk:** High (no mitigation).

**Recommended Action:** (To be filled during triage.)

## Technical Details

- **Affected files:** `os/overlays/app/pai-engine/etc/systemd/system/pai-engine.service`.
- **Note:** ReadWritePaths must include any directory the engine writes to (e.g. workspace, SQLite DB, logs).

## Acceptance Criteria

- [ ] Unit includes a documented set of hardening options.
- [ ] Engine still starts and can write to its data directories.
- [ ] No unnecessary capabilities retained.

## Work Log

| Date | Action / Learning |
|------|-------------------|
| (Review) | Finding created from security-sentinel report. |

## Resources

- Security-sentinel review (Important #11).
- systemd.exec(5) for hardening options.
