---
status: pending
priority: p3
issue_id: "010"
tags: code-review, security, os
dependencies: []
---

# P3: OS recipe and fstab hardening (nosuid, script safety, HTTP mirror)

## Problem Statement

(1) `/data` in fstab uses `defaults,noatime`; adding `nosuid,nodev` reduces risk. (2) Recipe `action: run` with script (e.g. systemctl enable) must only run from trusted YAML; document or sandbox. (3) base recipes use HTTP mirror; prefer HTTPS if supported. (4) Placeholder binary and udev rules: when replaced, avoid logging argv and avoid broad MODE="0666". Document for future edits.

## Findings

- **Locations:** `os/overlays/radxa-rock5c/etc/fstab`; `os/recipes/app/pai-engine.yaml`, `os/recipes/base.yaml`, `os/recipes/base-arm64.yaml`; `os/overlays/app/pai-engine/usr/local/bin/pai-engine`; `os/overlays/radxa-rock5c/etc/udev/rules.d/99-radxa-rock5c.rules`.
- **Source:** Security-sentinel (Important #12, #13; Nice #14, #15, #16).

## Proposed Solutions

1. **Fstab:** Add `nosuid,nodev` to `/data` mount options where appropriate. **Effort:** Small.
2. **Recipes:** Document in README or contributing that recipe YAML and script execution are supply-chain controlled; do not parse untrusted YAML as recipe input. **Effort:** Small.
3. **Mirror:** Switch to HTTPS for deb.debian.org if the recipe runner supports it. **Effort:** Small.
4. **Placeholders:** Add one-line comments in placeholder files: "When replacing: do not log argv; udev rules should be scoped, not MODE=0666." **Effort:** Trivial.

**Recommended Action:** (To be filled during triage.)

## Acceptance Criteria

- [ ] Fstab and recipe safety documented or applied as above.
- [ ] No regression in image build or boot.

## Work Log

| Date | Action / Learning |
|------|-------------------|
| (Review) | Finding created from security-sentinel report. |

## Resources

- Security-sentinel #12, #13, #14, #15, #16.
