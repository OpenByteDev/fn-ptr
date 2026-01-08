use core::{
    fmt::{Debug, Pointer},
    hash::Hash,
    panic::{RefUnwindSafe, UnwindSafe},
};

use crate::{
    AsSafe, AsUnsafe, WithAbi, WithArgs, WithOutput, WithSafety, abi,
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
    #[cfg(nightly_build)]
    (+ core::marker::FnPtr)
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

    /// The ABI associated with this function pointer.
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
    fn as_unsafe(&self) -> <Self as AsUnsafe>::F
    where
        Self: AsUnsafe,
    {
        unsafe { FnPtr::from_ptr(self.as_ptr()) }
    }

    /// Produces a safe version of this function pointer.
    ///
    /// # Safety
    /// Caller must ensure the underlying function is actually safe to call.
    #[must_use]
    unsafe fn as_safe(&self) -> <Self as AsSafe>::F
    where
        Self: AsSafe,
    {
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

    /// Produces a version of this function pointer with the given ABI.
    ///
    /// # Safety
    /// Caller must ensure that the resulting ABI transformation is sound.
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
    unsafe fn with_ret<Output>(&self) -> <Self as WithOutput<Output>>::F
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
