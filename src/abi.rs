use crate::AbiValue;

/// Type-level marker trait for function abi.
///
/// Types implementing this trait represent a specific `extern "..."` abi.
///
/// See [`Abi`] for the runtime representation.
pub trait Abi {
    /// The exact abi string used in `extern "..."`.
    const STR: &'static str;

    /// The runtime [`Abi`] that represent this marker type.
    const VALUE: AbiValue;

    /// The runtime [`Abi`] that represent this marker type.
    const ALLOWS_UNWIND: bool = Self::VALUE.allows_unwind();
}

/// Helper macro to implement [`Abi`].
macro_rules! define_abi_marker {
    ($name:ident, $lit:literal) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[doc = "Type-level marker for the `"]
        #[doc = $lit]
        #[doc = "` abi."]
        pub struct $name;

        impl Abi for $name {
            const STR: &'static str = $lit;
            const VALUE: AbiValue = AbiValue::from_str_const($lit).unwrap();
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

/// Macro to convert an abi string to the corresponding [`Abi`] marker type.
#[macro_export]
macro_rules! abi {
    // Common
    ("Rust") => {
        $crate::abi::Rust
    };
    ("C") => {
        $crate::abi::C
    };
    ("C-unwind") => {
        $crate::abi::CUnwind
    };
    ("system") => {
        $crate::abi::System
    };
    ("system-unwind") => {
        $crate::abi::SystemUnwind
    };

    // ARM
    ("aapcs") => {
        $crate::abi::Aapcs
    };
    ("aapcs-unwind") => {
        $crate::abi::AapcsUnwind
    };

    // x86
    ("cdecl") => {
        $crate::abi::Cdecl
    };
    ("cdecl-unwind") => {
        $crate::abi::CdeclUnwind
    };
    ("stdcall") => {
        $crate::abi::Stdcall
    };
    ("stdcall-unwind") => {
        $crate::abi::StdcallUnwind
    };
    ("fastcall") => {
        $crate::abi::Fastcall
    };
    ("fastcall-unwind") => {
        $crate::abi::FastcallUnwind
    };
    ("thiscall") => {
        $crate::abi::Thiscall
    };
    ("thiscall-unwind") => {
        $crate::abi::ThiscallUnwind
    };
    ("vectorcall") => {
        $crate::abi::Vectorcall
    };
    ("vectorcall-unwind") => {
        $crate::abi::VectorcallUnwind
    };

    // x86_64
    ("sysv64") => {
        $crate::abi::SysV64
    };
    ("sysv64-unwind") => {
        $crate::abi::SysV64Unwind
    };
    ("win64") => {
        $crate::abi::Win64
    };
    ("win64-unwind") => {
        $crate::abi::Win64Unwind
    };
}
