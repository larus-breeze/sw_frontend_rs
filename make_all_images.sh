#!/bin/sh

# stop script on error
set -e

rm -rf output
mkdir output

cd device/air_avionics_ad57
./make_image.sh $1
cd ../..
mv device/air_avionics_ad57/*.bin output
mv device/air_avionics_ad57/*.elf output

cd device/larus_frontend_v1
./make_image.sh -i
cd ../..
mv device/larus_frontend_v1/*.bin output
mv device/larus_frontend_v1/*.elf output

cd device/larus_frontend_v2
./make_image.sh -i
cd ../..
mv device/larus_frontend_v2/*.bin output
mv device/larus_frontend_v2/*.elf output

cd device/sim
./make_image.sh -i
cd ../..

cd core/tools
cargo run --bin extract_polar_store -r 
cargo run --bin extract_menus -r 
cd ../..

cd doc
./make_manual.py -i
cd ..
mv doc/*.pdf output

echo "finished"