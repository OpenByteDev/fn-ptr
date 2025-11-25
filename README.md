# fn-ptr

[![CI](https://github.com/OpenByteDev/fn-ptr/actions/workflows/main.yml/badge.svg)](https://github.com/OpenByteDev/fn-ptr/actions/workflows/ci.yml) [![crates.io](https://img.shields.io/crates/v/fn-ptr.svg)](https://crates.io/crates/fn-ptr) [![Documentation](https://docs.rs/fn-ptr/badge.svg)](https://docs.rs/fn-ptr) [![dependency status](https://deps.rs/repo/github/openbytedev/fn-ptr/status.svg)](https://deps.rs/repo/github/openbytedev/fn-ptr) [![MIT](https://img.shields.io/crates/l/fn-ptr.svg)](https://github.com/OpenByteDev/fn-ptr/blob/master/LICENSE)

A small Rust crate that exposes a `FnPtr` trait that is implemented for all function pointer types up to arity 12, for multiple calling conventions (ABIs). The trait looks like this:

```rust
pub trait FnPtr {
    const ARITY: usize;
    const SAFE: bool;
    const EXTERN: bool;
    const ABI: Abi;

    type Args;
    type Output;
}
```

**Usage**

```rust
fn print_info<F: fn_ptr::FnPtr>() {
    println!("arity = {}", F::ARITY);
    println!("safe = {}", F::SAFE);
    println!("extern = {}", F::EXTERN);
    println!("ABI = {:?}", F::ABI);
}

fn main() {
    print_info::<fn(i32) -> i32>();
    // Example output (typical):
    // arity = 1
    // safe = true
    // extern = false
    // ABI = Rust
    // example(10) = 11
}
```
