rm -f src/dev_const.rs || true
rm -rf assets || true

cp -r ../larus_frontend_v2/assets assets
cp ../larus_frontend_v2/src/dev_const.rs src/dev_const.rs
