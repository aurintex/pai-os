#!/usr/bin/env bash
# Wrapper for debos. Run from repository root.
# Examples:
#   ./os/build.sh
#   ./os/build.sh -v --disable-fakemachine
#   ./os/build.sh -t machine:radxa-rock5c --dry-run
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
RECIPE="$SCRIPT_DIR/image.yaml"

cd "$REPO_ROOT"

# KVM backend needs to read the host kernel from /boot (often root-only). Re-exec with sudo once.
if [ -z "${DEBOS_HAVE_SUDO}" ] && [ ! -r "/boot/vmlinuz-$(uname -r)" ] && [ -e /dev/kvm ] && ! grep -q -- '--disable-fakemachine' <<< " $* "; then
  export DEBOS_HAVE_SUDO=1
  exec sudo -E env DEBOS_HAVE_SUDO=1 "$SCRIPT_DIR/build.sh" "$@"
fi

MACHINE="${MACHINE:-radxa-rock5c}"
exec debos -t "machine:$MACHINE" "$@" "$RECIPE"
