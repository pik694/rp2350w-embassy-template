#!/usr/bin/env bash
set -ueo pipefail

REPO_ROOT=$(git rev-parse --show-toplevel)

probe-rs download "$REPO_ROOT/assets/cyw43-firmware/43439A0.bin" \
  --binary-format bin --chip RP235x --base-address 0x10100000 && \
probe-rs download "$REPO_ROOT/assets/cyw43-firmware/43439A0_clm.bin" \
  --binary-format bin --chip RP235x --base-address 0x10140000

