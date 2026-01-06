pub trait Arity {
    const N: usize;
}
macro_rules! define_arity_marker {
    ($(($name:ident, $n:expr)),+ $(,)?) => {
        $(
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

pub trait Safety {
    const IS_SAFE: bool;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Safe;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Unsafe;

impl Safety for Safe {
    const IS_SAFE: bool = true;
}
impl Safety for Unsafe {
    const IS_SAFE: bool = false;
}

/// Type-level ABI marker trait.
///
/// Types implementing this trait represent a specific `extern "..."` ABI.
///
/// See [`Abi`] for the runtime representation.
pub trait Abi {
    /// The exact ABI string used in `extern "..."`.
    const STR: &'static str;

    /// The runtime [`Abi`] that represent this marker type.
    const KIND: crate::Abi;
}

/// Helper macro to implement [`Abi`].
macro_rules! define_abi_marker {
    ($name:ident, $lit:literal) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $name;

        impl Abi for $name {
            const STR: &'static str = $lit;
            const KIND: $crate::Abi = $crate::Abi::from_str_const($lit).unwrap();
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

#[doc(hidden)]
#[macro_export]
macro_rules! arity_marker {
    (0) => {
        $crate::markers::A0
    };
    (1) => {
        $crate::markers::A1
    };
    (2) => {
        $crate::markers::A2
    };
    (3) => {
        $crate::markers::A3
    };
    (4) => {
        $crate::markers::A4
    };
    (5) => {
        $crate::markers::A5
    };
    (6) => {
        $crate::markers::A6
    };
    (7) => {
        $crate::markers::A7
    };
    (8) => {
        $crate::markers::A8
    };
    (9) => {
        $crate::markers::A9
    };
    (10) => {
        $crate::markers::A10
    };
    (11) => {
        $crate::markers::A11
    };
    (12) => {
        $crate::markers::A12
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! abi_marker {
    // Rust
    ("Rust") => {
        $crate::markers::Rust
    };

    // Universal C / system
    ("C") => {
        $crate::markers::C
    };
    ("C-unwind") => {
        $crate::markers::CUnwind
    };
    ("system") => {
        $crate::markers::System
    };
    ("system-unwind") => {
        $crate::markers::SystemUnwind
    };

    // ARM
    ("aapcs") => {
        $crate::markers::Aapcs
    };
    ("aapcs-unwind") => {
        $crate::markers::AapcsUnwind
    };

    // x86 (32-bit)
    ("cdecl") => {
        $crate::markers::Cdecl
    };
    ("cdecl-unwind") => {
        $crate::markers::CdeclUnwind
    };
    ("stdcall") => {
        $crate::markers::Stdcall
    };
    ("stdcall-unwind") => {
        $crate::markers::StdcallUnwind
    };
    ("fastcall") => {
        $crate::markers::Fastcall
    };
    ("fastcall-unwind") => {
        $crate::markers::FastcallUnwind
    };
    ("thiscall") => {
        $crate::markers::Thiscall
    };
    ("thiscall-unwind") => {
        $crate::markers::ThiscallUnwind
    };
    ("vectorcall") => {
        $crate::markers::Vectorcall
    };
    ("vectorcall-unwind") => {
        $crate::markers::VectorcallUnwind
    };

    // x86_64
    ("sysv64") => {
        $crate::markers::SysV64
    };
    ("sysv64-unwind") => {
        $crate::markers::SysV64Unwind
    };
    ("win64") => {
        $crate::markers::Win64
    };
    ("win64-unwind") => {
        $crate::markers::Win64Unwind
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! safety_marker {
    (safe) => {
        $crate::markers::Safe
    };
    (unsafe) => {
        $crate::markers::Unsafe
    };
    (true) => {
        $crate::markers::Safe
    };
    (false) => {
        $crate::markers::Unsafe
    };
}

pub use crate::{abi_marker as abi, arity_marker as arity, safety_marker as safety};
