#!/bin/bash

# stop script on error
set -e

scripts/adj_version.py
cargo clean
DEFMT_LOG=off cargo build --release
scripts/pack.py
