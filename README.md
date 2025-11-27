# fn-ptr

[![CI](https://github.com/OpenByteDev/fn-ptr/actions/workflows/ci.yml/badge.svg)](https://github.com/OpenByteDev/fn-ptr/actions/workflows/ci.yml) [![crates.io](https://img.shields.io/crates/v/fn-ptr.svg)](https://crates.io/crates/fn-ptr) [![Documentation](https://docs.rs/fn-ptr/badge.svg)](https://docs.rs/fn-ptr) [![dependency status](https://deps.rs/repo/github/openbytedev/fn-ptr/status.svg)](https://deps.rs/repo/github/openbytedev/fn-ptr) [![MIT](https://img.shields.io/crates/l/fn-ptr.svg)](https://github.com/OpenByteDev/fn-ptr/blob/master/LICENSE)

`fn-ptr` is a small utility crate that provides a [`FnPtr`](https://docs.rs/fn-ptr/latest/fn_ptr/trait.FnPtr.html) trait, implemented for all function pointer types:
- `fn(T) -> U`
- `unsafe fn(T) -> U`
- `extern "C" fn(T)`
- `unsafe extern "sysv64" fn() -> i32`

The trait provides associated types and constants to introspect function pointer types at compile time.

## Features

### 1. Function Pointer Metadata

Every function pointer automatically implements [`FnPtr`](https://docs.rs/fn-ptr/latest/fn_ptr/trait.FnPtr.html).
Depending on the type, they also implement [`SafeFnPtr`](https://docs.rs/fn-ptr/latest/fn_ptr/trait.SafeFnPtr.html),, [`UnsafeFnPtr`](https://docs.rs/fn-ptr/latest/fn_ptr/trait.UnsafeFnPtr.html), and [`HasAbi<Abi>`](https://docs.rs/fn-ptr/latest/fn_ptr/trait.HasAbi.html).
With it you can inspect the type of function:

```rust
use fn_ptr::{FnPtr, Abi};

type F = extern "C" fn(i32, i32) -> i32;

assert_eq!(<F as FnPtr>::ARITY, 2);
assert_eq!(<F as FnPtr>::IS_SAFE, true);
assert_eq!(<F as FnPtr>::IS_EXTERN, true);
assert_eq!(<F as FnPtr>::ABI, Abi::C);
```

There are also some const helper functons to do so ergonomically.

```rust
const A: usize = fn_ptr::arity::<F>();         // 2
const SAFE: bool = fn_ptr::is_safe::<F>();     // true
const EXT: bool = fn_ptr::is_extern::<F>();    // true
const ABI: Abi = fn_ptr::abi::<F>();           // Abi::C
```

### 2. Toggle Function Pointer Safety

You can toggle the safety of a function pointer at the type level:

```rust
use fn_ptr::{make_safe, make_unsafe};

type U = unsafe extern "C" fn(i32);
type S = make_safe!(U);       // extern "C" fn(i32)

type S2 = extern "C" fn(i32);
type U2 = make_unsafe!(S2);   // unsafe extern "C" fn(i32)
```

Or at the instance level:

```rust
let safe_add: fn(i32, i32) -> i32 = |a, b| {a + b};
let unsafe_add: unsafe fn(i32, i32) -> i32 = safe_add.as_unsafe();
let safe_add2: fn(i32, i32) -> i32 = unsafe { unsafe_add.as_safe() };
```

### 3. Changing ABIs

You can also change the ABI of a function pointer at the type level:

```rust
use fn_ptr::{with_abi, Abi};

type F = extern "C" fn(i32) -> i32;

type G = with_abi!(Abi::Sysv64, F);
type H = with_abi!("C", extern "system" fn());
```

Or at the instance level:

```rust
let rust_add: fn(i32, i32) -> i32 = |a, b| {a + b};
// Safety: not actually safe!
let c_add: extern "C" fn(i32, i32) -> i32 = unsafe { rust_add.with_abi::<{Abi::C}>() };
```

Note that this does not change the underlying ABI and should be used with caution.
Also since arbitrary const generic types are unstable the code above only works on nightly, and requires converting
the ABI to an [`u8`] on stable:
```rust
// Always works
let c_add: extern "C" fn(i32, i32) -> i32 = unsafe { rust_add.with_abi::<{fn_ptr::abi::key(Abi::C)}>() };
// Works only on stable or beta
let c_add: extern "C" fn(i32, i32) -> i32 = unsafe { rust_add.with_abi::<{Abi::C as u8}>() };
```

## How It Works

To implement the traits for all function pointer types, there is a large [macro](https://github.com/OpenByteDev/fn-ptr/blob/master/src/impl.rs).
For the conversion macros the crate relies on two traits: [`WithAbi`](https://docs.rs/fn-ptr/latest/fn_ptr/trait.WithAbi.html) and [`WithSafety`](https://docs.rs/fn-ptr/latest/fn_ptr/trait.WithSafety.html) that can also be used directly:

```rust
use fn_ptr::{FnPtr, WithAbi, WithSafety, Abi};

type F = extern "C" fn(i32);
type G = <F as WithAbi<{Abi::Sysv64}>>::F;
type U = <F as WithSafety<{false}>>::F;
```

## License

Licensed under the MIT license, see [LICENSE](https://github.com/OpenByteDev/fn-ptr/blob/master/LICENSE) for details.
