use core::fmt::Debug;

pub mod abi;
pub use abi::Abi;

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

macro_rules! impl_fn {
    (@recurse () ($($nm:ident : $ty:ident),*)) => {
        impl_fn!(@impl_all ($($nm : $ty),*));
    };
    (@recurse ($hd_nm:ident : $hd_ty:ident $(, $tl_nm:ident : $tl_ty:ident)*) ($($nm:ident : $ty:ident),*)) => {
        impl_fn!(@impl_all ($($nm : $ty),*));
        impl_fn!(@recurse ($($tl_nm : $tl_ty),*) ($($nm : $ty,)* $hd_nm : $hd_ty));
    };

    (@impl_all ($($nm:ident : $ty:ident),*)) => {
        impl_fn!(@impl_core ($($nm : $ty),*), fn($($ty),*) -> Ret, true, false, "Rust");
        impl_fn!(@impl_core ($($nm : $ty),*), unsafe fn($($ty),*) -> Ret, false, false, "Rust");

        // Universal conventions
        impl_fn!(@impl_u_and_s ($($nm : $ty),*), "C");
        impl_fn!(@impl_u_and_s ($($nm : $ty),*), "system");

        // x86-specific conventions
        #[cfg(target_arch = "x86")]
        impl_fn!(@impl_u_and_s ($($nm : $ty),*), "cdecl");
        #[cfg(target_arch = "x86")]
        impl_fn!(@impl_u_and_s ($($nm : $ty),*), "stdcall");
        #[cfg(target_arch = "x86")]
        impl_fn!(@impl_u_and_s ($($nm : $ty),*), "fastcall");

        // x86_64 Windows
        #[cfg(all(target_arch = "x86_64", target_os = "windows"))]
        impl_fn!(@impl_u_and_s ($($nm : $ty),*), "win64");

        // x86_64 System V (Linux/macOS)
        #[cfg(all(target_arch = "x86_64", not(target_os = "windows")))]
        impl_fn!(@impl_u_and_s ($($nm : $ty),*), "sysv64");

        // ARM conventions
        #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
        impl_fn!(@impl_u_and_s ($($nm : $ty),*), "aapcs");
    };

    (@impl_u_and_s ($($nm:ident : $ty:ident),*), $abi:expr) => {
        impl_fn!(@impl_core ($($nm : $ty),*), extern $abi fn($($ty),*) -> Ret, true, true, $abi);
        impl_fn!(@impl_core ($($nm : $ty),*), unsafe extern $abi fn($($ty),*) -> Ret, false, true, $abi);
    };

    (@impl_core ($($nm:ident : $ty:ident),*), $fn_type:ty, $is_safe:expr, $is_extern:expr, $call_conv:expr) => {
        unsafe impl<Ret: 'static, $($ty: 'static),*> crate::FunctionPtr for $fn_type {
            type Args = ($($ty,)*);
            type Output = Ret;

            const ARITY: ::core::primitive::usize = impl_fn!(@count ($($ty)*));
            const SAFE: ::core::primitive::bool = $is_safe;
            const EXTERN: ::core::primitive::bool = $is_extern;
            const ABI: crate::Abi = crate::abi::parse_or_fail($call_conv);
        }
    };

    (@count ()) => {
        0
    };
    (@count ($hd:tt $($tl:tt)*)) => {
        1 + impl_fn!(@count ($($tl)*))
    };

    ($($nm:ident : $ty:ident),*) => {
        impl_fn!(@recurse ($($nm : $ty),*) ());
    };
}

impl_fn! {
    __arg_0:  A, __arg_1:  B, __arg_2:  C, __arg_3:  D, __arg_4:  E, __arg_5:  F, __arg_6:  G,
    __arg_7:  H, __arg_8:  I, __arg_9:  J, __arg_10: K, __arg_11: L
}
