#!/bin/bash

# stop script on error
set -e

scripts/adj_version.py $1
cargo clean
DEFMT_LOG=off cargo build --release
cargo strip --bin vario --release
scripts/pack.py
