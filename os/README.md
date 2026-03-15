# paiOS Build System

This directory contains Debos recipes and RAUC configurations for building paiOS images.

**CI:** The [OS Build](https://github.com/aurintex/pai-os/actions/workflows/os-build.yml) workflow builds the image on tag push, manual trigger, or daily schedule. Artifacts are gzipped; after download run `gunzip radxa-rock5c.img.gz`. CI uses `--disable-fakemachine` (no KVM on hosted runners).

> **📚 Documentation:** For complete build instructions and OS development guide, see [docs.aurintex.com](https://docs.aurintex.com/).
