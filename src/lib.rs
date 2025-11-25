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
