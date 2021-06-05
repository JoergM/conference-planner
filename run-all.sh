export RUST_BACKTRACE=1
#export FAILURE_RATE=10
#export RANDOM_DELAY_MAX=2000
cargo run -p speaker &
cargo run -p session &
cargo run -p schedule &
cargo run -p frontend &
wait
