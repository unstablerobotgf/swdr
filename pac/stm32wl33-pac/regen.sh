#!/bin/sh
# Regenerate the PAC from ST's SVD (STM32WL3x_DFP 1.2.0).
# Patch: SVD claims nvicPrioBits=4, but Cortex-M0+ implements 2.
set -e
cd "$(dirname "$0")"
sed 's|<nvicPrioBits>4</nvicPrioBits>|<nvicPrioBits>2</nvicPrioBits>|' STM32WL33.svd > patched.svd
rm -rf src gen && mkdir gen
svd2rust -i patched.svd --target cortex-m -o gen
form -i gen/lib.rs -o src/
mv gen/build.rs gen/device.x . && rm -rf gen patched.svd
rustfmt --edition 2021 src/lib.rs
