#![cfg_attr(nightly_build, feature(adt_const_params, fn_ptr_trait))]
#![warn(clippy::pedantic)]
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
//! assert_eq!(<F as FnPtr>::ABI, Abi::C);
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
//! let c_add: extern "C" fn(i32, i32) -> i32 = unsafe { rust_add.with_abi::<{abi!("C")}>() };
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

use core::{
    fmt::{Debug, Pointer},
    hash::Hash,
    panic::{RefUnwindSafe, UnwindSafe},
};

/// Module containing the Abi abstraction.
pub mod abi;
pub use abi::Abi;
pub(crate) use abi::AbiKey;

mod r#impl;
pub mod prelude;

ffi_opaque::opaque! {
    /// A struct representing an opaque function.
    pub struct OpaqueFn;
}

/// Type alias for a raw untyped function pointer.
pub type UntypedFnPtr = *const OpaqueFn;

macro_rules! fnptr_trait_body {
    () => {
        /// The argument types as a tuple.
        type Args;

        /// The return type.
        type Output;

        /// The function's arity (number of arguments).
        const ARITY: usize;

        /// Whether the function pointer is safe (fn) or unsafe (unsafe fn).
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

        /// Produces an unsafe version of this function pointer.
        #[must_use]
        fn as_unsafe(&self) -> make_unsafe!(Self) {
            unsafe { FnPtr::from_ptr(self.as_ptr()) }
        }

        /// Produces a safe version of this function pointer.
        ///
        /// # Safety
        /// Caller must ensure the underlying function is actually safe to call.
        #[must_use]
        unsafe fn as_safe(&self) -> make_safe!(Self) {
            unsafe { FnPtr::from_ptr(self.as_ptr()) }
        }

        /// Produces a version of this function pointer with the given ABI.
        ///
        /// # Safety
        /// Caller must ensure that the resulting ABI transformation is sound.
        #[must_use]
        unsafe fn with_abi<const ABI: AbiKey>(&self) -> <Self as WithAbi<ABI>>::F
        where
            Self: WithAbi<ABI>,
        {
            unsafe { FnPtr::from_ptr(self.as_ptr()) }
        }
    };
}
/// Marker trait for all function pointers.
#[cfg(not(nightly_build))]
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
    + Sized
    + 'static
    + WithSafety<true>
    + WithSafety<false>
    + WithAbi<{ abi!("Rust") }>
    + WithAbi<{ abi!("C") }>
    + WithAbi<{ abi!("system") }>
{
    fnptr_trait_body!();
}

/// Marker trait for all function pointers.
#[cfg(nightly_build)]
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
    + Sized
    + 'static
    + WithSafety<true>
    + WithSafety<false>
    + WithAbi<{ abi!("Rust") }>
    + WithAbi<{ abi!("C") }>
    + WithAbi<{ abi!("system") }>
    + std::marker::FnPtr
{
    fnptr_trait_body!();
}

/// Marker trait for all *safe* function pointer types (`fn` / `extern fn`).
pub trait SafeFnPtr: FnPtr {
    /// Invokes the function pointed to with the given args.
    ///
    /// # Examples
    ///
    /// ```
    /// # use fn_ptr::SafeFnPtr;
    /// fn add(a: i32, b: i32) -> i32 { a + b }
    ///
    /// let f: fn(i32, i32) -> i32 = add;
    /// let result = f.invoke((2, 3));
    /// assert_eq!(result, 5);
    /// ```
    // NOTE: Can't use "call" due to fn_traits feature
    fn invoke(&self, args: Self::Args) -> Self::Output;
}

/// Marker trait for all *unsafe* function pointer types (`unsafe fn` / `unsafe extern fn`).
pub trait UnsafeFnPtr: FnPtr {
    /// Invokes the function pointed to with the given args.
    ///
    /// # Safety
    /// Calling this function pointer is unsafe because the function may have
    /// invariants that must be manually upheld by the caller.
    /// 
    /// # Examples
    ///
    /// ```
    /// # use fn_ptr::UnsafeFnPtr;
    /// unsafe fn add(a: *const i32, b: *const i32) -> i32 { *a + *b }
    ///
    /// let f: unsafe fn(*const i32, *const i32) -> i32 = add;
    /// let result = unsafe { f.invoke((&2, &3)) };
    /// assert_eq!(result, 5);
    /// ```
    // NOTE: Can't use "call" due to fn_traits feature
    unsafe fn invoke(&self, args: Self::Args) -> Self::Output;
}

/// Marker trait implemented for function pointers of a specific ABI.
///
/// For example:
/// - `HasAbi<Abi::C>` for `extern "C" fn(...)`
/// - `HasAbi<Abi::Sysv64>` for `extern "sysv64" fn(...)`
pub trait HasAbi<const ABI: AbiKey>: FnPtr {}

/// Computes the function pointer type obtained by changing the ABI
/// while preserving arity, arguments, return type, and safety.
pub trait WithAbi<const ABI: AbiKey> {
    /// The function pointer type with the requested ABI (preserving safety and signature).
    type F: FnPtr + HasAbi<ABI>;
}

/// Marker trait denoting the safety of a function pointer type.
///
/// For example:
/// - `HasSafety<true>` for `extern "C" fn(...)`
/// - `HasSafety<false>` for `unsafe fn(...)`
pub trait HasSafety<const B: bool> {}
impl<T: SafeFnPtr> HasSafety<true> for T {}
impl<T: UnsafeFnPtr> HasSafety<false> for T {}

/// Computes the function pointer type obtained by switching between safe/unsafe
/// while preserving arity, ABI, and signature.
pub trait WithSafety<const SAFE: bool> {
    /// The function pointer type with the requested safety (preserving ABI and signature).
    type F: FnPtr + HasSafety<SAFE>;
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
#[macro_export]
macro_rules! with_abi {
    // ABI given as a path (Abi::C, Abi::Sysv64, ...)
    ( $abi:path, $ty:ty ) => {
        <$ty as $crate::WithAbi<{ $crate::abi::key($abi) }>>::F
    };

    // ABI given as a string literal
    ( $abi_lit:literal, $ty:ty ) => {
        <$ty as $crate::WithAbi<{ $crate::abi!($abi_lit) }>>::F
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
        <$ty as $crate::WithSafety<{ true }>>::F
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
        <$ty as $crate::WithSafety<{ false }>>::F
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
#[macro_export]
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
#[macro_export]
macro_rules! make_non_extern {
    ( $ty:ty ) => {
        $crate::with_abi!($crate::Abi::Rust, $ty)
    };
}

/// Converts an ABI string like "C" into the corresponding value for use in const generics.
/// This is most useful for stable rust since there [`u8`]s are used.
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
#[macro_export]
macro_rules! abi {
    ( $abi:literal ) => {
        $crate::abi::key($crate::abi::parse_or_fail($abi))
    };
}

