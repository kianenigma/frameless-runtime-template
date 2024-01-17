RUST_LOG=grading=debug WASM_FILE=$1 cargo \
  nextest \
  run \
  --release \
  --features pre-grade \
  -p runtime \
  --failure-output immediate \
  --success-output immediate \
  --no-fail-fast

cp ./target/nextest/default/result.xml ./result.xml
cat result.xml | jtm  > result.json
