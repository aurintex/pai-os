# AI Agent Context Map

> **Start here**: This file serves as the primary index for AI agents (Cursor, Windsurf, Copilot) to understand the paiOS project structure, conventions, and active tasks.

## 1. Project Identity
**paiOS** is an open-source operating system for Personal AI Hardware (paiLink, paiGo).
- **Core Principle**: Privacy-first, local-only inference.
- **Architecture**: Hexagonal, Monorepo, Rust-based Engine.

## 2. Critical Context Files
Before writing code, **always** check these files for the latest standards:

| Topic | Source of Truth |
|-------|-----------------|
| **Architecture** | `docs/src/content/docs/architecture/adr/index.mdx` (ADRs) |
| **Workspace & Build** | `docs/src/content/docs/architecture/workspace-and-build.mdx` (engine/ layout, feature flags, crates) |
| **Coding Style** | `docs/src/content/docs/guides/contributing/standards.mdx` |
| **Rust Style & Best Practices** | `docs/src/content/docs/guides/contributing/rust-style.mdx` (stack vs heap, generics vs `Box<dyn Trait>`, embedded) |
| **Workflow** | `docs/src/content/docs/guides/contributing/workflow.mdx` |
| **Glossary** | `docs/src/content/docs/glossary.mdx` (link here when using defined terms) |
| **Task Status** | `.taskmaster/tasks/tasks.json` (via `task-master` CLI) |

## 3. Tool Usage Rules
- **Search first**: Use `grep` / `glob` to find existing patterns before inventing new ones.
- **No hallucinations**: If you need a library, check `Cargo.toml` or `package.json` first.
- **Tests**: All new features require unit tests. Run `cargo test` (Rust) or `npm test` (JS/TS).
- **GitHub issues**: When creating issues (e.g. via MCP or CLI), use **Conventional Commits** for the title: `type(scope): short description` (e.g. `feat(common): add config format detection`). See [Workflow](docs/src/content/docs/guides/contributing/workflow.mdx) for types, scope, and branch naming.
- **Documentation**: When editing docs, link to the [Glossary](docs/src/content/docs/glossary.mdx) for terms that have an entry (e.g. IPC, gRPC, HITL, UDS). Use `[Term](/glossary/#letter)`.
- **Rust code**: When writing or reviewing Rust in `engine/`, follow [Rust Style and Best Practices](docs/src/content/docs/guides/contributing/rust-style.mdx) (stack vs heap, generics vs `Box<dyn Trait>`, formatting).

## 4. Architecture Summary
- **Engine**: Rust daemon (`engine/`) running distinct threads for inference (NPU/CPU/GPU).
- **IPC**: gRPC over Unix Domain Sockets.
- **Docs**: Astro Starlight site (`docs/`).

## 5. Active Focus
Check the Task Master for the current priority:
`task-master next`

## 6. Cursor MCP (optional)
The repo ships with **Taskmaster** in `.cursor/mcp.json` so contributors can try it without setup (“ah, this could be useful”). Task data stays local (see [ADR-007](docs/src/content/docs/architecture/adr/007-project-management-strategy.mdx) and [ADR-009](docs/src/content/docs/architecture/adr/009-ai-context-strategy.mdx)); GitHub issues remain the source of truth. For MCP AI features (parse-prd, expand, research, etc.), add at least one provider API key: replace the placeholder values in the `env` section of `.cursor/mcp.json` (e.g. `ANTHROPIC_API_KEY`, `PERPLEXITY_API_KEY`) with your keys. For CLI-only use, `.env` is enough. See [Task Master configuration](https://docs.task-master.dev/getting-started/quick-start/configuration-quick).

## 7. Excalidraw MCP (optional)
**Not all contributors have this installed.** Architecture diagrams are `.excalidraw` files in `docs/public/images/Architecture/`. They can be edited in the browser (e.g. [Excalidraw](https://excalidraw.com)) or via the optional [yctimlin/mcp_excalidraw](https://github.com/yctimlin/mcp_excalidraw) MCP for AI-assisted read/edit/iterate.

- **If you don’t use the MCP**: Edit `.excalidraw` files by opening them in Excalidraw or the VS Code Excalidraw extension. Docs embed from `docs/public/images/Architecture/`; no MCP required.
- **If you use the MCP**: Clone the repo to a location of your choice, run `npm ci && npm run build`, then start the canvas (e.g. `HOST=0.0.0.0 PORT=3000 npm run canvas` in that clone). Add the server to `.cursor/mcp.json` with `command`/`args` pointing at your `dist/index.js` and `env.EXPRESS_SERVER_URL` set to your canvas URL (default `http://localhost:3000`). See the project README for Cursor config examples. Tools: `import_scene` (load file), `describe_scene` / `get_canvas_screenshot` (inspect), `export_scene` (save).
- **When giving instructions**: Prefer “open/edit the `.excalidraw` file in Excalidraw” unless the user has confirmed they use the Excalidraw MCP.

## Learned User Preferences
- Prefer simple, easy-to-read code over clever abstractions; avoid unnecessary refactors and discuss broader changes first.
- Keep documentation concise, with only the essential details, and avoid duplicated explanations across pages.
- Align module and architecture docs to a shared base structure, allowing small, justified deviations only when they clearly improve readability.
- Favor progress over perfection; ship a good, flexible architecture first and iterate rather than stalling on perfect designs.
- Use existing patterns in the codebase and docs before introducing new ones, to keep things consistent and easy to follow.
- Never use em dashes (—) in documentation; they read as AI-generated. Use a colon for "label: explanation" and commas or parentheses for asides.
- Avoid documentation that sounds AI-generated; prefer natural, human-sounding writing throughout.
- Apply 80/20 when improving or fixing docs: tackle only the most impactful issues, not everything at once.
- Strategic content (product strategy, PRDs, roadmap decisions) belongs in Notion, not the open-source repository.
- Illustrative code blocks (showing an idea rather than the real implementation) should carry a note that they are examples and the real code may differ; comparison or "why we didn't do X" code blocks do not need this note.

## Learned Workspace Facts
- `docs/src/content/docs/` is the single source of truth for architecture, workflow, and standards; other docs should point back there.
- `AGENTS.md` is the starting context file for AI agents working in this repository.
- Architecture follows a hexagonal pattern with domain crates (e.g. `common`, `core`, `inference`, `audio`, `api`, `peripherals`, `vision`), each documented under `docs/src/content/docs/architecture/modules/`.
- GitHub issues are the primary source of truth for user-facing tasks; Taskmaster (`.taskmaster/tasks/tasks.json`) is an optional, local helper to break down complex GitHub issues and is not pushed to git.
- The documentation site lives under `docs/` and is built with Astro Starlight; significant API or docs changes should be validated with the docs build.
- `.cursor/hooks/state/continual-learning-index.json` contains machine-specific absolute paths and must be gitignored; do not push it to the repository.
- Brainstorm files are stored in `archive_schon_bearbeitet/brainstorms/`, not under `docs/`.
- Strategic content (PRDs, product roadmap, market research) lives in Notion, not in the repository.

## Recommended Workflow
- Start from the GitHub issue: treat it as the single source of truth; restate its goal and acceptance criteria before touching code, and keep any local tools (including Taskmaster) consistent with it.
- Use planning mode for real complexity, not every change: if the work has 3+ steps, crosses domains, or the solution shape is fuzzy, spend a few minutes in planning mode to produce a short, ordered checklist; if the change fits in 1–2 sentences and ~1–2 files, skip formal planning and just implement.
- Use Taskmaster only for big/ongoing work: for multi-day, multi-PR, or multi-person issues, optionally mirror the GitHub issue into a small Taskmaster breakdown to track subtasks locally; for everything else, keep the workflow lightweight and work directly from the issue plus a simple plan.
