rm -f src/dev_const.rs || true
rm -rf assets || true

cp -r ../air_avionics_ad57/assets assets
cp ../air_avionics_ad57/src/dev_const.rs src/dev_const.rs
