# llafos

A practice operating system build by rust.

## command
These commands requires `qemu` which should be installed first, for example, `brew install qemu` in macos.

build and run in bare metal environment
```shell
cargo run
```

run test
```shell
cargo test
```

## features

- [x] println, print message to console.
- [x] serial_println, print message to host machine (if have).
- [x] test framework, include unit test and integrated test.

## thanks

Thanks to https://os.phil-opp.com/  , the great tutorial. 

