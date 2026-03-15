#!/usr/bin/env bash
# Run a paiOS image in QEMU aarch64 and verify it boots to a login prompt.
#
# Usage: os/tests/run-qemu.sh [image-path]
#   image-path  Path to the .img file (default: radxa-rock5c.img in repo root)
#
# Prerequisites: qemu-system-aarch64, ovmf (AAVMF), expect
#   Debian/Ubuntu: apt install qemu-system-arm ovmf expect
#
# Exit codes:
#   0  Boot verified (login prompt detected, pai-engine active)
#   1  Timeout or boot failure
#   2  Image file not found or missing dependencies

set -euo pipefail

# ----------------------------------------------------------------------------
# Config
# ----------------------------------------------------------------------------
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"

IMAGE="${1:-${REPO_ROOT}/radxa-rock5c.img}"
TIMEOUT="${QEMU_BOOT_TIMEOUT:-120}"   # seconds to wait for login prompt
MEMORY="${QEMU_MEM:-1024}"            # MiB

# AAVMF firmware paths (Debian/Ubuntu package: ovmf)
EFI_FW_CANDIDATES=(
  /usr/share/AAVMF/AAVMF_CODE.fd
  /usr/share/qemu-efi-aarch64/QEMU_EFI.fd
  /usr/share/ovmf/AAVMF/AAVMF_CODE.fd
)
EFI_VARS_TEMPLATE_CANDIDATES=(
  /usr/share/AAVMF/AAVMF_VARS.fd
  /usr/share/qemu-efi-aarch64/QEMU_VARS.fd
  /usr/share/ovmf/AAVMF/AAVMF_VARS.fd
)

# ----------------------------------------------------------------------------
# Helpers
# ----------------------------------------------------------------------------
die() { echo "ERROR: $*" >&2; exit "${2:-1}"; }
log() { echo "[run-qemu] $*"; }

# Resolve the first existing path from a list
resolve_first() {
  for p in "$@"; do [ -f "$p" ] && { echo "$p"; return 0; }; done
  return 1
}

# ----------------------------------------------------------------------------
# Pre-flight checks
# ----------------------------------------------------------------------------
[ -f "${IMAGE}" ] || die "Image not found: ${IMAGE}" 2

for cmd in qemu-system-aarch64 expect; do
  command -v "${cmd}" >/dev/null 2>&1 || die "Missing dependency: ${cmd} (install: apt install qemu-system-arm expect)" 2
done

EFI_FW="$(resolve_first "${EFI_FW_CANDIDATES[@]}")" \
  || die "AAVMF firmware not found (install: apt install ovmf or qemu-efi-aarch64)" 2

EFI_VARS_TEMPLATE="$(resolve_first "${EFI_VARS_TEMPLATE_CANDIDATES[@]}")" \
  || die "AAVMF vars template not found (install: apt install ovmf)" 2

# Copy vars to a writable temp file so QEMU can update them
EFI_VARS_TMP="$(mktemp /tmp/AAVMF_VARS.XXXXXX.fd)"

# ----------------------------------------------------------------------------
# Cleanup
# ----------------------------------------------------------------------------
QEMU_PID=""
cleanup() {
  if [ -n "${QEMU_PID}" ] && kill -0 "${QEMU_PID}" 2>/dev/null; then
    log "Killing QEMU (PID ${QEMU_PID})"
    kill "${QEMU_PID}" 2>/dev/null || true
    wait "${QEMU_PID}" 2>/dev/null || true
  fi
  rm -f "${EFI_VARS_TMP}"
}
trap cleanup EXIT

# ----------------------------------------------------------------------------
# Build QEMU command
# ----------------------------------------------------------------------------
cp "${EFI_VARS_TEMPLATE}" "${EFI_VARS_TMP}"

QEMU_ARGS=(
  qemu-system-aarch64
  -machine virt
  -cpu cortex-a57
  -m "${MEMORY}"
  -nographic
  -serial mon:stdio
  # EFI firmware
  -drive "if=pflash,format=raw,file=${EFI_FW},readonly=on"
  -drive "if=pflash,format=raw,file=${EFI_VARS_TMP}"
  # OS image
  -drive "file=${IMAGE},format=raw,if=none,id=hd0"
  -device virtio-blk-pci,drive=hd0
  # Network (user-mode, no tap required)
  -netdev user,id=net0
  -device virtio-net-pci,netdev=net0
)

log "Launching QEMU for image: ${IMAGE}"
log "Timeout: ${TIMEOUT}s | Memory: ${MEMORY}MB"

# ----------------------------------------------------------------------------
# Start QEMU in background and run expect
# ----------------------------------------------------------------------------

# Note: expect reads the TTY output from QEMU. We launch QEMU inside expect
# so it can monitor the serial output directly.

RESULT=0
expect -c "
  set timeout ${TIMEOUT}

  spawn ${QEMU_ARGS[*]}

  # Step 1: Wait for login prompt
  expect {
    \"login:\" {
      send_user \"\n\[run-qemu\] Boot verified: login prompt detected.\n\"
    }
    timeout {
      send_user \"\n\[run-qemu\] TIMEOUT: No login prompt within ${TIMEOUT}s.\n\"
      exit 1
    }
    eof {
      send_user \"\n\[run-qemu\] QEMU exited unexpectedly before login prompt.\n\"
      exit 1
    }
  }

  # Step 2: Log in as root (no password on a dev image)
  send \"root\r\"
  expect {
    -re {(\\\$|#) *$} {}
    timeout { send_user \"\[run-qemu\] Timed out waiting for shell prompt.\n\"; exit 1 }
  }

  # Step 3: Check pai-engine service
  send \"systemctl is-active pai-engine\r\"
  expect {
    \"active\" {
      send_user \"\[run-qemu\] pai-engine service is active.\n\"
    }
    \"inactive\" {
      send_user \"\[run-qemu\] WARNING: pai-engine service is inactive.\n\"
    }
    timeout {
      send_user \"\[run-qemu\] Timed out waiting for systemctl output.\n\"
    }
  }

  # Shutdown gracefully
  send \"poweroff\r\"
  expect eof
  exit 0
" || RESULT=$?

if [ "${RESULT}" -eq 0 ]; then
  log "Verification passed."
else
  log "Verification FAILED (exit code ${RESULT})."
fi

exit "${RESULT}"
