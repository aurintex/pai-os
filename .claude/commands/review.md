# /review — Review paiOS Code Against Architecture Standards

Load skill **paios-review** (`.claude/skills/paios-review/SKILL.md`; SSoT: `.agents/skills/paios-review/SKILL.md`).

Review the current `git diff` (or a named PR / file list if provided as argument) against paiOS architecture standards: hexagonal ports, feature-flag discipline, Rust style, ADR compliance, and test coverage.
