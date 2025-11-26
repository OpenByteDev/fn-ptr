// NOTE: ABI target cfgs are provided by the build script as `has_abi_<name>`.

macro_rules! cfg_all {
    ($meta:meta { $($items:item)* }) => {
        $(#[cfg($meta)] $items)*
    };
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

    (@impl_core ($($nm:ident : $ty:ident),*), $fn_type:ty, true, $abi_ident:ident, $call_conv:expr) => {
        impl<Ret: 'static, $($ty: 'static),*> $crate::FnPtr for $fn_type {
            type Args = ($($ty,)*);
            type Output = Ret;

            const ARITY: ::core::primitive::usize = impl_fn!(@count ($($ty)*));
            const IS_SAFE: ::core::primitive::bool = true;
            const IS_EXTERN: ::core::primitive::bool = !matches!($crate::Abi::$abi_ident, $crate::Abi::Rust);
            const ABI: $crate::Abi = $crate::Abi::$abi_ident;
        }
        impl<Ret: 'static, $($ty: 'static),*> $crate::SafeFnPtr for $fn_type {}
        
        // WithSafety
        impl<Ret: 'static, $($ty: 'static),*> $crate::WithSafety<{true}, $fn_type> for $fn_type { type F = $fn_type; }
        impl<Ret: 'static, $($ty: 'static),*> $crate::WithSafety<{false}, $fn_type> for $fn_type { type F = unsafe extern $call_conv fn($($ty),*) -> Ret; }

        cfg_all!(feature = "nightly" {
            // HasAbi
            impl<Ret: 'static, $($ty: 'static),*> $crate::HasAbi<{$crate::Abi::$abi_ident}> for $fn_type {}

            // WithAbi
            impl<Ret: 'static, $($ty: 'static),*> $crate::WithAbi<{$crate::Abi::Rust}, $fn_type> for $fn_type { type F = extern "Rust" fn($($ty),*) -> Ret; }
            impl<Ret: 'static, $($ty: 'static),*> $crate::WithAbi<{$crate::Abi::C}, $fn_type> for $fn_type { type F = extern "C" fn($($ty),*) -> Ret; }
            impl<Ret: 'static, $($ty: 'static),*> $crate::WithAbi<{$crate::Abi::System}, $fn_type> for $fn_type { type F = extern "system" fn($($ty),*) -> Ret; }
            #[cfg(has_abi_cdecl)]
            impl<Ret: 'static, $($ty: 'static),*> $crate::WithAbi<{$crate::Abi::Cdecl}, $fn_type> for $fn_type { type F = extern "cdecl" fn($($ty),*) -> Ret; }
            #[cfg(has_abi_stdcall)]
            impl<Ret: 'static, $($ty: 'static),*> $crate::WithAbi<{$crate::Abi::Stdcall}, $fn_type> for $fn_type { type F = extern "stdcall" fn($($ty),*) -> Ret; }
            #[cfg(has_abi_fastcall)]
            impl<Ret: 'static, $($ty: 'static),*> $crate::WithAbi<{$crate::Abi::Fastcall}, $fn_type> for $fn_type { type F = extern "fastcall" fn($($ty),*) -> Ret; }
            #[cfg(has_abi_win64)]
            impl<Ret: 'static, $($ty: 'static),*> $crate::WithAbi<{$crate::Abi::Win64}, $fn_type> for $fn_type { type F = extern "win64" fn($($ty),*) -> Ret; }
            #[cfg(has_abi_sysv64)]
            impl<Ret: 'static, $($ty: 'static),*> $crate::WithAbi<{$crate::Abi::Sysv64}, $fn_type> for $fn_type { type F = extern "sysv64" fn($($ty),*) -> Ret; }
            #[cfg(has_abi_aapcs)]
            impl<Ret: 'static, $($ty: 'static),*> $crate::WithAbi<{$crate::Abi::Aapcs}, $fn_type> for $fn_type { type F = extern "aapcs" fn($($ty),*) -> Ret; }
        });
    };

    (@impl_core ($($nm:ident : $ty:ident),*), $fn_type:ty, false, $abi_ident:ident, $call_conv:expr) => {
        impl<Ret: 'static, $($ty: 'static),*> $crate::FnPtr for $fn_type {
            type Args = ($($ty,)*);
            type Output = Ret;

            const ARITY: ::core::primitive::usize = impl_fn!(@count ($($ty)*));
            const IS_SAFE: ::core::primitive::bool = false;
            const IS_EXTERN: ::core::primitive::bool = !matches!($crate::Abi::$abi_ident, $crate::Abi::Rust);
            const ABI: $crate::Abi = $crate::Abi::$abi_ident;
        }
        impl<Ret: 'static, $($ty: 'static),*> $crate::UnsafeFnPtr for $fn_type {}
        
        // WithSafety
        impl<Ret: 'static, $($ty: 'static),*> $crate::WithSafety<{true}, $fn_type> for $fn_type { type F = extern $call_conv fn($($ty),*) -> Ret; }
        impl<Ret: 'static, $($ty: 'static),*> $crate::WithSafety<{false}, $fn_type> for $fn_type { type F = $fn_type; }

        cfg_all!(feature = "nightly" {
            // HasAbi
            impl<Ret: 'static, $($ty: 'static),*> $crate::HasAbi<{$crate::Abi::$abi_ident}> for $fn_type {}
            
            // WithAbi
            impl<Ret: 'static, $($ty: 'static),*> $crate::WithAbi<{$crate::Abi::Rust}, $fn_type> for $fn_type { type F = unsafe extern "Rust" fn($($ty),*) -> Ret; }
            impl<Ret: 'static, $($ty: 'static),*> $crate::WithAbi<{$crate::Abi::C}, $fn_type> for $fn_type { type F = unsafe extern "C" fn($($ty),*) -> Ret; }
            impl<Ret: 'static, $($ty: 'static),*> $crate::WithAbi<{$crate::Abi::System}, $fn_type> for $fn_type { type F = unsafe extern "system" fn($($ty),*) -> Ret; }
            #[cfg(has_abi_cdecl)]
            impl<Ret: 'static, $($ty: 'static),*> $crate::WithAbi<{$crate::Abi::Cdecl}, $fn_type> for $fn_type { type F = unsafe extern "cdecl" fn($($ty),*) -> Ret; }
            #[cfg(has_abi_stdcall)]
            impl<Ret: 'static, $($ty: 'static),*> $crate::WithAbi<{$crate::Abi::Stdcall}, $fn_type> for $fn_type { type F = unsafe extern "stdcall" fn($($ty),*) -> Ret; }
            #[cfg(has_abi_fastcall)]
            impl<Ret: 'static, $($ty: 'static),*> $crate::WithAbi<{$crate::Abi::Fastcall}, $fn_type> for $fn_type { type F = unsafe extern "fastcall" fn($($ty),*) -> Ret; }
            #[cfg(has_abi_win64)]
            impl<Ret: 'static, $($ty: 'static),*> $crate::WithAbi<{$crate::Abi::Win64}, $fn_type> for $fn_type { type F = unsafe extern "win64" fn($($ty),*) -> Ret; }
            #[cfg(has_abi_sysv64)]
            impl<Ret: 'static, $($ty: 'static),*> $crate::WithAbi<{$crate::Abi::Sysv64}, $fn_type> for $fn_type { type F = unsafe extern "sysv64" fn($($ty),*) -> Ret; }
            #[cfg(has_abi_aapcs)]
            impl<Ret: 'static, $($ty: 'static),*> $crate::WithAbi<{$crate::Abi::Aapcs}, $fn_type> for $fn_type { type F = unsafe extern "aapcs" fn($($ty),*) -> Ret; }
        });
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

// Default: generate impls up to 6 arguments
impl_fn! {
    __arg_0:  A, __arg_1:  B, __arg_2:  C, __arg_3:  D, __arg_4:  E, __arg_5:  F
}

// Optional: generate impls up to 12 arguments when feature is enabled
#[cfg(feature = "max-arity-12")]
impl_fn! {
    __arg_0:  A, __arg_1:  B, __arg_2:  C, __arg_3:  D, __arg_4:  E, __arg_5:  F, __arg_6:  G,
    __arg_7:  H, __arg_8:  I, __arg_9:  J, __arg_10: K, __arg_11: L
}
