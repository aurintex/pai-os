# Contributing to paiOS

Thank you for your interest in contributing to **paiOS**!

For complete contribution guidelines, including development setup, code style, CLA signing process, workflow, and more, please visit our **[Contributing Guide](https://docs.aurintex.com/guides/contributing/)**.

**Quick note:** First-time contributors need to sign our CLA (Contributor License Agreement). The CLAassistant bot will guide you through the process on your first Pull Request.

## Local checks (Rust)

CI runs `cargo fmt --all -- --check` inside `engine/`. Before pushing Rust changes, run the same command locally (or run `cargo fmt --all` in `engine/` to fix formatting).

Optional: install [pre-commit](https://pre-commit.com/) and run `pre-commit install` in the repo root to run that fmt check automatically when you commit (see `.pre-commit-config.yaml`). Using Cursor/VS Code with rust-analyzer, the committed `.vscode/settings.json` enables format on save for Rust files.

## Getting Help

If you have questions, found a bug, or need clarification, please reach out:

-   **Email**: [info@aurintex.com](mailto:info@aurintex.com)
-   **GitHub**: Open an [issue](https://github.com/aurintex/pai-os/issues) or a [discussion](https://github.com/aurintex/pai-os/discussions)
-   *(Discord, Slack, or Matrix coming soon!)*

