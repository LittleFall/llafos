# llafos

## build

build for some bare metal environment
```shell
rustup target add thumbv7em-none-eabihf

cargo build --target thumbv7em-none-eabihf
```

build for host system
```shell
# Linux
cargo rustc -- -C link-arg=-nostartfiles
# Windows
cargo rustc -- -C link-args="/ENTRY:_start /SUBSYSTEM:console"
# macOS
cargo rustc -- -C link-args="-e __start -static -nostartfiles"
```

## thanks

thanks to https://os.phil-opp.com/freestanding-rust-binary/#summary
