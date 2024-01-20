# Frameless Assignment

Run `cargo doc -p runtime --open --document-private-items`, and start hacking 🚀

## Grading Notes

Building the archived nextest stuff

```
cargo nextest archive --release -p runtime --features pre-grade  --archive-file grading-pre.tar.zst

# Grading with it
RUST_BACKTRACE=1 \
RUST_LOG=grading=debug,frameless=debug \
WASM_FILE=$1 \
cargo \
  nextest \
  run \
  --archive-file ./grading-pre.tar.zst \
  --failure-output immediate \
  --success-output immediate \
  --no-fail-fast

cp ./target/nextest/default/result.xml ./result.xml
cat result.xml | jtm  > result.json


```
