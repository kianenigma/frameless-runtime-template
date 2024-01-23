RUST_BACKTRACE=1 RUST_LOG=grading=debug,frameless=debug WASM_FILE=$1 cargo \
  nextest \
  run \
  --release \
  -p runtime \
  --features pre-grade \
  --failure-output immediate \
  --success-output immediate \
  --no-fail-fast

cp ./target/nextest/default/result.xml ./result.xml
cat result.xml > result.json
