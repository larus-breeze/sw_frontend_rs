rm -f src/dev_const.rs || true
rm -rf assets || true

cp -r ../larus_frontend_v1/assets assets
cp ../larus_frontend_v1/src/dev_const.rs src/dev_const.rs
