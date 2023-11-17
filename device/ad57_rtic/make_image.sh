#!/bin/bash

assets/adj_version.py $1
cargo clean
DEFMT_LOG=off cargo build --release
assets/pack.py
