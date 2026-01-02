<div align="center">

![PAI Hero Banner](docs/src/assets/pai_hero_banner.png)

# PAI (Personal AI)

**The Open Source OS for Personal Intelligence.**

[![License: AGPL v3](https://img.shields.io/badge/License-AGPL_v3-blue.svg)](LICENSE)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE-MIT)
[![Build Status](https://img.shields.io/github/actions/workflow/status/aurintex/pai-os/ci.yml?label=build)](https://github.com/aurintex/pai-os/actions)
[![Docs](https://img.shields.io/badge/docs-starlight-orange.svg)](https://docs.aurintex.com/)
[![Discord](https://img.shields.io/badge/Discord-Join%20Us-7289da?logo=discord&logoColor=white)](https://discord.gg/7uSGGpRgK)
[![Twitter](https://img.shields.io/twitter/follow/aurintex?style=social)](https://twitter.com/aurintex)

[Documentation](https://docs.aurintex.com/) • [Website](https://aurintex.com) • [Contributing](CONTRIBUTING.md)

</div>

---

## 🔮 The Vision

**PAI** is more than just an operating system; it is the foundation for a new category of **Personal AI Hardware**.

Our mission is to build a privacy-first "Second Brain" that lives with you, understands your context, and operates independently of the cloud. We believe your intelligence should belong to you-running on trusted hardware, physically under your control.

To achieve this, we are building a unified, modular OS that powers multiple form factors, starting with a USB Accelerator and evolving into a standalone Wearable.

## ⚡️ Form Factors

PAI OS is designed to run on diverse hardware, bringing local intelligence to where you need it most.

### 1. PAI Accelerator (Your Local AI Server)
*Currently in Development*

A powerful USB device (based on the **Radxa Rock 5C**) that acts as a **private AI backend** for your existing tools.

**The Idea:** Plug it in, and any app that supports [Ollama](https://ollama.com) (or a compatible API) can use PAI for inference. Your data stays on the device – no cloud required.

**Compatible Apps (Examples):**
*   **Coding:** VS Code (Continue, CodeGPT), Zed, Neovim
*   **Writing:** Obsidian, LibreOffice, OnlyOffice
*   **Chat:** Open WebUI, Lobe Chat

**Key Features:**
*   🔌 **Standard API:** Ollama-compatible endpoint for seamless integration.
*   🔒 **Air-Gapped Privacy:** Inference happens on the USB device, never in the cloud.
*   ⚡ **Hardware Acceleration:** NPU/GPU powered for real-time responses.

### 2. PAI Companion (The Wearable)
*The North Star*

The evolution of the Accelerator into a standalone, battery-powered AI wearable. It includes **all capabilities of the Accelerator** (local AI server, Ollama-compatible API) *plus* integrated sensors for contextual awareness.

Equipped with a camera and microphone array, it passively captures context to answer questions like *"Where did I leave my keys?"* or *"What was the action item from that meeting?"*.

**Everything the Accelerator can do, plus:**
*   📷 **Vision:** Onboard camera for multimodal AI (Vision-Language models).
*   🎙️ **Always-On Audio:** Continuous, private voice capture.
*   🔋 **Standalone:** No phone or PC required – works independently.
*   🧠 **Contextual Memory:** "What did I discuss with Sarah last Tuesday?"

---

## 🆚 Why PAI?

| Feature | ☁️ Cloud AI (SaaS) | 🔐 PAI (Local) |
| :--- | :--- | :--- |
| **Privacy** | Your data trains their models. | Your data never leaves the device. |
| **Ownership** | Renting intelligence (Subscription). | Owning the intelligence (One-off). |
| **Latency** | Network dependency (>500ms). | Real-time Neural Engine (<100ms). |
| **Usage** | Pay per token / Rate limits. | **Unlimited.** Use it as much as you want. |
| **Trust** | "Trust us, we're compliant." | "Verify it yourself." (Open Source). |

---

## 🗺️ Roadmap

We are currently in **Phase 0 (Foundation)**.

*   **Phase 0: Foundation:** Groundwork, Architecture Definitions, CI/CD, and Documentation infrastructure.
*   **Phase 1: The Accelerator:** Launching the USB Hardware in **Developer** (Radxa Rock 5C) and **Professional** (Aluminum Case) editions.
*   **Phase 2: The Companion:** Evolution into a standalone, multimodal AI Wearable.

👉 [**View Detailed Roadmap**](https://docs.aurintex.com/roadmap/)

---

## �️ Architecture & Tech Stack

PAI OS is built on a **Clean Architecture** principle to ensure that the core logic ("The Brain") is completely decoupled from the specific hardware it runs on. This allows `paiOS` to run on a USB Stick today and a Wearable tomorrow.

### The Stack

*   **Language:** **Rust** (Safety, Performance, Concurrency)
*   **Build System:** **Debos** (Reproducible, immutable Debian-based OS images)
*   **Documentation:** **Starlight** (Modern, accessible, version-controlled)

### System Design

```mermaid
graph TD
    subgraph "Device (Hardware Agnostic)"
        HAL["HAL (Hardware Abstraction)"] -->|Stream| Orch[Orchestrator]
        Orch -->|Inference| Engine["Core Engine (Rust)"]
        Engine -->|Action| Interface[HID / Voice / UI]
    end
    Interface -.->|IO| Host[External World]
```

*   **Hardware Abstraction:** The Engine doesn't know *what* it runs on, only that it has sensors and outputs.
*   **Security by Design:** Because we control the OS, all hardware access (camera, microphone) goes through the Engine. Apps cannot bypass this layer – ensuring only *you* decide what gets recorded.
*   **Single Source of Truth:** For deep dives into the Clean Architecture layers, IPC (gRPC), and decision records, please consult the **[Architecture Documentation](https://docs.aurintex.com/architecture/)**.

## 🏁 Quick Start for Developers

This is a monorepo containing the full stack. Dependencies include Rust (latest stable) and `debos`.

### 1. Build the Engine
```bash
cd engine
cargo build --release
# Run tests
cargo test
```

### 2. Build the OS Image
*Requires Linux (Debian/Ubuntu recommended)*
```bash
cd os
# Build the image (defaults to Accelerator profile)
sudo debos image.yaml
```

> **Note:** For flashing instructions, hardware compatibility lists (Radxa Rock 5C), and detailed architecture guides, please consult the [Documentation](https://docs.aurintex.com/).

## 🤝 Contributing

We are building the user-centric, privacy-first alternative to Big Tech AI.

1.  Check the [Contributing Guide](CONTRIBUTING.md).
2.  Pick an issue from [GitHub Issues](https://github.com/aurintex/pai-os/issues).
3.  **Sign your commits** (`git commit -s`) - we use DCO.

## 📜 License

*   **Engine & OS:** [GNU AGPL v3](LICENSE) (Viral Open Source)
*   **Apps & SDK:** [MIT](LICENSE-MIT) (Ecosystem Friendly)
