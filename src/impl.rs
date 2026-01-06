// NOTE: ABI target cfgs are provided by the build script as `has_abi_<name>`.

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

        // Common platform ABIs
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
    };

    // call for safe and unsafe
    (@impl_u_and_s ($($nm:ident : $ty:ident),*), $abi_ident:ident, $abi_str:expr) => {
        impl_fn!(@impl_core ($($nm : $ty),*), extern $abi_str fn($($ty),*) -> Ret, true, $abi_ident, $abi_str);
        impl_fn!(@impl_core ($($nm : $ty),*), unsafe extern $abi_str fn($($ty),*) -> Ret, false, $abi_ident, $abi_str);
    };

    // core macro
    (@impl_core ($($nm:ident : $ty:ident),*), $fn_type:ty, $safety:tt, $abi_ident:ident, $call_conv:expr) => {
        #[automatically_derived]
        impl<Ret, $($ty),*> $crate::FnPtr for $fn_type {
            type Args = ($($ty,)*);
            type Output = Ret;

            type ArityMarker = impl_fn!(@arity_marker ($($ty),*));
            type SafetyMarker = $crate::safety!($safety);
            type AbiMarker = $crate::marker::$abi_ident;

            const ARITY: ::core::primitive::usize = <Self::ArityMarker as $crate::marker::Arity>::N;
            const IS_SAFE: ::core::primitive::bool = <Self::SafetyMarker as $crate::marker::Safety>::IS_SAFE;
            const ABI: $crate::Abi = <$crate::marker::$abi_ident as $crate::marker::Abi>::VALUE;
            const IS_EXTERN: ::core::primitive::bool = !matches!(Self::ABI, $crate::Abi::Rust);

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
        impl<Ret: 'static, $($ty: 'static),*> $crate::StaticFnPtr for $fn_type {
        }

        // WithSafety
        #[automatically_derived]
        impl<Ret, $($ty),*> $crate::WithSafety<$crate::marker::Safe> for $fn_type {
            type F = extern $call_conv fn($($ty),*) -> Ret;
        }
        #[automatically_derived]
        impl<Ret, $($ty),*> $crate::WithSafety<$crate::marker::Unsafe> for $fn_type {
            type F = unsafe extern $call_conv fn($($ty),*) -> Ret;
        }

        // HasAbi
        #[automatically_derived]
        impl<Ret, $($ty),*> $crate::HasAbi<$crate::marker::$abi_ident> for $fn_type {}

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
    };

    (@impl_withabi ($($nm:ident : $ty:ident),*), $fn_type:ty, $safety:tt, $abi:tt) => {
        #[automatically_derived]
        impl<Ret, $($ty),*> $crate::WithAbi<$crate::abi!($abi)> for $fn_type {
            type F = impl_fn!(@make_unsafe extern $abi fn($($ty),*) -> Ret, $safety);
        }
    };

    (@arity_marker ()) => { $crate::marker::A0 };
    (@arity_marker ($a:ty)) => { $crate::marker::A1 };
    (@arity_marker ($a:ty, $b:ty)) => { $crate::marker::A2 };
    (@arity_marker ($a:ty, $b:ty, $c:ty)) => { $crate::marker::A3 };
    (@arity_marker ($a:ty, $b:ty, $c:ty, $d:ty)) => { $crate::marker::A4 };
    (@arity_marker ($a:ty, $b:ty, $c:ty, $d:ty, $e:ty)) => { $crate::marker::A5 };
    (@arity_marker ($a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty)) => { $crate::marker::A6 };
    (@arity_marker ($a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty)) => { $crate::marker::A7 };
    (@arity_marker ($a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty)) => { $crate::marker::A8 };
    (@arity_marker ($a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty, $i:ty)) => { $crate::marker::A9 };
    (@arity_marker ($a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty, $i:ty, $j:ty)) => { $crate::marker::A10 };
    (@arity_marker ($a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty, $i:ty, $j:ty, $k:ty)) => { $crate::marker::A11 };
    (@arity_marker ($a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty, $i:ty, $j:ty, $k:ty, $l:ty)) => { $crate::marker::A12 };

    (@make_unsafe $fn_type:ty, true) => { $fn_type };
    (@make_unsafe extern $abi:literal fn($($args:ty),*) -> $ret:ty, false) => {
        unsafe extern $abi fn($($args),*) -> $ret
    };

    (@impl_safe_fn_type ($($nm:ident : $ty:ident),*), $fn_type:ty, true) => {
        #[automatically_derived]
        impl<Ret, $($ty),*> $crate::SafeFnPtr for $fn_type {
            fn invoke(&self, impl_fn!(@call_args ($($nm),*)): Self::Args) -> Self::Output {
                (*self)($($nm),*)
            }
        }
    };
    (@impl_safe_fn_type ($($nm:ident : $ty:ident),*), $fn_type:ty, false) => {
        #[automatically_derived]
        impl<Ret, $($ty),*> $crate::UnsafeFnPtr for $fn_type {
            unsafe fn invoke(&self, impl_fn!(@call_args ($($nm),*)): Self::Args) -> Self::Output {
                unsafe { (*self)($($nm),*) }
            }
        }
    };

    (@call_args ($single:ident)) => { ($single,) };
    (@call_args ($($args:ident),*)) => { ($($args),*) };
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
