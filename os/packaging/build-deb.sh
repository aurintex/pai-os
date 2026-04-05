#!/usr/bin/env bash
# Build the pai-engine Debian package.
#
# Usage:
#   ./os/packaging/build-deb.sh
#
# The .deb is output to os/packaging/dist/.
# To inject the real engine binary before packaging:
#   cp /path/to/pai-engine os/packaging/pai-engine/usr/local/bin/pai-engine
#   chmod 755 os/packaging/pai-engine/usr/local/bin/pai-engine
#   ./os/packaging/build-deb.sh
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SRC_DIR="$SCRIPT_DIR/pai-engine"
DIST_DIR="$SCRIPT_DIR/dist"
PKG_NAME="pai-engine"
PKG_VERSION="0.1.0-1"
PKG_ARCH="arm64"
DEB_FILE="${PKG_NAME}_${PKG_VERSION}_${PKG_ARCH}.deb"

mkdir -p "$DIST_DIR"

# Build a staging tree for dpkg-deb.
STAGING=$(mktemp -d)
trap 'rm -rf "$STAGING"' EXIT

# Copy package source files into staging root.
cp -r "$SRC_DIR/usr" "$STAGING/"
cp -r "$SRC_DIR/lib" "$STAGING/"

# Ensure binary is executable.
chmod 755 "$STAGING/usr/local/bin/$PKG_NAME"

# Create DEBIAN/ control directory in the staging tree.
mkdir -p "$STAGING/DEBIAN"

# Generate the DEBIAN/control file (canonical dpkg-deb format).
cat > "$STAGING/DEBIAN/control" <<EOF
Package: $PKG_NAME
Version: $PKG_VERSION
Architecture: $PKG_ARCH
Maintainer: paiOS Contributors <hello@paibox.ai>
Depends: systemd
Section: misc
Priority: optional
Description: paiEngine AI inference daemon
 The paiEngine daemon runs on-device AI inference for paiOS.
 It communicates over gRPC via Unix Domain Sockets, exposing
 local-only inference to on-device applications.
EOF

# Copy maintainer scripts.
cp "$SRC_DIR/debian/pai-engine.postinst" "$STAGING/DEBIAN/postinst"
chmod 755 "$STAGING/DEBIAN/postinst"

# Build the .deb.
dpkg-deb --build "$STAGING" "$DIST_DIR/$DEB_FILE"

echo ""
echo "Built: $DIST_DIR/$DEB_FILE"
dpkg-deb --info "$DIST_DIR/$DEB_FILE"
