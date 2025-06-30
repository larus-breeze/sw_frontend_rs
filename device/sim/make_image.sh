#!/bin/bash

# stop script on error
set -e

scripts/adj_version.py $1
cargo build
