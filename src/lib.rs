#![cfg_attr(nightly_build, fn_ptr_trait)]
#![cfg_attr(has_abi_vectorcall, feature(abi_vectorcall))]
#![warn(clippy::pedantic, missing_docs)]
#![no_std]

//! `fn-ptr` is a small utility crate that provides a [`FnPtr`] trait, implemented for all function pointer types:
//! - `fn(T) -> U`
//! - `unsafe fn(T) -> U`
//! - `extern "C" fn(T)`
//! - `unsafe extern "sysv64" fn() -> i32`
//!
//! The trait provides associated types and constants to introspect function pointer types at compile time.
//!
//! ## Features
//!
//! ### 1. Function Pointer Metadata
//!
//! Every function pointer automatically implements [`FnPtr`].
//! Depending on the type, they also implement [`SafeFnPtr`], [`UnsafeFnPtr`], and [`HasAbi<Abi>`].
//! With it you can inspect the type of function:
//!
//! ```rust
//! use fn_ptr::{FnPtr, Abi};
//!
//! type F = extern "C" fn(i32, i32) -> i32;
//!
//! assert_eq!(<F as FnPtr>::ARITY, 2);
//! assert_eq!(<F as FnPtr>::IS_SAFE, true);
//! assert_eq!(<F as FnPtr>::IS_EXTERN, true);
//! assert_eq!(<F as FnPtr>::ABI, Abi::C { unwind: false });
//! ```
//!
//! There are also some const helper functons to do so ergonomically.
//!
//! ```rust
//! # type F = extern "C" fn(i32, i32) -> i32;
//! # use fn_ptr::{FnPtr, Abi};
//! const A: usize = fn_ptr::arity::<F>();         // 2
//! const SAFE: bool = fn_ptr::is_safe::<F>();     // true
//! const EXT: bool = fn_ptr::is_extern::<F>();    // true
//! const ABI: Abi = fn_ptr::abi::<F>();           // Abi::C
//! ```
//!
//! ### 2. Toggle Function Pointer Safety
//!
//! You can toggle the safety of a function pointer at the type level:
//!
//! ```rust
//! use fn_ptr::{make_safe, make_unsafe};
//!
//! type U = unsafe extern "C" fn(i32);
//! type S = make_safe!(U);       // extern "C" fn(i32)
//!
//! type S2 = extern "C" fn(i32);
//! type U2 = make_unsafe!(S2);   // unsafe extern "C" fn(i32)
//! ```
//!
//! Or at the instance level:
//!
//! ```rust
//! # use fn_ptr::FnPtr;
//! let safe_add: fn(i32, i32) -> i32 = |a, b| {a + b};
//! let unsafe_add: unsafe fn(i32, i32) -> i32 = safe_add.as_unsafe();
//! let safe_add2: fn(i32, i32) -> i32 = unsafe { unsafe_add.as_safe() };
//! # assert_eq!(safe_add.addr(), safe_add2.addr());
//! ```
//!
//! ### 3. Changing ABIs
//!
//! You can also change the ABI of a function pointer at the type level:
//!
//! ```rust
//! # #[cfg(nightly_build)] {
//! use fn_ptr::{with_abi, Abi};
//!
//! type F = extern "C" fn(i32) -> i32;
//!
//! type G = with_abi!(Abi::Sysv64, F);
//! type H = with_abi!("C", extern "system" fn());
//! # }
//! ```
//!
//! Or at the instance level:
//!
//! ```rust
//! use fn_ptr::{FnPtr, abi};
//! let rust_add: fn(i32, i32) -> i32 = |a, b| {a + b};
//! // Safety: not actually safe!
//! let c_add: extern "C" fn(i32, i32) -> i32 = unsafe { rust_add.with_abi::<abi!("C")>() };
//! # assert_eq!(rust_add.addr(), c_add.addr());
//! ```
//!
//! Note that this does not change the underlying ABI and should be used with caution.
//!
//! ## How It Works
//!
//! To implement the traits for all function pointer types, there is a large [macro](https://github.com/OpenByteDev/fn-ptr/blob/master/src/impl.rs).
//! For the conversion macros the crate relies on two traits: [`WithAbi`] and [`WithSafety`] that can also be used directly:
//!
//! ```rust
//! # #[cfg(nightly_build)] {
//! use fn_ptr::{FnPtr, WithAbi, WithSafety, Abi};
//!
//! type F = extern "C" fn(i32);
//! type G = <F as WithAbi<{Abi::Sysv64}>>::F;
//! type U = <F as WithSafety<{false}>>::F;
//! # }
//! ```
//!
//! ## License
//!
//! Licensed under the MIT license, see [LICENSE](https://github.com/OpenByteDev/fn-ptr/blob/master/LICENSE) for details.

/// Module containing the Abi abstraction.
pub mod abi;
pub use abi::Abi;

mod r#impl;

/// Module containing all marker types and traits.
pub mod marker;

/// Prelude for this crate.
pub mod prelude;

mod base;
pub use base::*;

mod conversion;
pub use conversion::*;

/// Returns the number of arguments of a function pointer type.
#[must_use]
pub const fn arity<F: FnPtr>() -> usize {
    F::ARITY
}

/// Returns `true` for safe function pointers (`fn`).
#[must_use]
pub const fn is_safe<F: FnPtr>() -> bool {
    F::IS_SAFE
}

/// Returns `true` for unsafe function pointers (`unsafe fn`).
#[must_use]
pub const fn is_unsafe<F: FnPtr>() -> bool {
    !is_safe::<F>()
}

/// Returns `true` if the function pointer uses an extern ABI.
#[must_use]
pub const fn is_extern<F: FnPtr>() -> bool {
    F::IS_EXTERN
}

/// Returns the ABI of the function pointer.
#[must_use]
pub const fn abi<F: FnPtr>() -> Abi {
    F::ABI
}
