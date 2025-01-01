#!/bin/sh

# stop script on error
set -e

cd device/air_avionics_ad57
cargo build -r
cd ../..

cd device/larus_frontend_v1
cargo build -r
cd ../..

cd device/larus_frontend_v2
cargo build -r
cd ../..

cd device/pc
cargo build -r
cd ../..

echo "finished"