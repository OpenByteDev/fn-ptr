#![feature(adt_const_params)]

use core::fmt::Debug;

pub mod abi;
pub use abi::Abi;

mod r#impl;

/// Trait representing a function.
///
/// # Safety
/// This trait should only be implemented for function pointers and the associated types and constants have to match the function pointer type.
pub unsafe trait FunctionPtr: Sized + Copy + Debug + Send + Sync + 'static {
    /// The argument types as a tuple.
    type Args;

    /// The return type.
    type Output;

    /// The function's arity (number of arguments).
    const ARITY: usize;

    /// Whether this function is safe to call.
    const SAFE: bool;

    /// Whether this function is extern.
    const EXTERN: bool;

    /// The ABI of this function.
    const ABI: Abi;
}

/// Convert a function pointer type to the same signature but with a different ABI.
pub trait WithAbi<const ABI: Abi, F: FunctionPtr> {
    /// The function pointer type with the requested ABI (preserving safety and signature).
    type F: FunctionPtr;
}

// Macro to map an ABI (either `Abi::Variant` or a string literal) and a function
// pointer type `T` into the associated `WithAbi` output type: `<T as WithAbi<{...}, T>>::F`
// Examples:
// - `with_abi!(crate::Abi::C, F)` expands to `<F as crate::WithAbi<{crate::Abi::C}, F>>::F`
// - `with_abi!("C", F)` expands to the same
#[macro_export]
macro_rules! with_abi {
    // Allow passing a path like `crate::Abi::C` or `Abi::C`
    ( $abi:path, $ty:ty ) => {
        < $ty as $crate::WithAbi<{ $abi }, $ty> >::F
    };

    // Allow passing a string literal and use the const parser to produce an `Abi`
    ( $abi_lit:literal, $ty:ty ) => {
        < $ty as $crate::WithAbi<{ $crate::abi::parse_or_fail($abi_lit) }, $ty> >::F
    };
}
