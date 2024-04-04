# open62541-ffi
Rust FFI bindings for [open62541](https://github.com/open62541/open62541) library.

A follow-up to [open62541-sys](https://github.com/miehe-dup/open62541-sys) work.

**Note:** this repository is unmaintained; there are more up-to-date crates available, e.g. https://github.com/HMIProject/open62541-sys

## Building

```bash
git clone git@github.com:konradsz/open62541-ffi.git
cd open62541-ffi
git submodule init
git submodule update

# run one of the binaries, e.g.:
cargo run --release --bin tutorial_server_firststeps
```
