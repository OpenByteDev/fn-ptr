#![cfg_attr(feature = "nightly", feature(adt_const_params))]
#![warn(clippy::pedantic)]
//! # fn-ptr
//!
//! `fn-ptr` is a small utility crate that provides a [`FnPtr`] trait, implemented for all function pointer types:
//! - `fn(T) -> U`
//! - `unsafe fn(T) -> U`
//! - `extern "C" fn(T)`
//! - `unsafe extern "sysv64" fn() -> i32`
//!
//! The trait provides associated types and constants to introspect function pointer types at compile time.
//!
//! ```rust
//! # use fn_ptr::Abi;
//! #
//! pub trait FnPtr {
//!     /// The argument types as a tuple.
//!     type Args;
//!
//!     /// The return type.
//!     type Output;
//!
//!     /// The function's arity (number of arguments).
//!     const ARITY: usize;
//!
//!     /// Whether the function pointer is safe (`fn`) or unsafe (`unsafe fn`).
//!     const IS_SAFE: bool;
//!
//!     /// Whether the function pointer uses an extern calling convention.
//!     const IS_EXTERN: bool;
//!
//!     /// The ABI associated with this function pointer.
//!     const ABI: Abi;
//! }
//! ```
//!
//! ## Features
//!
//! ### 1. Function Pointer Metadata
//!
//! Every function pointer automatically implements [`FnPtr`].
//! Depending on the type, it may also implement [`SafeFnPtr`], [`UnsafeFnPtr`], and [`HasAbi<Abi>`].
//!
//! ```rust
//! use fn_ptr::{FnPtr, Abi};
//!
//! type F = extern "C" fn(i32, i32) -> i32;
//!
//! assert_eq!(<F as FnPtr>::ARITY, 2);
//! assert_eq!(<F as FnPtr>::IS_SAFE, true);
//! assert_eq!(<F as FnPtr>::IS_EXTERN, true);
//! assert_eq!(<F as FnPtr>::ABI, Abi::C);
//! ```
//!
//! Const helper functions are also provided:
//!
//! ```rust
//! # use fn_ptr::{FnPtr, Abi};
//! # type F = extern "C" fn(i32, i32) -> i32;
//! const A: usize = fn_ptr::arity::<F>();         // 2
//! const SAFE: bool = fn_ptr::is_safe::<F>();     // true
//! const EXT: bool = fn_ptr::is_extern::<F>();    // true
//! const ABI: Abi = fn_ptr::abi::<F>();           // Abi::C
//! ```
//!
//! ### 2. Changing ABIs at the Type Level
//!
//! You can change the ABI of a function pointer type using macros:
//!
//! ```rust
//! # #[cfg(feature = "nightly")] {
//! use fn_ptr::{with_abi, Abi};
//!
//! type F = extern "C" fn(i32) -> i32;
//!
//! type G = with_abi!(Abi::Sysv64, F);
//! type H = with_abi!("C", extern "system" fn());
//! }
//! ```
//!
//! ### 3. Toggle Function Pointer Safety
//!
//! Macros are provided to make function pointers safe or unsafe:
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
//! ## How It Works
//!
//! All macros rely on type-level traits [`WithAbi`] and [`WithSafety`].
//! Each trait exposes an associated type representing the transformed function pointer. You can use these traits directly for const generics or explicit type transformations:
//!
//! ```rust
//! # #[cfg(feature = "nightly")] {
//! use fn_ptr::{FnPtr, WithAbi, WithSafety, Abi};
//!
//! type F = extern "C" fn(i32);
//! type G = <F as WithAbi<{Abi::Sysv64}, F>>::F;
//! type U = <F as WithSafety<{false}, F>>::F;
//! }
//! ```
//!
//! ## License
//!
//! Licensed under the MIT license, see [LICENSE](https://github.com/OpenByteDev/fn-ptr/blob/master/LICENSE) for details.

use core::{
    fmt::{Debug, Pointer},
    hash::Hash,
    panic::{RefUnwindSafe, UnwindSafe},
};

/// Module containing the Abi abstraction.
pub mod abi;
pub use abi::Abi;

mod r#impl;

ffi_opaque::opaque! {
    /// A struct representing an opaque function.
    pub struct OpaqueFn;
}

/// Type alias for a raw untyped function pointer.
pub type UntypedFnPtr = *const OpaqueFn;

/// Marker trait for all function pointers.
// list of implemented traits from https://rust.docs.kernel.org/core/primitive.fn.html#trait-implementations-1
pub trait FnPtr:
    PartialEq
    + Eq
    + PartialOrd
    + Ord
    + Hash
    + Pointer
    + Debug
    + Clone
    + Copy
    + Send
    + Sync
    + Unpin
    + UnwindSafe
    + RefUnwindSafe
    + 'static
{
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

    /// Returns the address of this function.
    #[must_use]
    fn addr(&self) -> usize {
        self.as_ptr() as usize
    }
    /// Constructs an instance from an address.
    ///
    /// # Safety
    /// This function is unsafe because it can not check if the argument points to a function
    /// of the correct type.
    #[must_use]
    unsafe fn from_addr(addr: usize) -> Self {
        unsafe { Self::from_ptr(addr as UntypedFnPtr) }
    }

    /// Returns a untyped function pointer for this function.
    #[must_use]
    fn as_ptr(&self) -> UntypedFnPtr;
    /// Constructs an instance from an untyped function pointer.
    ///
    /// # Safety
    /// This function is unsafe because it can not check if the argument points to a function
    /// of the correct type.
    #[must_use]
    unsafe fn from_ptr(ptr: UntypedFnPtr) -> Self;
}

/// Marker trait for all *safe* function pointer types (`fn` / `extern fn`).
pub trait SafeFnPtr: FnPtr {
    /// Invokes the function pointed to with the given args.
    // NOTE: Can't use call due to fn_traits feature
    fn invoke(&self, args: Self::Args) -> Self::Output;
}

/// Marker trait for all *unsafe* function pointer types (`unsafe fn` / `unsafe extern fn`).
pub trait UnsafeFnPtr: FnPtr {
    /// Invokes the function pointed to with the given args.
    ///
    /// # Safety
    /// Calling this function pointer is unsafe because the function may have
    /// invariants that must be manually upheld by the caller.
    // NOTE: Can't use call due to fn_traits feature
    unsafe fn invoke(&self, args: Self::Args) -> Self::Output;
}

/// Marker trait implemented for extern function pointers of a specific ABI.
///
/// For example:
/// - `HasAbi<Abi::C>` for `extern "C" fn(...)`
/// - `HasAbi<Abi::Sysv64>` for `extern "sysv64" fn(...)`
#[cfg(feature = "nightly")]
pub trait HasAbi<const ABI: Abi>: FnPtr {}

/// Computes the function pointer type obtained by changing the ABI
/// while preserving arity, arguments, return type, and safety.
#[cfg(feature = "nightly")]
pub trait WithAbi<const ABI: Abi, F: FnPtr> {
    /// The function pointer type with the requested ABI (preserving safety and signature).
    type F: FnPtr;
}

/// Computes the function pointer type obtained by switching between safe/unsafe
/// while preserving arity, ABI, and signature.
pub trait WithSafety<const SAFE: bool, F: FnPtr> {
    /// The function pointer type with the requested safety (preserving ABI and signature).
    type F: FnPtr;
}

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

/// Construct a function-pointer type identical to the given one but using
/// the specified ABI.
///
/// Accepts either:
/// - an `Abi` value (e.g., `Abi::C`, `Abi::Sysv64`), or
/// - a string literal (e.g., `"C"`, `"system"`, `"stdcall"`).
///
/// # Examples
///
/// ```rust
/// # use fn_ptr::{with_abi, Abi};
/// type F = extern "C" fn(i32) -> i32;
///
/// type G = with_abi!(Abi::Sysv64, F);
/// // `G` is `extern "sysv64" fn(i32) -> i32`
///
/// type H = with_abi!("C", extern "system" fn());
/// // `H` is `extern "C" fn()`
/// ```
#[cfg(feature = "nightly")]
#[cfg_attr(feature = "nightly", macro_export)]
macro_rules! with_abi {
    // ABI given as a path (Abi::C, Abi::Sysv64, ...)
    ( $abi:path, $ty:ty ) => {
        <$ty as $crate::WithAbi<{ $abi }, $ty>>::F
    };

    // ABI given as a string literal
    ( $abi_lit:literal, $ty:ty ) => {
        <$ty as $crate::WithAbi<{ $crate::abi::parse_or_fail($abi_lit) }, $ty>>::F
    };
}

/// Convert a function-pointer type to the *safe* variant of the same
/// signature. Arguments, return type, and ABI are preserved.
///
/// # Example
///
/// ```rust
/// # use fn_ptr::make_safe;
/// type U = unsafe extern "C" fn(i32);
/// type S = make_safe!(U);
/// // `S` is `extern "C" fn(i32)`
/// ```
#[macro_export]
macro_rules! make_safe {
    ( $ty:ty ) => {
        <$ty as $crate::WithSafety<{ true }, $ty>>::F
    };
}

/// Convert a function-pointer type to the *unsafe* variant of the same
/// signature. Arguments, return type, and ABI are preserved.
///
/// # Example
///
/// ```rust
/// # use fn_ptr::make_unsafe;
/// type S = extern "C" fn(i32);
/// type U = make_unsafe!(S);
/// // `U` is `unsafe extern "C" fn(i32)`
/// ```
#[macro_export]
macro_rules! make_unsafe {
    ( $ty:ty ) => {
        <$ty as $crate::WithSafety<{ false }, $ty>>::F
    };
}

/// Convert a function-pointer type to an `extern` function that uses
/// the specified ABI. Arguments, return type, and safety are preserved.
///
/// # Example
///
/// ```rust
/// # use fn_ptr::{make_extern, Abi};
/// type F = fn(i32) -> i32;
/// type C = make_extern!(Abi::C, F);
/// // `C` is `extern "C" fn(i32) -> i32`
/// ```
///
/// Equivalent to:
/// ```rust
/// # use fn_ptr::{Abi, with_abi};
/// # type F = fn(i32) -> i32;
/// # type G =
/// with_abi!(Abi::C, F)
/// # ;
/// ```
#[cfg(feature = "nightly")]
#[cfg_attr(feature = "nightly", macro_export)]
macro_rules! make_extern {
    ( $abi:path, $ty:ty ) => {
        $crate::with_abi!($abi, $ty)
    };
}

/// Convert a function-pointer type to a Rust-ABI (`fn`) function while
/// preserving its arguments, return type, and safety.
///
/// # Example
///
/// ```rust
/// # use fn_ptr::make_non_extern;
/// type F = extern "C" fn(i32) -> i32;
/// type R = make_non_extern!(F);
/// // `R` is `fn(i32) -> i32`
/// ```
///
/// Equivalent to:
/// ```rust
/// # use fn_ptr::{Abi, with_abi};
/// # type F = extern "C" fn(i32) -> i32;
/// # type G =
/// with_abi!(Abi::Rust, F)
/// # ;
/// ```
#[cfg(feature = "nightly")]
#[cfg_attr(feature = "nightly", macro_export)]
macro_rules! make_non_extern {
    ( $ty:ty ) => {
        $crate::with_abi!($crate::Abi::Rust, $ty)
    };
}
