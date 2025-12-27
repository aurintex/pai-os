# PAI (Personal AI)

**The Local-First Intelligence Engine.**

> 🚧 **Status:** Phase 0 (Architecture & Foundation)
>
> **Note:** This is a preliminary draft to establish the new monorepo structure. Comprehensive documentation, license details, and usage guides will follow shortly.

## Vision

**PAI** is an open-source, local-first AI ecosystem designed to act as your privacy-conscious second brain. It processes voice, context, and intelligence directly on trusted hardware, giving you full control over your data.

**The Product: paiOS**
The core of this repository is **paiOS**, an embedded operating system built to run strictly local inference pipelines.

**Current Implementation:**
We are currently building the **PAI USB Accelerator** (V1), a device that brings this intelligence to any computer via a simple USB connection.

**Future Roadmap:**
The technology developed here lays the foundation for the **PAI Companion**, a standalone AI wearable.

## Repository Structure

This monorepo contains the entire software stack:

- **`engine/`**: The Rust-based neural orchestrator (The "Brain"). It manages audio streams, NPU inference, and host communication via HID.
- **`os/`**: Reproducible embedded Linux OS (Debian-based) built with Debos.
- **`apps/`**: Modular extensions and reference implementations for the PAI ecosystem.
- **`docs/`**: Source for the documentation site (built with Starlight).

## Development Status

We are currently executing the **"Great Pivot"**: Transitioning from our internal Proof-of-Concept to this clean, production-ready monorepo architecture.

* **Architecture:** Clean Architecture (Rust)
* **License:** AGPL-3.0 (Planned)
* **Documentation:** Available at [docs.aurintex.com](https://docs.aurintex.com) (source in `docs/`)

## Getting Help

If you have questions, found a bug, or need clarification, please reach out:

- **Email**: [info@aurintex.com](mailto:info@aurintex.com)
- **GitHub**: Open an [issue](https://github.com/aurintex/pai-os/issues) or a [discussion](https://github.com/aurintex/pai-os/discussions)
- *(Discord, Slack, or Matrix coming soon!)*
