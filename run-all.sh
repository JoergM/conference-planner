export RUST_BACKTRACE=1
cargo run -p speaker &
cargo run -p session &
cargo run -p schedule &
cargo run -p frontend &
wait
