#!/bin/bash -e


# Remove existing files in the destination directories
rm -f ./target/x86_64-unknown-linux-gnu/release/libanoncreds_uniffi.a || true
rm -f ./target/x86_64-unknown-linux-gnu/release/libanoncreds_uniffi.d || true
rm -f ./target/x86_64-unknown-linux-gnu/release/libanoncreds_uniffi.so || true
rm -f ./wrappers/kotlin/anoncreds/uniffi/anoncreds/anoncreds.kt || true

# Generate code 
cargo build --release --target x86_64-unknown-linux-gnu

# Generate the file anoncreds.kt in wrappers/kotlin/anoncreds/uniffi/anoncreds/anoncreds.kt
~/.cargo/bin/uniffi-bindgen generate src/anoncreds.udl --language kotlin -o ./wrappers/kotlin/anoncreds

# Move code to output-frameworks/anoncreds-jvm
rm -f ./output-frameworks/anoncreds-jvm/src/main/uniffi/anoncreds/anoncreds.kt
mv ./wrappers/kotlin/anoncreds/uniffi/anoncreds/anoncreds.kt ./output-frameworks/anoncreds-jvm/src/main/uniffi/anoncreds/anoncreds.kt

# make the jar
cd ./output-frameworks/anoncreds-jvm
./gradlew jar