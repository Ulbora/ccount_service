cargo clean

# cargo build --release --target=x86_64-unknown-linux-musl
# cp target/x86_64-unknown-linux-musl/release/ccount_service server

cargo build --release 
cp target/release/ccount_service server


# cargo build --target=x86_64-unknown-linux-musl
# cp target/x86_64-unknown-linux-musl/debug/ccount_service server