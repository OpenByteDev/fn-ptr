#![feature(adt_const_params)]

use core::{
    fmt::{Debug, Pointer},
    hash::Hash,
    panic::{RefUnwindSafe, UnwindSafe},
};

pub mod abi;
pub use abi::Abi;

mod r#impl;

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
}

/// Marker trait for all *safe* function pointer types (`fn` / `extern fn`).
pub trait SafeFnPtr: FnPtr {}

/// Marker trait for all *unsafe* function pointer types (`unsafe fn` / `unsafe extern fn`).
pub trait UnsafeFnPtr: FnPtr {}

/// Marker trait implemented for extern function pointers of a specific ABI.
///
/// For example:
/// - `HasAbi<Abi::C>` for `extern "C" fn(...)`
/// - `HasAbi<Abi::Sysv64>` for `extern "sysv64" fn(...)`
pub trait HasAbi<const ABI: Abi>: FnPtr {}

/// Computes the function pointer type obtained by changing the ABI
/// while preserving arity, arguments, return type, and safety.
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
pub const fn arity<F: FnPtr>() -> usize {
    F::ARITY
}

/// Returns `true` for safe function pointers (`fn`).
pub const fn is_safe<F: FnPtr>() -> bool {
    F::IS_SAFE
}

/// Returns `true` for unsafe function pointers (`unsafe fn`).
pub const fn is_unsafe<F: FnPtr>() -> bool {
    !is_safe::<F>()
}

/// Returns `true` if the function pointer uses an extern ABI.
pub const fn is_extern<F: FnPtr>() -> bool {
    F::IS_EXTERN
}

/// Returns the ABI of the function pointer.
pub const fn abi<F: FnPtr>() -> Abi {
    F::ABI
}

#[macro_export]
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

#[macro_export]
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
macro_rules! make_safe {
    ( $ty:ty ) => {
        <$ty as $crate::WithSafety<{ true }, $ty>>::F
    };
}

#[macro_export]
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
macro_rules! make_unsafe {
    ( $ty:ty ) => {
        <$ty as $crate::WithSafety<{ false }, $ty>>::F
    };
}

#[macro_export]
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
macro_rules! make_extern {
    ( $abi:path, $ty:ty ) => {
        $crate::with_abi!($abi, $ty)
    };
}

#[macro_export]
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
macro_rules! make_non_extern {
    ( $ty:ty ) => {
        $crate::with_abi!($crate::Abi::Rust, $ty)
    };
}
