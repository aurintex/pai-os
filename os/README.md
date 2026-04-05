# paiOS Build System

This directory contains Debos recipes for building paiOS images.

For build instructions and OS development details, see the **official documentation**:

- Main docs: https://docs.aurintex.com/
- OS & Infrastructure: `/architecture/operating-system/` in the docs site.

## Packaging

The paiEngine binary is installed into the image as a Debian package, not via file overlay.

Before running debos, produce the `.deb`:

```bash
# Optional: replace the placeholder with the real compiled binary first.
cp /path/to/pai-engine os/packaging/pai-engine/usr/local/bin/pai-engine

# Build the package.
bash os/packaging/build-deb.sh
```

The package is output to `os/packaging/dist/` (gitignored) and the debos recipe copies it into the image during the build.
