#!/bin/sh

# stop script on error
set -e

rm -f *.elf
rm -f *.bin

cd device/air_avionics_ad57
./make_image.sh
cd ../..
mv device/air_avionics_ad57/*.bin .
mv device/air_avionics_ad57/*.elf .

cd device/larus_frontend_v1
./make_image.sh -i
cd ../..
mv device/larus_frontend_v1/*.bin .
mv device/larus_frontend_v1/*.elf .

cd device/larus_frontend_v2
./make_image.sh -i
cd ../..
mv device/larus_frontend_v2/*.bin .
mv device/larus_frontend_v2/*.elf .

echo "finished"