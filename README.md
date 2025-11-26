# fn-ptr

[![CI](https://github.com/OpenByteDev/fn-ptr/actions/workflows/ci.yml/badge.svg)](https://github.com/OpenByteDev/fn-ptr/actions/workflows/ci.yml) [![crates.io](https://img.shields.io/crates/v/fn-ptr.svg)](https://crates.io/crates/fn-ptr) [![Documentation](https://docs.rs/fn-ptr/badge.svg)](https://docs.rs/fn-ptr) [![dependency status](https://deps.rs/repo/github/openbytedev/fn-ptr/status.svg)](https://deps.rs/repo/github/openbytedev/fn-ptr) [![MIT](https://img.shields.io/crates/l/fn-ptr.svg)](https://github.com/OpenByteDev/fn-ptr/blob/master/LICENSE)

`fn-ptr` is a small utility crate that provides a [`FnPtr`](https://docs.rs/fn-ptr/latest/fn_ptr/trait.FnPtr.html) trait, implemented for all function pointer types:
- `fn(T) -> U`
- `unsafe fn(T) -> U`
- `extern "C" fn(T)`
- `unsafe extern "sysv64" fn() -> i32`

The trait provides associated types and constants to introspect function pointer types at compile time.
```rust
pub trait FnPtr {
    /// The argument types as a tuple.
    type Args;

    /// The return type.
    type Output;

    /// The function's arity (number of arguments).
    const ARITY: usize;

    /// Whether the function pointer is safe (`fn`) or unsafe (`unsafe fn`).
    const IS_SAFE: bool;

    /// Whether the function pointer uses an extern calling convention.
    const IS_EXTERN: bool;

    /// The ABI associated with this function pointer.
    const ABI: Abi;
}
```

## Features

1. Function-pointer metadata

Every function pointer automatically implements [`FnPtr`](https://docs.rs/fn-ptr/latest/fn_ptr/trait.FnPtr.html). Depending on the type, it may also implement [`SafeFnPtr`](https://docs.rs/fn-ptr/latest/fn_ptr/trait.SafeFnPtr.html), [`UnsafeFnPtr`](https://docs.rs/fn-ptr/latest/fn_ptr/trait.UnsafeFnPtr.html), and [`HasAbi<Abi>`](https://docs.rs/fn-ptr/latest/fn_ptr/trait.HasAbi.html).

```rust
use fn_ptr::{FnPtr, Abi};

type F = extern "C" fn(i32, i32) -> i32;

assert_eq!(<F as FnPtr>::ARITY, 2);
assert_eq!(<F as FnPtr>::IS_SAFE, true);
assert_eq!(<F as FnPtr>::IS_EXTERN, true);
assert_eq!(<F as FnPtr>::ABI, Abi::C);
```

Const helper functions are also provided:
```rust
const A: usize = fn_ptr::arity::<F>();         // 2
const SAFE: bool = fn_ptr::is_safe::<F>();     // true
const EXT: bool = fn_ptr::is_extern::<F>();    // true
const ABI: Abi = fn_ptr::abi::<F>();           // Abi::C
```

2. Changing ABIs at the type level

You can change the ABI of a function pointer type using macros:
```rust
use fn_ptr::{with_abi, Abi};

type F = extern "C" fn(i32) -> i32;

type G = with_abi!(Abi::Sysv64, F);
type H = with_abi!("C", extern "system" fn());
```

3. Toggle function pointer safety

Macros are provided to make function pointers safe or unsafe:
```rust
use fn_ptr::{make_safe, make_unsafe};

type U = unsafe extern "C" fn(i32);
type S = make_safe!(U);       // extern "C" fn(i32)

type S2 = extern "C" fn(i32);
type U2 = make_unsafe!(S2);   // unsafe extern "C" fn(i32)
```

## How it works under the hood
All macros rely on type-level traits [`WithAbi`](https://docs.rs/fn-ptr/latest/fn_ptr/trait.WithAbi.html) and [`WithSafety`](https://docs.rs/fn-ptr/latest/fn_ptr/trait.WithSafety.html). Each trait exposes an associated type representing the transformed function pointer. You can use these traits directly for const generics or explicit type transformations:

```rust
use fn_ptr::{FnPtr, WithAbi, WithSafety, Abi};

type F = extern "C" fn(i32);
type G = <F as WithAbi<{Abi::Sysv64}, F>>::F;
type U = <F as WithSafety<{false}, F>>::F;
```

## License
Licensed under the MIT license, see [LICENSE](./LICENSE) for details.
