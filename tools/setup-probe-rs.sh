#!/bin/sh
# Build probe-rs 0.32.0 with our STM32WL3x debug sequence into .tools/bin.
# The stock reset_and_halt fails on this part (AIRCR write never ACKed); see patches/.
set -e
cd "$(dirname "$0")/.."
[ -d third_party/probe-rs ] || git clone --depth 1 --branch v0.32.0 https://github.com/probe-rs/probe-rs.git third_party/probe-rs
cd third_party/probe-rs
if git apply --check ../../patches/probe-rs-0.32.0-stm32wl3-sequence.patch 2>/dev/null; then
    git apply ../../patches/probe-rs-0.32.0-stm32wl3-sequence.patch
fi
cargo install --path probe-rs-tools --locked --root ../../.tools
