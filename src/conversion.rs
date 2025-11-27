use crate::{FnPtr, HasAbi, HasSafety, abi::AbiKey};

/// Computes the function pointer type obtained by changing the ABI
/// while preserving arity, arguments, return type, and safety.
pub trait WithAbi<const ABI: AbiKey> {
    /// The function pointer type with the requested ABI (preserving safety and signature).
    type F: FnPtr + HasAbi<ABI>;
}

/// Computes the function pointer type obtained by switching between safe/unsafe
/// while preserving arity, ABI, and signature.
pub trait WithSafety<const SAFE: bool> {
    /// The function pointer type with the requested safety (preserving ABI and signature).
    type F: FnPtr + HasSafety<SAFE>;
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
