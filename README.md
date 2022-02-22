# `linear-malloc`

[![Latest version](https://badgen.net/crates/v/linear-malloc)](https://crates.io/crates/linear-malloc)
[![Documentation](https://docs.rs/linear-malloc/badge.svg)](https://docs.rs/linear-malloc/)

# DISCLAIMER! This is a quick hack I did to test things. Don't use!

An ultra simple single-threaded linear allocator.

Useful to make the output of applications running under Cachegrind/Callgrind
more stable (since it doesn't try to do the clever optimizations that the
system allocator does).

## Usage

Linux:
```sh
cargo build
LD_PRELOAD=./target/debug/liblinear_malloc.so your-binary
```

macOS:
```sh
cargo build
DYLD_INSERT_LIBRARIES=./target/debug/liblinear_malloc.dylib DYLD_FORCE_FLAT_NAMESPACE=1 your-binary
```

## Acknowledgements

Got some useful info about allocation strategies from [@mtrebi's `memory-allocators`](https://github.com/mtrebi/memory-allocators), and [@ezrosent's "Allocators in Rust"](https://github.com/ezrosent/allocators-rs) was a good source to help set up the basic framework.
