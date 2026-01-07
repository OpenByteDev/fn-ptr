/// Type-level marker trait for function arity, from [`A0`] to [`A12`].
pub trait Arity {
    /// Number of parameters for this arity.
    const N: usize;
}
macro_rules! define_arity_marker {
    ($(($name:ident, $n:expr)),+ $(,)?) => {
        $(
            #[doc = "Type-level marker for functions with exactly "]
            #[doc = stringify!($n)]
            #[doc = " parameters."]
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub struct $name;

            impl Arity for $name {
                const N: usize = $n;
            }
        )+
    };
}
define_arity_marker!(
    (A0, 0),
    (A1, 1),
    (A2, 2),
    (A3, 3),
    (A4, 4),
    (A5, 5),
    (A6, 6),
    (A7, 7),
    (A8, 8),
    (A9, 9),
    (A10, 10),
    (A11, 11),
    (A12, 12),
);

/// Type-level marker trait for function safety, either [`Safe`] or [`Unsafe`].
pub trait Safety {
    /// `true` for safe functions, `false` for unsafe ones.
    const IS_SAFE: bool;
}

/// Marker type for safe functions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Safe;
/// Marker type for unsafe functions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Unsafe;

impl Safety for Safe {
    const IS_SAFE: bool = true;
}
impl Safety for Unsafe {
    const IS_SAFE: bool = false;
}

/// Type-level marker trait for function ABI.
///
/// Types implementing this trait represent a specific `extern "..."` ABI.
///
/// See [`Abi`] for the runtime representation.
pub trait Abi {
    /// The exact ABI string used in `extern "..."`.
    const STR: &'static str;

    /// The runtime [`Abi`] that represent this marker type.
    const VALUE: crate::Abi;

    /// The runtime [`Abi`] that represent this marker type.
    const ALLOWS_UNWIND: bool = Self::VALUE.allows_unwind();
}

/// Helper macro to implement [`Abi`].
macro_rules! define_abi_marker {
    ($name:ident, $lit:literal) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[doc = "Type-level marker for the `"]
        #[doc = $lit]
        #[doc = "` ABI."]
        pub struct $name;

        impl Abi for $name {
            const STR: &'static str = $lit;
            const VALUE: $crate::Abi = $crate::Abi::from_str_const($lit).unwrap();
        }
    };
}

// Universal
define_abi_marker!(C, "C");
define_abi_marker!(CUnwind, "C-unwind");
define_abi_marker!(System, "system");
define_abi_marker!(SystemUnwind, "system-unwind");

// Rust
define_abi_marker!(Rust, "Rust");

// ARM
define_abi_marker!(Aapcs, "aapcs");
define_abi_marker!(AapcsUnwind, "aapcs-unwind");

// x86
define_abi_marker!(Cdecl, "cdecl");
define_abi_marker!(CdeclUnwind, "cdecl-unwind");
define_abi_marker!(Stdcall, "stdcall");
define_abi_marker!(StdcallUnwind, "stdcall-unwind");
define_abi_marker!(Fastcall, "fastcall");
define_abi_marker!(FastcallUnwind, "fastcall-unwind");
define_abi_marker!(Thiscall, "thiscall");
define_abi_marker!(ThiscallUnwind, "thiscall-unwind");
define_abi_marker!(Vectorcall, "vectorcall");
define_abi_marker!(VectorcallUnwind, "vectorcall-unwind");

// x86_64
define_abi_marker!(SysV64, "sysv64");
define_abi_marker!(SysV64Unwind, "sysv64-unwind");
define_abi_marker!(Win64, "win64");
define_abi_marker!(Win64Unwind, "win64-unwind");

/// Macro to convert an integral number to the corresponding [`Arity`] marker type.
#[macro_export]
macro_rules! arity {
    (0) => {
        $crate::marker::A0
    };
    (1) => {
        $crate::marker::A1
    };
    (2) => {
        $crate::marker::A2
    };
    (3) => {
        $crate::marker::A3
    };
    (4) => {
        $crate::marker::A4
    };
    (5) => {
        $crate::marker::A5
    };
    (6) => {
        $crate::marker::A6
    };
    (7) => {
        $crate::marker::A7
    };
    (8) => {
        $crate::marker::A8
    };
    (9) => {
        $crate::marker::A9
    };
    (10) => {
        $crate::marker::A10
    };
    (11) => {
        $crate::marker::A11
    };
    (12) => {
        $crate::marker::A12
    };
}

/// Macro to convert an ABI string to the corrsponding [`Abi`] marker type.
#[macro_export]
macro_rules! abi {
    // Common
    ("Rust") => {
        $crate::marker::Rust
    };
    ("C") => {
        $crate::marker::C
    };
    ("C-unwind") => {
        $crate::marker::CUnwind
    };
    ("system") => {
        $crate::marker::System
    };
    ("system-unwind") => {
        $crate::marker::SystemUnwind
    };

    // ARM
    ("aapcs") => {
        $crate::marker::Aapcs
    };
    ("aapcs-unwind") => {
        $crate::marker::AapcsUnwind
    };

    // x86
    ("cdecl") => {
        $crate::marker::Cdecl
    };
    ("cdecl-unwind") => {
        $crate::marker::CdeclUnwind
    };
    ("stdcall") => {
        $crate::marker::Stdcall
    };
    ("stdcall-unwind") => {
        $crate::marker::StdcallUnwind
    };
    ("fastcall") => {
        $crate::marker::Fastcall
    };
    ("fastcall-unwind") => {
        $crate::marker::FastcallUnwind
    };
    ("thiscall") => {
        $crate::marker::Thiscall
    };
    ("thiscall-unwind") => {
        $crate::marker::ThiscallUnwind
    };
    ("vectorcall") => {
        $crate::marker::Vectorcall
    };
    ("vectorcall-unwind") => {
        $crate::marker::VectorcallUnwind
    };

    // x86_64
    ("sysv64") => {
        $crate::marker::SysV64
    };
    ("sysv64-unwind") => {
        $crate::marker::SysV64Unwind
    };
    ("win64") => {
        $crate::marker::Win64
    };
    ("win64-unwind") => {
        $crate::marker::Win64Unwind
    };
}

/// Macro to convert a safety token (`safe` or `unsafe`) or a boolean literal to the corrsponding [`Safety`] marker type.
#[macro_export]
macro_rules! safety {
    (safe) => {
        $crate::marker::Safe
    };
    (unsafe) => {
        $crate::marker::Unsafe
    };
    (true) => {
        $crate::marker::Safe
    };
    (false) => {
        $crate::marker::Unsafe
    };
}
