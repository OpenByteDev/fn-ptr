use crate::{
    FnPtr, HasAbi, HasSafety, markers::{self, Safe, Unsafe}
};

/// Helper trait to change the ABI of a function pointer type while preserving arity, safety, and signature.
pub trait WithAbi<Abi>: FnPtr
where
    Abi: markers::Abi,
{
    /// The function pointer type with the requested ABI (preserving safety and signature).
    type F: FnPtr<
            Args = Self::Args,
            Output = Self::Output,
            ArityMarker = Self::ArityMarker,
            SafetyMarker = Self::SafetyMarker,
            AbiMarker = Abi,
        > + HasAbi<Abi>;
}

/// Helper trait to change the safety of a function pointer type while preserving arity, ABI, and signature.
pub trait WithSafety<Safety>: FnPtr
where
    Safety: markers::Safety,
{
    /// The function pointer type with the requested safety (preserving ABI and signature).
    type F: FnPtr<
            Args = Self::Args,
            Output = Self::Output,
            ArityMarker = Self::ArityMarker,
            SafetyMarker = Safety,
            AbiMarker = Self::AbiMarker,
        > + HasSafety<Safety>;
}

/// Helper trait to compute the safe version of a function pointer type while preserving arity, ABI, and signature.
pub trait AsSafe: WithSafety<Safe> {}
impl<F: WithSafety<Safe>> AsSafe for F {}

/// Helper trait to compute the unsafe version of a function pointer type while preserving arity, ABI, and signature.
pub trait AsUnsafe: WithSafety<Unsafe> {}
impl<F: WithSafety<Unsafe>> AsUnsafe for F {}

/// Construct a function-pointer type identical to the given one but using
/// the specified ABI.
///
/// Accepts either:
/// - an `Abi` value (e.g., `Abi::C`, `Abi::SysV64`), or
/// - a string literal (e.g., `"C"`, `"system"`, `"stdcall"`).
///
/// # Examples
///
/// ```rust
/// # use fn_ptr::{with_abi, markers};
/// type F = extern "C" fn(i32) -> i32;
///
/// type G = with_abi!(markers::SysV64, F);
/// // `G` is `extern "sysv64" fn(i32) -> i32`
///
/// type H = with_abi!("C", extern "system" fn());
/// // `H` is `extern "C" fn()`
/// ```
#[macro_export]
macro_rules! with_abi {
    // ABI given as a path (Abi::C, Abi::SysV64, ...)
    ( $abi:path, $ty:ty ) => {
        <$ty as $crate::WithAbi<$abi>>::F
    };

    // ABI given as a string literal
    ( $abi_lit:tt, $ty:ty ) => {
        <$ty as $crate::WithAbi<$crate::markers::abi!($abi_lit)>>::F
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
        <$ty as $crate::WithSafety<$crate::markers::Safe>>::F
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
        <$ty as $crate::WithSafety<$crate::markers::Unsafe>>::F
    };
}
