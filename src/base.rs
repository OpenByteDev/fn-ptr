use core::{
    fmt::{Debug, Pointer},
    hash::Hash,
    panic::{RefUnwindSafe, UnwindSafe},
};

use crate::{
    WithAbi, WithAbiImpl, WithArgs, WithArgsImpl, WithOutput, WithOutputImpl, WithSafety,
    WithSafetyImpl, abi,
    abi_value::AbiValue,
    safety::{self, Safe, Unsafe},
    tuple::Tuple,
};

ffi_opaque::opaque! {
    /// A struct representing an opaque function.
    pub struct OpaqueFn;
}

/// Type alias for a raw untyped function pointer.
pub type UntypedFnPtr = *const OpaqueFn;

cfg_tt::cfg_tt! {
/// Marker trait for all function pointers.
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
    + WithOutputImpl
    + WithArgsImpl
    + WithSafetyImpl<safety::Safe>
    + WithSafetyImpl<safety::Unsafe>
    + WithAbiImpl<abi::Rust>
    + WithAbiImpl<abi::C>
    + WithAbiImpl<abi::CUnwind>
    + WithAbiImpl<abi::System>
    + WithAbiImpl<abi::SystemUnwind>
    #[cfg(nightly_build)](+ core::marker::FnPtr)
    #[cfg(has_abi_aapcs)](+ WithAbiImpl<abi::Aapcs>)
    #[cfg(has_abi_aapcs)](+ WithAbiImpl<abi::AapcsUnwind>)
    #[cfg(has_abi_cdecl)](+ WithAbiImpl<abi::Cdecl>)
    #[cfg(has_abi_cdecl)](+ WithAbiImpl<abi::CdeclUnwind>)
    #[cfg(has_abi_cdecl)](+ WithAbiImpl<abi::Stdcall>)
    #[cfg(has_abi_cdecl)](+ WithAbiImpl<abi::StdcallUnwind>)
    #[cfg(has_abi_cdecl)](+ WithAbiImpl<abi::Fastcall>)
    #[cfg(has_abi_cdecl)](+ WithAbiImpl<abi::FastcallUnwind>)
    #[cfg(has_abi_cdecl)](+ WithAbiImpl<abi::Thiscall>)
    #[cfg(has_abi_cdecl)](+ WithAbiImpl<abi::ThiscallUnwind>)
    #[cfg(has_abi_vectorcall)](+ WithAbiImpl<abi::Vectorcall>)
    #[cfg(has_abi_vectorcall)](+ WithAbiImpl<abi::VectorcallUnwind>)
    #[cfg(has_abi_sysv64)](+ WithAbiImpl<abi::SysV64>)
    #[cfg(has_abi_sysv64)](+ WithAbiImpl<abi::SysV64Unwind>)
    #[cfg(has_abi_win64)](+ WithAbiImpl<abi::Win64>)
    #[cfg(has_abi_win64)](+ WithAbiImpl<abi::Win64Unwind>)
    #[cfg(has_abi_efiapi)](+ WithAbiImpl<abi::EfiApi>)
{
    /// The argument types as a tuple.
    type Args: Tuple;

    /// The return type.
    type Output;

    /// Marker type denoting safety
    type Safety: safety::Safety;

    /// Marker type denoting abi
    type Abi: abi::Abi;

    /// The function's arity (number of arguments).
    const ARITY: usize;

    /// Whether the function pointer is safe (fn) or unsafe (unsafe fn).
    const IS_SAFE: bool;

    /// Whether the function pointer uses an extern calling convention.
    const IS_EXTERN: bool;

    /// The abi associated with this function pointer.
    const ABI: AbiValue;

    /// Returns the address of this function.
    #[must_use]
    fn addr(&self) -> usize {
        self.as_ptr() as usize
    }
    /// Constructs an instance from an address.
    ///
    /// # Safety
    /// The given pointer has to point to a function of the correct type.
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
    /// The given pointer has to point to a function of the correct type.
    #[must_use]
    #[allow(clippy::missing_safety_doc)] // false positive?
    unsafe fn from_ptr(ptr: UntypedFnPtr) -> Self;

    /// Casts this function pointer to a different function pointer type.
    ///
    /// # Safety
    /// Caller must ensure that the resulting transformation is sound.
    #[must_use]
    unsafe fn cast<F: FnPtr>(&self) -> F {
        unsafe { FnPtr::from_ptr(self.as_ptr()) }
    }

    /// Produces an unsafe version of this function pointer.
    #[must_use]
    fn as_unsafe(&self) -> <Self as WithSafety<Unsafe>>::F {
        unsafe { FnPtr::from_ptr(self.as_ptr()) }
    }

    /// Produces a safe version of this function pointer.
    ///
    /// # Safety
    /// Caller must ensure the underlying function is actually safe to call.
    #[must_use]
    unsafe fn as_safe(&self) ->  <Self as WithSafety<Safe>>::F {
        self.cast()
    }

    /// Produces a version of this function pointer with the given safety.
    ///
    /// # Safety
    /// Caller must ensure that this function pointer is safe when casting to a safe function.
    #[must_use]
    unsafe fn with_safety<Safety: safety::Safety>(&self) -> <Self as WithSafety<Safety>>::F
    where
        Self: WithSafety<Safety>,
    {
        self.cast()
    }

    /// Produces a version of this function pointer with the given abi.
    ///
    /// # Safety
    /// Caller must ensure that the resulting abi transformation is sound.
    #[must_use]
    unsafe fn with_abi<Abi: abi::Abi>(&self) -> <Self as WithAbi<Abi>>::F
    where
        Self: WithAbi<Abi>,
    {
        self.cast()
    }

    /// Produces a version of this function pointer with the given return type.
    ///
    /// # Safety
    /// Caller must ensure that the resulting transformation is sound.
    #[must_use]
    unsafe fn with_output<Output>(&self) -> <Self as WithOutput<Output>>::F
    where
        Self: WithOutput<Output>,
    {
        self.cast()
    }

    /// Produces a version of this function pointer with the given return type.
    ///
    /// # Safety
    /// Caller must ensure that the resulting transformation is sound.
    #[must_use]
    unsafe fn with_args<Args: Tuple>(&self) -> <Self as WithArgs<Args>>::F
    where
        Self: WithArgs<Args>,
    {
        self.cast()
    }
}
}

/// Marker trait for all callable *safe* function pointer types (`fn` / `extern fn`).
pub trait SafeFnPtr: FnPtr<Safety = Safe> {
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

/// Marker trait for all callable *unsafe* function pointer types (`unsafe fn` / `unsafe extern fn`).
pub trait UnsafeFnPtr: FnPtr<Safety = Unsafe> {
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

/// Marker trait for all *static* function pointer types.
/// The return type and all parameter types have to be `'static`.
pub trait StaticFnPtr: FnPtr + 'static {}
impl<F: FnPtr + 'static> StaticFnPtr for F {}

#[cfg(test)]
#[allow(unused)]
mod test {
    use super::*;

    fn h<F: FnPtr>(f: F) -> crate::with_abi!("system", F) {
        unsafe { f.cast() }
    }
    fn f<F: FnPtr>(f: F) -> crate::with_safety!(unsafe, F) {
        unsafe { f.cast() }
    }
    fn j<F: FnPtr>(f: F) -> crate::with_output!(i32, F) {
        unsafe { f.cast() }
    }
    fn k<F: FnPtr>(f: F) -> crate::with_args!((i32,), F) {
        unsafe { f.cast() }
    }
}
