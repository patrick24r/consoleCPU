# console_cpu
Using the STM32H743 as a game console CPU in Rust!

Build binary with
```
cargo build --release --bin console_cpu
cargo objcopy --release --bin console_cpu -- -O binary target/thumbv7em-none-eabihf/release/console_cpu.bin
```