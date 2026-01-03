// NOTE: ABI target cfgs are provided by the build script as `has_abi_<name>`.

macro_rules! impl_fn {
    (@recurse () ($($nm:ident : $ty:ident),*)) => {
        impl_fn!(@impl_all ($($nm : $ty),*));
    };
    (@recurse ($hd_nm:ident : $hd_ty:ident $(, $tl_nm:ident : $tl_ty:ident)*) ($($nm:ident : $ty:ident),*)) => {
        impl_fn!(@impl_all ($($nm : $ty),*));
        impl_fn!(@recurse ($($tl_nm : $tl_ty),*) ($($nm : $ty,)* $hd_nm : $hd_ty));
    };

    (@impl_all ($($nm:ident : $ty:ident),*)) => {
        impl_fn!(@impl_u_and_s ($($nm : $ty),*), Rust, "Rust");
        impl_fn!(@impl_u_and_s ($($nm : $ty),*), C, "C");
        impl_fn!(@impl_u_and_s ($($nm : $ty),*), System, "system");
        #[cfg(has_abi_cdecl)]
        impl_fn!(@impl_u_and_s ($($nm : $ty),*), Cdecl, "cdecl");
        #[cfg(has_abi_stdcall)]
        impl_fn!(@impl_u_and_s ($($nm : $ty),*), Stdcall, "stdcall");
        #[cfg(has_abi_fastcall)]
        impl_fn!(@impl_u_and_s ($($nm : $ty),*), Fastcall, "fastcall");
        #[cfg(has_abi_win64)]
        impl_fn!(@impl_u_and_s ($($nm : $ty),*), Win64, "win64");
        #[cfg(has_abi_sysv64)]
        impl_fn!(@impl_u_and_s ($($nm : $ty),*), Sysv64, "sysv64");
        #[cfg(has_abi_aapcs)]
        impl_fn!(@impl_u_and_s ($($nm : $ty),*), Aapcs, "aapcs");
    };

    (@impl_u_and_s ($($nm:ident : $ty:ident),*), $abi_ident:ident, $abi_str:expr) => {
        impl_fn!(@impl_core ($($nm : $ty),*), extern $abi_str fn($($ty),*) -> Ret, true, $abi_ident, $abi_str);
        impl_fn!(@impl_core ($($nm : $ty),*), unsafe extern $abi_str fn($($ty),*) -> Ret, false, $abi_ident, $abi_str);
    };

    (@impl_core ($($nm:ident : $ty:ident),*), $fn_type:ty, $safety:tt, $abi_ident:ident, $call_conv:expr) => {
        #[automatically_derived]
        impl<Ret, $($ty),*> $crate::FnPtr for $fn_type {
            type Args = ($($ty,)*);
            type Output = Ret;

            const ARITY: ::core::primitive::usize = impl_fn!(@count ($($ty)*));
            const IS_SAFE: ::core::primitive::bool = $safety;
            const IS_EXTERN: ::core::primitive::bool = !matches!($crate::Abi::$abi_ident, $crate::Abi::Rust);
            const ABI: $crate::Abi = $crate::Abi::$abi_ident;

            fn as_ptr(&self) -> $crate::UntypedFnPtr {
                *self as $crate::UntypedFnPtr
            }
            unsafe fn from_ptr(ptr: $crate::UntypedFnPtr) -> Self {
                ::core::assert!(!ptr.is_null());
                unsafe { ::core::mem::transmute::<$crate::UntypedFnPtr, Self>(ptr) }
            }
        }
        impl_fn!(@impl_safe_fn_type ($($nm : $ty),*), $fn_type, $safety);

        // WithSafety
        #[automatically_derived]
        impl<Ret, $($ty),*> $crate::WithSafety<{true}> for $fn_type {
            type F = extern $call_conv fn($($ty),*) -> Ret;
        }
        #[automatically_derived]
        impl<Ret, $($ty),*> $crate::WithSafety<{false}> for $fn_type {
            type F = unsafe extern $call_conv fn($($ty),*) -> Ret;
        }

        // HasAbi
        #[automatically_derived]
        impl<Ret, $($ty),*> $crate::HasAbi<{$crate::abi!($call_conv)}> for $fn_type {}

        // WithAbi
        #[automatically_derived]
        impl<Ret, $($ty),*> $crate::WithAbi<{$crate::abi!("Rust")}> for $fn_type {
            type F = impl_fn!(@make_unsafe extern "Rust" fn($($ty),*) -> Ret, $safety);
        }
        #[automatically_derived]
        impl<Ret, $($ty),*> $crate::WithAbi<{$crate::abi!("C")}> for $fn_type {
            type F = impl_fn!(@make_unsafe extern "C" fn($($ty),*) -> Ret, $safety);
        }
        #[automatically_derived]
        impl<Ret, $($ty),*> $crate::WithAbi<{$crate::abi!("system")}> for $fn_type {
            type F = impl_fn!(@make_unsafe extern "system" fn($($ty),*) -> Ret, $safety);
        }
        #[cfg(has_abi_cdecl)]
        #[automatically_derived]
        impl<Ret, $($ty),*> $crate::WithAbi<{$crate::abi!("cdecl")}> for $fn_type {
            type F = impl_fn!(@make_unsafe extern "cdecl" fn($($ty),*) -> Ret, $safety);
        }
        #[cfg(has_abi_stdcall)]
        #[automatically_derived]
        impl<Ret, $($ty),*> $crate::WithAbi<{$crate::abi!("stdcall")}> for $fn_type {
            type F = impl_fn!(@make_unsafe extern "stdcall" fn($($ty),*) -> Ret, $safety);
        }
        #[cfg(has_abi_fastcall)]
        #[automatically_derived]
        impl<Ret, $($ty),*> $crate::WithAbi<{$crate::abi!("fastcall")}> for $fn_type {
            type F = impl_fn!(@make_unsafe extern "fastcall" fn($($ty),*) -> Ret, $safety);
        }
        #[cfg(has_abi_win64)]
        #[automatically_derived]
        impl<Ret, $($ty),*> $crate::WithAbi<{$crate::abi!("win64")}> for $fn_type {
            type F = impl_fn!(@make_unsafe extern "win64" fn($($ty),*) -> Ret, $safety);
        }
        #[cfg(has_abi_sysv64)]
        #[automatically_derived]
        impl<Ret, $($ty),*> $crate::WithAbi<{$crate::abi!("sysv64")}> for $fn_type {
            type F = impl_fn!(@make_unsafe extern "sysv64" fn($($ty),*) -> Ret, $safety);
        }
        #[cfg(has_abi_aapcs)]
        #[automatically_derived]
        impl<Ret, $($ty),*> $crate::WithAbi<{$crate::abi!("aapcs")}> for $fn_type {
            type F = impl_fn!(@make_unsafe extern "aapcs" fn($($ty),*) -> Ret, $safety);
        }
    };

    (@count ()) => {
        0
    };
    (@count ($hd:tt $($tl:tt)*)) => {
        1 + impl_fn!(@count ($($tl)*))
    };

    (@make_unsafe $fn_type:ty, true) => {
        $fn_type
    };
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

    (@call_args ($single:ident)) => {
        ($single,)
    };
    (@call_args ($($args:ident),*)) => {
        ($($args),*)
    };

    ($($nm:ident : $ty:ident),*) => {
        impl_fn!(@recurse ($($nm : $ty),*) ());
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
