use crate::{
    FnPtr, abi,
    safety::{self, Safe, Unsafe},
    tuple::Tuple,
};

/// Helper trait to change the ABI of a function pointer type while preserving arity, safety, signature, etc.
pub trait WithAbi<Abi>: FnPtr
where
    Abi: abi::Abi,
{
    /// The function pointer type with the requested ABI (preserving safety and signature).
    type F: FnPtr<Args = Self::Args, Output = Self::Output, Safety = Self::Safety, Abi = Abi>;
}

/// Helper trait to change the safety of a function pointer type while preserving arity, ABI, signature, etc.
pub trait WithSafety<Safety>: FnPtr
where
    Safety: safety::Safety,
{
    /// The function pointer type with the requested safety (preserving ABI and signature).
    type F: FnPtr<Args = Self::Args, Output = Self::Output, Safety = Safety, Abi = Self::Abi>;
}

/// Helper trait to compute the safe version of a function pointer type while preserving arity, ABI, signature, etc.
pub trait AsSafe: WithSafety<Safe> {
    /// The safe version of this function pointer type.
    type F: FnPtr<Args = Self::Args, Output = Self::Output, Safety = Safe, Abi = Self::Abi>;
}
impl<F: WithSafety<Safe>> AsSafe for F {
    type F = F::F;
}

/// Helper trait to compute the unsafe version of a function pointer type while preserving arity, ABI, signature, etc.
pub trait AsUnsafe: WithSafety<Unsafe> {
    /// The unsafe version of this function pointer type.
    type F: FnPtr<Args = Self::Args, Output = Self::Output, Safety = Unsafe, Abi = Self::Abi>;
}
impl<F: WithSafety<Unsafe>> AsUnsafe for F {
    type F = F::F;
}

/// Helper trait to change the return type of a function pointer type while preserving arguments, safety, ABI, etc.
pub trait WithOutput<T>: FnPtr {
    /// The function pointer type with the requested return type.
    type F: FnPtr<Args = Self::Args, Output = T, Safety = Self::Safety, Abi = Self::Abi>;
}

/// Helper trait to change the arguments of a function pointer type while preserving ABI, safety, signature, etc.
pub trait WithArgs<Args: Tuple>: FnPtr {
    /// The function pointer type with the requested argument types.
    type F: FnPtr<Args = Args, Output = Self::Output, Safety = Self::Safety, Abi = Self::Abi>;
}

/// Construct a function-pointer type identical to the given one but using
/// the specified ABI.
///
/// Accepts either:
/// - an [`Abi`](crate::abi::Abi) marker type (e.g., [`C`](crate::abi::C), [`SysV64`](crate::abi::SysV64)), or
/// - a string literal (e.g., `"C"`, `"system"`, `"stdcall"`).
///
/// # Examples
///
/// ```rust
/// # use fn_ptr::{with_abi, abi};
/// type F = with_abi!(abi::SysV64, extern "C" fn(i32) -> i32);
/// // `F` is `extern "sysv64" fn(i32) -> i32`
///
/// type G = with_abi!("C", extern "system" fn());
/// // `G` is `extern "C" fn()`
/// ```
#[macro_export]
macro_rules! with_abi {
    ( $abi:path, $ty:ty ) => {
        <$ty as $crate::WithAbi<$abi>>::F
    };

    ( $lit:tt, $ty:ty ) => {
        <$ty as $crate::WithAbi<$crate::abi!($lit)>>::F
    };
}

/// Construct a function-pointer type identical to the given one but using
/// the specified safety.
///
/// Accepts either:
/// - an [`Safety`](crate::safety::Safety) marker type ([`Safe`](crate::safety::Safe) or [`Unsafe`](crate::safety::Unsafe))
/// - safety keyword (`safe` or `unsafe`).
/// - a boolean literal.
///
/// # Examples
///
/// ```rust
/// # use fn_ptr::{with_safety, safety};
/// type F = with_safety!(safety::Safe, unsafe extern "C" fn(i32) -> i32);
/// // `F` is `extern "C" fn(i32) -> i32`
///
/// type G = with_safety!(unsafe, fn());
/// // `G` is `unsafe fn()`
/// ```
#[macro_export]
macro_rules! with_safety {
    ( safe, $ty:ty ) => {
        $crate::with_safety!(@inner $crate::safety::Safe, $ty)
    };
    ( unsafe, $ty:ty ) => {
        $crate::with_safety!(@inner $crate::safety::Unsafe, $ty)
    };
    ( $safety:path, $ty:ty ) => {
        $crate::with_safety!(@inner $safety, $ty)
    };
    ( $safety:tt, $ty:ty ) => {
        $crate::with_safety!(@inner $crate::safety!($safety), $ty)
    };

    ( @inner $safety:ty, $ty:ty ) => {
        <$ty as $crate::WithSafety<$safety>>::F
    };
}

/// Construct a function-pointer type identical to the given one but `safe`.
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
        <$ty as $crate::WithSafety<$crate::safety::Safe>>::F
    };
}

/// Construct a function-pointer type identical to the given one but `unsafe`.
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
        <$ty as $crate::WithSafety<$crate::safety::Unsafe>>::F
    };
}

/// Construct a function-pointer type identical to the given one but using
/// the specified return type.
///
/// # Examples
///
/// ```rust
/// # use fn_ptr::with_output;
/// type F = extern "C" fn(i32) -> i32;
/// type G = with_output!(u64, F);
/// // `G` is `extern "C" fn(i32) -> u64`
/// ```
#[macro_export]
macro_rules! with_output {
    ( $out:ty, $ty:ty ) => {
        <$ty as $crate::WithOutput<$out>>::F
    };
}

/// Construct a function-pointer type identical to the given one but using
/// the specified argument tuple type.
///
/// # Examples
///
/// ```rust
/// # use fn_ptr::with_args;
/// type F = extern "C" fn(i32) -> i32;
/// type G = with_args!((u8, u16), F);
/// // `G` is `extern "C" fn(u8, u16) -> i32`
/// ```
#[macro_export]
macro_rules! with_args {
    ( $args:ty, $ty:ty ) => {
        <$ty as $crate::WithArgs<$args>>::F
    };
}
