use crate::{
    FnPtr, abi,
    safety::{self, Safe, Unsafe},
    tuple::Tuple,
};

/// Helper trait used by [`WithAbi`] (use it instead).
/// This trait is required to allow [`FnPtr`] to be a subtrait of [`WithAbi`], which eliminates
/// the need for a type bound when using a fixed abi marker.
/// Using [`WithAbi`] directly would result in a cyclic supertrait error.
pub trait WithAbiImpl<Abi: abi::Abi, F: FnPtr = Self> {
    /// The function pointer type with the requested abi (preserving all other properties).
    type F: FnPtr<Args = F::Args, Output = F::Output, Safety = F::Safety, Abi = Abi>;
}

/// Helper trait to change the abi of a function pointer type while preserving its safety, arguments and return type.
///
/// This is used by [`with_abi!`](crate::with_abi) under the hood.
///
/// # Example
///
/// ```rust
/// # use fn_ptr::{abi, WithAbi};
/// type F = extern "C" fn(i32) -> i32;
/// type G = <F as WithAbi<abi::System>>::F;
/// // `G` is `extern "system" fn(i32) -> i32`
/// # static_assertions::assert_type_eq_all!(G, extern "system" fn(i32) -> i32);
/// ```
pub trait WithAbi<Abi: abi::Abi>: FnPtr {
    /// The function pointer type with the requested abi (preserving all other properties).
    type F: FnPtr<Args = Self::Args, Output = Self::Output, Safety = Self::Safety, Abi = Abi>;
}
impl<Abi: abi::Abi, F: FnPtr + WithAbiImpl<Abi, Self>> WithAbi<Abi> for F {
    type F = <Self as WithAbiImpl<Abi, Self>>::F;
}

/// Helper trait used by [`WithSafety`] (use it instead).
/// This trait is required to allow [`FnPtr`] to be a subtrait of [`WithSafety`], which eliminates
/// the need for a type bound when using a fixed safety marker.
/// Using [`WithSafety`] directly would result in a cyclic supertrait error.
pub trait WithSafetyImpl<Safety: safety::Safety, F: FnPtr = Self> {
    /// The function pointer type with the requested safety (preserving all other properties).
    type F: FnPtr<Args = F::Args, Output = F::Output, Safety = Safety, Abi = F::Abi>;
}

/// Helper trait to change the safety of a function pointer type while preserving its abi, arguments and return type.
///
/// This is used by [`with_safety!`](crate::with_safety) under the hood.
///
/// # Example
///
/// ```rust
/// # use fn_ptr::{WithSafety, safety};
/// type F = extern "C" fn(i32) -> i32;
/// type G = <F as WithSafety<safety::Unsafe>>::F;
/// // `G` is `unsafe extern "C" fn(i32) -> i32`
/// # static_assertions::assert_type_eq_all!(G, unsafe extern "C" fn(i32) -> i32);
/// ```
pub trait WithSafety<Safety: safety::Safety>: FnPtr {
    /// The function pointer type with the requested safety (preserving all other properties).
    type F: FnPtr<Args = Self::Args, Output = Self::Output, Safety = Safety, Abi = Self::Abi>;
}
impl<Safety: safety::Safety, F: FnPtr + WithSafetyImpl<Safety, Self>> WithSafety<Safety> for F {
    type F = <Self as WithSafetyImpl<Safety, Self>>::F;
}

/// Helper trait to compute the safe version of a function pointer type while preserving its abi, arguments and return type. Equivalent to [`WithSafety<Safe>`].
///
/// # Example
///
/// ```rust
/// # use fn_ptr::AsSafe;
/// type U = unsafe extern "C" fn(i32) -> i32;
/// type S = <U as AsSafe>::F;
/// // `S` is `extern "C" fn(i32) -> i32`
/// # static_assertions::assert_type_eq_all!(S, extern "C" fn(i32) -> i32);
/// ```
pub trait AsSafe: WithSafety<Safe> {
    /// The safe version of this function pointer type.
    type F: FnPtr<Args = Self::Args, Output = Self::Output, Safety = Safe, Abi = Self::Abi>;
}
impl<F: FnPtr + WithSafety<Safe>> AsSafe for F {
    type F = <F as WithSafety<Safe>>::F;
}

/// Helper trait to compute the unsafe version of a function pointer type while preserving arity, abi and signature.
///
/// # Example
///
/// ```rust
/// # use fn_ptr::AsUnsafe;
/// type S = extern "C" fn(i32) -> i32;
/// type U = <S as AsUnsafe<S>>::F;
/// // `U` is `unsafe extern "C" fn(i32) -> i32`
/// # static_assertions::assert_type_eq_all!(U, unsafe extern "C" fn(i32) -> i32);
/// ```
pub trait AsUnsafe<F: FnPtr>: WithSafety<Unsafe> {
    /// The unsafe version of a function pointer type.
    type F: FnPtr<Args = F::Args, Output = F::Output, Safety = Unsafe, Abi = F::Abi>;
}
impl<F: FnPtr + WithSafety<Unsafe>> AsUnsafe<F> for F {
    type F = <F as WithSafety<Unsafe>>::F;
}

/// Helper trait used by [`WithOutput`] (use it instead).
/// This trait is required to allow [`FnPtr`] to be a subtrait of [`WithOutput`], which eliminates
/// the need for a type bound when using a fixed output type.
/// Using [`WithOutput`] directly would result in a cyclic supertrait error.
pub trait WithOutputImpl<F: FnPtr = Self> {
    /// The function pointer type with the requested output type (preserving all other properties).
    type F<T>: FnPtr<Args = F::Args, Output = T, Safety = F::Safety, Abi = F::Abi>;
}

/// Helper trait to change the return type of a function pointer type while preserving its safety, abi and arguments.
///
/// This is used by [`with_output!`](crate::with_output) under the hood.
///
/// # Example
///
/// ```rust
/// # use fn_ptr::WithOutput;
/// type F = extern "C" fn(i32) -> i32;
/// type G = <F as WithOutput<u32>>::F;
/// // `G` is `extern "C" fn(i32) -> u32`
/// # static_assertions::assert_type_eq_all!(G, extern "C" fn(i32) -> u32);
/// ```
pub trait WithOutput<T>: FnPtr + WithOutputImpl<Self> {
    /// The function pointer type with the requested return type (preserving all other properties).
    type F: FnPtr<Args = Self::Args, Output = T, Safety = Self::Safety, Abi = Self::Abi>;
}
impl<Output, F: FnPtr + WithOutputImpl<Self>> WithOutput<Output> for F {
    type F = <Self as WithOutputImpl<Self>>::F<Output>;
}

/// Helper trait used by [`WithArgs`] (use it instead).
/// This trait is required to allow [`FnPtr`] to be a subtrait of [`WithArgs`], which eliminates
/// the need for a type bound when using a fixed args type.
/// Using [`WithArgs`] directly would result in a cyclic supertrait error.
pub trait WithArgsImpl<F: FnPtr = Self> {
    /// The function pointer type with the requested argument type(s) (preserving all other properties).
    type F<Args: Tuple>: FnPtr<Args = Args, Output = F::Output, Safety = F::Safety, Abi = F::Abi>;
}

/// Helper trait to change the argument types of a function pointer type while preserving its safety, abi and return type.
///
/// This is used by [`with_args!`](crate::with_args) under the hood.
///
/// # Example
///
/// ```rust
/// # use fn_ptr::WithArgs;
/// type F = extern "C" fn(i32) -> i32;
/// type G = <F as WithArgs<(u8, u16)>>::F;
/// // `G` `extern "C" fn(u8, u16) -> i32`
/// # static_assertions::assert_type_eq_all!(G, extern "C" fn(u8, u16) -> i32);
/// ```
pub trait WithArgs<Args: Tuple>: FnPtr {
    /// The function pointer type with the requested argument type(s) (preserving all other properties).
    type F: FnPtr<Args = Args, Output = Self::Output, Safety = Self::Safety, Abi = Self::Abi>;
}
impl<Args: Tuple, F: FnPtr + WithArgsImpl<Self>> WithArgs<Args> for F {
    type F = <Self as WithArgsImpl<Self>>::F<Args>;
}

/// Construct a function-pointer type identical to the given one but using the specified abi.
///
/// Accepts either:
/// - an [`Abi`](crate::abi::Abi) marker type (e.g. [`C`](crate::abi::C), [`SysV64`](crate::abi::SysV64))
/// - a string literal (e.g. `"C"`, `"system"`, `"stdcall"`).
///
/// # Examples
///
/// ```rust
/// # use fn_ptr::{with_abi, abi};
/// type F = with_abi!(abi::Rust, extern "C" fn(i32) -> i32);
/// // `F` is `fn(i32) -> i32` (equivalent to `extern "Rust" fn(i32) -> i32`)
/// # static_assertions::assert_type_eq_all!(F, fn(i32) -> i32);
///
/// type G = with_abi!("C", extern "system" fn());
/// // `G` is `extern "C" fn()`
/// # static_assertions::assert_type_eq_all!(G, extern "C" fn());
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
/// - a [`Safety`](crate::safety::Safety) marker type ([`Safe`](crate::safety::Safe) or [`Unsafe`](crate::safety::Unsafe))
/// - safety keyword (`safe` or `unsafe`).
/// - a boolean literal.
///
/// # Examples
///
/// ```rust
/// # use fn_ptr::{with_safety, safety};
/// type F = with_safety!(safety::Safe, unsafe extern "C" fn(i32) -> i32);
/// // `F` is `extern "C" fn(i32) -> i32`
/// # static_assertions::assert_type_eq_all!(F, extern "C" fn(i32) -> i32);
///
/// type G = with_safety!(unsafe, fn());
/// // `G` is `unsafe fn()`
/// # static_assertions::assert_type_eq_all!(G, unsafe fn());
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
/// # static_assertions::assert_type_eq_all!(S, extern "C" fn(i32));
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
/// # static_assertions::assert_type_eq_all!(U, unsafe extern "C" fn(i32));
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
/// # static_assertions::assert_type_eq_all!(G, extern "C" fn(i32) -> u64);
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
/// # static_assertions::assert_type_eq_all!(G, extern "C" fn(u8, u16) -> i32);
/// ```
#[macro_export]
macro_rules! with_args {
    ( $args:ty, $ty:ty ) => {
        <$ty as $crate::WithArgs<$args>>::F
    };
}
