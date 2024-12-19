#!/bin/bash

# stop script on error
set -e

rm -f *.elf
rm -f *.bin
scripts/adj_version.py $1
cargo clean
DEFMT_LOG=off cargo build --release
DEFMT_LOG=off cargo strip --bin vario --release
scripts/pack.py
