use crate::{
    FnPtr, HasAbi, HasSafety,
    marker::{self, Safe, Unsafe},
};

/// Helper trait to change the ABI of a function pointer type while preserving arity, safety, and signature.
pub trait WithAbi<Abi>: FnPtr
where
    Abi: marker::Abi,
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
    Safety: marker::Safety,
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

cfg_tt::cfg_tt!{
/// Helper trait that simplifies generic bounds when converting between funciton pointer types.
pub trait Convertible:
    FnPtr
    + WithAbi<marker::Rust>
    + WithAbi<marker::C>
    + WithAbi<marker::CUnwind>
    + WithAbi<marker::System>
    + WithAbi<marker::SystemUnwind>
    + WithSafety<marker::Safe>
    + WithSafety<marker::Unsafe>
    + AsSafe
    + AsUnsafe
    #[cfg(has_abi_aapcs)](+ WithAbi<marker::Aapcs>)
    #[cfg(has_abi_aapcs)](+ WithAbi<marker::AapcsUnwind>)
    #[cfg(has_abi_cdecl)](+ WithAbi<marker::Cdecl>)
    #[cfg(has_abi_cdecl)](+ WithAbi<marker::CdeclUnwind>)
    #[cfg(has_abi_stdcall)](+ WithAbi<marker::Stdcall>)
    #[cfg(has_abi_stdcall)](+ WithAbi<marker::StdcallUnwind>)
    #[cfg(has_abi_fastcall)](+ WithAbi<marker::Fastcall>)
    #[cfg(has_abi_fastcall)](+ WithAbi<marker::FastcallUnwind>)
    #[cfg(has_abi_thiscall)](+ WithAbi<marker::Thiscall>)
    #[cfg(has_abi_thiscall)](+ WithAbi<marker::ThiscallUnwind>)
    #[cfg(has_abi_vectorcall)](+ WithAbi<marker::Vectorcall>)
    #[cfg(has_abi_vectorcall)](+ WithAbi<marker::VectorcallUnwind>)
    #[cfg(has_abi_sysv64)](+ WithAbi<marker::SysV64>)
    #[cfg(has_abi_sysv64)](+ WithAbi<marker::SysV64Unwind>)
    #[cfg(has_abi_win64)](+ WithAbi<marker::Win64>)
    #[cfg(has_abi_win64)](+ WithAbi<marker::Win64Unwind>)
{
}
impl<T> Convertible for T
where
    T: FnPtr
        + WithAbi<marker::Rust>
        + WithAbi<marker::C>
        + WithAbi<marker::CUnwind>
        + WithAbi<marker::System>
        + WithAbi<marker::SystemUnwind>
        + WithSafety<marker::Safe>
        + WithSafety<marker::Unsafe>
        + AsSafe
        + AsUnsafe
        #[cfg(has_abi_aapcs)](+ WithAbi<marker::Aapcs>)
        #[cfg(has_abi_aapcs)](+ WithAbi<marker::AapcsUnwind>)
        #[cfg(has_abi_cdecl)](+ WithAbi<marker::Cdecl>)
        #[cfg(has_abi_cdecl)](+ WithAbi<marker::CdeclUnwind>)
        #[cfg(has_abi_stdcall)](+ WithAbi<marker::Stdcall>)
        #[cfg(has_abi_stdcall)](+ WithAbi<marker::StdcallUnwind>)
        #[cfg(has_abi_fastcall)](+ WithAbi<marker::Fastcall>)
        #[cfg(has_abi_fastcall)](+ WithAbi<marker::FastcallUnwind>)
        #[cfg(has_abi_thiscall)](+ WithAbi<marker::Thiscall>)
        #[cfg(has_abi_thiscall)](+ WithAbi<marker::ThiscallUnwind>)
        #[cfg(has_abi_vectorcall)](+ WithAbi<marker::Vectorcall>)
        #[cfg(has_abi_vectorcall)](+ WithAbi<marker::VectorcallUnwind>)
        #[cfg(has_abi_sysv64)](+ WithAbi<marker::SysV64>)
        #[cfg(has_abi_sysv64)](+ WithAbi<marker::SysV64Unwind>)
        #[cfg(has_abi_win64)](+ WithAbi<marker::Win64>)
        #[cfg(has_abi_win64)](+ WithAbi<marker::Win64Unwind>)
{}
}

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
/// # use fn_ptr::{with_abi, marker};
/// type F = extern "C" fn(i32) -> i32;
///
/// type G = with_abi!(marker::SysV64, F);
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
        <$ty as $crate::WithAbi<$crate::abi!($abi_lit)>>::F
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
        <$ty as $crate::WithSafety<$crate::marker::Safe>>::F
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
        <$ty as $crate::WithSafety<$crate::marker::Unsafe>>::F
    };
}
