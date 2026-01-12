// NOTE: abi target cfgs are provided by the build script as `has_abi_<name>`.

macro_rules! impl_fn {
    // main entry point
    ($($nm:ident : $ty:ident),*) => {
        impl_fn!(@recurse ($($nm : $ty),*) ());
    };

    // recurse for all parameter counts
    (@recurse () ($($nm:ident : $ty:ident),*)) => {
        impl_fn!(@impl_all ($($nm : $ty),*));
    };
    (@recurse ($hd_nm:ident : $hd_ty:ident $(, $tl_nm:ident : $tl_ty:ident)*) ($($nm:ident : $ty:ident),*)) => {
        impl_fn!(@impl_all ($($nm : $ty),*));
        impl_fn!(@recurse ($($tl_nm : $tl_ty),*) ($($nm : $ty,)* $hd_nm : $hd_ty));
    };

    // call for every abi
    (@impl_all ($($nm:ident : $ty:ident),*)) => {
        // Always-present
        impl_fn!(@impl_u_and_s ($($nm : $ty),*), Rust, "Rust");
        impl_fn!(@impl_u_and_s ($($nm : $ty),*), C, "C");
        impl_fn!(@impl_u_and_s ($($nm : $ty),*), CUnwind, "C-unwind");
        impl_fn!(@impl_u_and_s ($($nm : $ty),*), System, "system");
        impl_fn!(@impl_u_and_s ($($nm : $ty),*), SystemUnwind, "system-unwind");

        // Platform-specific ABIs
        #[cfg(has_abi_cdecl)]
        impl_fn!(@impl_u_and_s ($($nm : $ty),*), Cdecl, "cdecl");
        #[cfg(has_abi_cdecl)]
        impl_fn!(@impl_u_and_s ($($nm : $ty),*), CdeclUnwind, "cdecl-unwind");

        #[cfg(has_abi_stdcall)]
        impl_fn!(@impl_u_and_s ($($nm : $ty),*), Stdcall, "stdcall");
        #[cfg(has_abi_stdcall)]
        impl_fn!(@impl_u_and_s ($($nm : $ty),*), StdcallUnwind, "stdcall-unwind");

        #[cfg(has_abi_fastcall)]
        impl_fn!(@impl_u_and_s ($($nm : $ty),*), Fastcall, "fastcall");
        #[cfg(has_abi_fastcall)]
        impl_fn!(@impl_u_and_s ($($nm : $ty),*), FastcallUnwind, "fastcall-unwind");

        #[cfg(has_abi_thiscall)]
        impl_fn!(@impl_u_and_s ($($nm : $ty),*), Thiscall, "thiscall");
        #[cfg(has_abi_thiscall)]
        impl_fn!(@impl_u_and_s ($($nm : $ty),*), ThiscallUnwind, "thiscall-unwind");

        #[cfg(has_abi_vectorcall)]
        impl_fn!(@impl_u_and_s ($($nm : $ty),*), Vectorcall, "vectorcall");
        #[cfg(has_abi_vectorcall)]
        impl_fn!(@impl_u_and_s ($($nm : $ty),*), VectorcallUnwind, "vectorcall-unwind");

        #[cfg(has_abi_win64)]
        impl_fn!(@impl_u_and_s ($($nm : $ty),*), Win64, "win64");
        #[cfg(has_abi_win64)]
        impl_fn!(@impl_u_and_s ($($nm : $ty),*), Win64Unwind, "win64-unwind");

        #[cfg(has_abi_sysv64)]
        impl_fn!(@impl_u_and_s ($($nm : $ty),*), SysV64, "sysv64");
        #[cfg(has_abi_sysv64)]
        impl_fn!(@impl_u_and_s ($($nm : $ty),*), SysV64Unwind, "sysv64-unwind");

        #[cfg(has_abi_aapcs)]
        impl_fn!(@impl_u_and_s ($($nm : $ty),*), Aapcs, "aapcs");
        #[cfg(has_abi_aapcs)]
        impl_fn!(@impl_u_and_s ($($nm : $ty),*), AapcsUnwind, "aapcs-unwind");

        #[cfg(has_abi_efiapi)]
        impl_fn!(@impl_u_and_s ($($nm : $ty),*), EfiApi, "efiapi");
    };

    // call for safe and unsafe
    (@impl_u_and_s ($($nm:ident : $ty:ident),*), $abi_ident:ident, $abi_str:expr) => {
        impl_fn!(@impl_core ($($nm : $ty),*), extern $abi_str fn($($ty),*) -> Output, true, $abi_ident, $abi_str);
        impl_fn!(@impl_core ($($nm : $ty),*), unsafe extern $abi_str fn($($ty),*) -> Output, false, $abi_ident, $abi_str);
    };

    // core macro
    (@impl_core ($($nm:ident : $ty:ident),*), $fn_type:ty, $safety:tt, $abi_ident:ident, $call_conv:expr) => {
        #[automatically_derived]
        impl<Output, $($ty),*> $crate::FnPtr for $fn_type {
            type Args = ($($ty,)*);
            type Output = Output;

            type Safety = $crate::safety!($safety);
            type Abi = $crate::abi::$abi_ident;

            const ARITY: ::core::primitive::usize = <<Self::Args as $crate::tuple::Tuple>::Arity as $crate::arity::Arity>::N;
            const IS_SAFE: ::core::primitive::bool = <Self::Safety as $crate::safety::Safety>::IS_SAFE;
            const ABI: $crate::AbiValue = <$crate::abi::$abi_ident as $crate::abi::Abi>::VALUE;
            const IS_EXTERN: ::core::primitive::bool = !matches!(Self::ABI, $crate::AbiValue::Rust);

            fn as_ptr(&self) -> $crate::UntypedFnPtr {
                *self as $crate::UntypedFnPtr
            }
            unsafe fn from_ptr(ptr: $crate::UntypedFnPtr) -> Self {
                ::core::assert!(!ptr.is_null());
                unsafe { ::core::mem::transmute::<$crate::UntypedFnPtr, Self>(ptr) }
            }
        }
        impl_fn!(@impl_safe_fn_type ($($nm : $ty),*), $fn_type, $safety);

        #[automatically_derived]
        impl<Output, $($ty),*> $crate::BuildFn<$crate::safety!($safety), $crate::abi::$abi_ident, Output> for ($($ty,)*) {
            type F = impl_fn!(@make_unsafe extern $call_conv fn($($ty),*) -> Output, $safety);
        }

        // WithSafetyFrom
        #[automatically_derived]
        impl<Output, $($ty),*> $crate::WithSafetyImpl<$crate::safety::Safe> for $fn_type {
            type F = extern $call_conv fn($($ty),*) -> Output;
        }
        #[automatically_derived]
        impl<Output, $($ty),*> $crate::WithSafetyImpl<$crate::safety::Unsafe> for $fn_type {
            type F = unsafe extern $call_conv fn($($ty),*) -> Output;
        }

        // WithArgsFrom
        #[automatically_derived]
        impl<Output, $($ty),*> $crate::WithArgsImpl<$fn_type> for $fn_type {
            type F<Args: $crate::Tuple> = <<<Args::BaseFn as $crate::WithSafety<$crate::safety!($safety)>>::F as $crate::WithAbi<$crate::abi::$abi_ident>>::F as $crate::WithOutput<Output>>::F;
        }

        // WithRetFrom
        #[automatically_derived]
        impl<Output, $($ty),*> $crate::WithOutputImpl for $fn_type {
            type F<R> = impl_fn!(@make_unsafe extern $call_conv fn($($ty),*) -> R, $safety);
        }

        // WithAbi
        impl_fn!(@impl_withabi ($($nm : $ty),*), $fn_type, $safety, "Rust");
        impl_fn!(@impl_withabi ($($nm : $ty),*), $fn_type, $safety, "C");
        impl_fn!(@impl_withabi ($($nm : $ty),*), $fn_type, $safety, "system");
        impl_fn!(@impl_withabi ($($nm : $ty),*), $fn_type, $safety, "C-unwind");
        impl_fn!(@impl_withabi ($($nm : $ty),*), $fn_type, $safety, "system-unwind");

        #[cfg(has_abi_cdecl)]
        impl_fn!(@impl_withabi ($($nm : $ty),*), $fn_type, $safety, "cdecl");
        #[cfg(has_abi_cdecl)]
        impl_fn!(@impl_withabi ($($nm : $ty),*), $fn_type, $safety, "cdecl-unwind");

        #[cfg(has_abi_stdcall)]
        impl_fn!(@impl_withabi ($($nm : $ty),*), $fn_type, $safety, "stdcall");
        #[cfg(has_abi_stdcall)]
        impl_fn!(@impl_withabi ($($nm : $ty),*), $fn_type, $safety, "stdcall-unwind");

        #[cfg(has_abi_fastcall)]
        impl_fn!(@impl_withabi ($($nm : $ty),*), $fn_type, $safety, "fastcall");
        #[cfg(has_abi_fastcall)]
        impl_fn!(@impl_withabi ($($nm : $ty),*), $fn_type, $safety, "fastcall-unwind");

        #[cfg(has_abi_thiscall)]
        impl_fn!(@impl_withabi ($($nm : $ty),*), $fn_type, $safety, "thiscall");
        #[cfg(has_abi_thiscall)]
        impl_fn!(@impl_withabi ($($nm : $ty),*), $fn_type, $safety, "thiscall-unwind");

        #[cfg(has_abi_vectorcall)]
        impl_fn!(@impl_withabi ($($nm : $ty),*), $fn_type, $safety, "vectorcall");
        #[cfg(has_abi_vectorcall)]
        impl_fn!(@impl_withabi ($($nm : $ty),*), $fn_type, $safety, "vectorcall-unwind");

        #[cfg(has_abi_win64)]
        impl_fn!(@impl_withabi ($($nm : $ty),*), $fn_type, $safety, "win64");
        #[cfg(has_abi_win64)]
        impl_fn!(@impl_withabi ($($nm : $ty),*), $fn_type, $safety, "win64-unwind");

        #[cfg(has_abi_sysv64)]
        impl_fn!(@impl_withabi ($($nm : $ty),*), $fn_type, $safety, "sysv64");
        #[cfg(has_abi_sysv64)]
        impl_fn!(@impl_withabi ($($nm : $ty),*), $fn_type, $safety, "sysv64-unwind");

        #[cfg(has_abi_aapcs)]
        impl_fn!(@impl_withabi ($($nm : $ty),*), $fn_type, $safety, "aapcs");
        #[cfg(has_abi_aapcs)]
        impl_fn!(@impl_withabi ($($nm : $ty),*), $fn_type, $safety, "aapcs-unwind");

        #[cfg(has_abi_efiapi)]
        impl_fn!(@impl_withabi ($($nm : $ty),*), $fn_type, $safety, "efiapi");
    };

    (@impl_withabi ($($nm:ident : $ty:ident),*), $fn_type:ty, $safety:tt, $abi:tt) => {
        #[automatically_derived]
        impl<Output, $($ty),*> $crate::WithAbiImpl<$crate::abi!($abi)> for $fn_type {
            type F = impl_fn!(@make_unsafe extern $abi fn($($ty),*) -> Output, $safety);
        }
    };

    (@make_unsafe $fn_type:ty, true) => { $fn_type };
    (@make_unsafe extern $abi:literal fn($($args:ty),*) -> $Output:ty, false) => {
        unsafe extern $abi fn($($args),*) -> $Output
    };

    (@impl_safe_fn_type ($($nm:ident : $ty:ident),*), $fn_type:ty, true) => {
        #[automatically_derived]
        impl<Output, $($ty),*> $crate::SafeFnPtr for $fn_type {
            fn invoke(&self, ($($nm,)*): Self::Args) -> Self::Output {
                (*self)($($nm),*)
            }
        }
    };
    (@impl_safe_fn_type ($($nm:ident : $ty:ident),*), $fn_type:ty, false) => {
        #[automatically_derived]
        impl<Output, $($ty),*> $crate::UnsafeFnPtr for $fn_type {
            unsafe fn invoke(&self, ($($nm,)*): Self::Args) -> Self::Output {
                unsafe { (*self)($($nm),*) }
            }
        }
    };
}

// Default: generate impls up to 6 arguments
#[cfg(not(feature = "max-arity-12"))]
impl_fn! {
    __arg_0: A, __arg_1: B, __arg_2: C, __arg_3: D, __arg_4: E, __arg_5: F
}

// Optional: generate impls up to 12 arguments when feature is enabled
#[cfg(feature = "max-arity-12")]
impl_fn! {
    __arg_0: A, __arg_1: B, __arg_2: C, __arg_3: D, __arg_4: E, __arg_5: F, __arg_6: G,
    __arg_7: H, __arg_8: I, __arg_9: J, __arg_10: K, __arg_11: L
}
