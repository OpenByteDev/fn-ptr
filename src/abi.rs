use core::{
    fmt::{self, Debug, Display},
    str::FromStr,
};

/// The ABI or calling convention of a function pointer.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
// from https://github.com/rust-lang/rust/blob/4fa80a5e733e2202d7ca4c203c2fdfda41cfe7dc/compiler/rustc_abi/src/extern_abi.rs#L21
pub enum Abi {
    /* universal */
    /// This is the same as `extern fn foo()`; whatever the default your C compiler supports.
    C { unwind: bool },
    /// Usually the same as [`extern "C"`](Abi::C), except on Win32, in which case it's
    /// [`"stdcall"`](Abi::Stdcall), or what you should use to link to the Windows API itself.
    System { unwind: bool },

    /// The default ABI when you write a normal `fn foo()` in any Rust code.
    Rust,

    /* arm */
    /// The default for ARM.
    Aapcs { unwind: bool },

    /* x86 */
    /// The default for `x86_32` C code.
    Cdecl { unwind: bool },
    /// The default for the Win32 API on `x86_32`.
    Stdcall { unwind: bool },
    /// The `fastcall` ABI.
    Fastcall { unwind: bool },
    /// The Windows C++ ABI.
    Thiscall { unwind: bool },
    /// The `vectorcall` ABI.
    Vectorcall { unwind: bool },

    /* x86_64 */
    /// The default for C code on non-Windows `x86_64`.
    SysV64 { unwind: bool },
    /// The default for C code on `x86_64` Windows.
    Win64 { unwind: bool },
}

impl Abi {
    #[must_use]
    pub fn canonize(self, has_c_varargs: bool) -> Option<Abi> {
        // from https://github.com/rust-lang/rust/blob/4fa80a5e733e2202d7ca4c203c2fdfda41cfe7dc/compiler/rustc_target/src/spec/abi_map.rs#L79
        let os_windows = cfg!(target_os = "windows");
        let os_vexos = cfg!(target_os = "vexos");

        let arch_x86 = cfg!(target_arch = "x86");
        let arch_x86_64 = cfg!(target_arch = "x86_64");
        let arch_arm = cfg!(target_arch = "arm");
        let arch_aarch64 = cfg!(target_arch = "aarch64");
        let arch_arm_any = arch_arm || arch_aarch64;

        let out = match self {
            Abi::C { unwind } => Abi::C { unwind },
            Abi::Rust => Abi::Rust,
            Abi::System { unwind } if arch_x86 && os_windows && !has_c_varargs => {
                Abi::Stdcall { unwind }
            }
            Abi::System { unwind } if arch_arm && os_vexos => Abi::Aapcs { unwind },
            Abi::System { unwind } => Abi::C { unwind },

            // arm
            Abi::Aapcs { unwind } if arch_arm_any => Abi::Aapcs { unwind },
            Abi::Aapcs { .. } => return None,

            // x86
            Abi::Cdecl { unwind } if arch_x86 => Abi::C { unwind },
            Abi::Cdecl { unwind } => Abi::C { unwind },

            Abi::Fastcall { unwind } if arch_x86 => Abi::Fastcall { unwind },
            Abi::Fastcall { unwind } if os_windows => Abi::C { unwind },
            Abi::Fastcall { .. } => return None,

            Abi::Stdcall { unwind } if arch_x86 => Abi::Stdcall { unwind },
            Abi::Stdcall { unwind } if os_windows => Abi::C { unwind },
            Abi::Stdcall { .. } => return None,

            Abi::Thiscall { unwind } if arch_x86 => Abi::Thiscall { unwind },
            Abi::Thiscall { .. } => return None,

            Abi::Vectorcall { unwind } if arch_x86 || arch_x86_64 => Abi::Vectorcall { unwind },
            Abi::Vectorcall { .. } => return None,

            Abi::SysV64 { unwind } if arch_x86_64 => Abi::SysV64 { unwind },
            Abi::Win64 { unwind } if arch_x86_64 => Abi::Win64 { unwind },
            Abi::SysV64 { .. } | Abi::Win64 { .. } => return None,
        };

        Some(out)
    }
}

impl Display for Abi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

macro_rules! abi_kind_impl {
    (
        $t:ty => {
            $(
                $variant:ident $( { unwind: $uw:literal } )? => $tok:literal,
            )*
        }
    ) => {
        impl $t {
            /// Returns the string representation of this ABI.
            #[must_use]
            pub const fn to_str(&self) -> &'static str {
                match self {
                    $( Self::$variant $( { unwind: $uw } )? => $tok, )*
                }
            }

            /// The same as the [`FromStr`] implementation, but (only!) for use in `const` contexts.
            #[must_use]
            pub const fn from_str_const(conv: &'static str) -> Option<Self> {
                $(
                    if konst::eq_str(conv, $tok) {
                        return Some(Self::$variant $( { unwind: $uw } )?);
                    }
                )*
                None
            }
        }

        impl FromStr for $t {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $( $tok => Ok(Self::$variant $( { unwind: $uw } )?), )*
                    _ => Err(()),
                }
            }
        }
    };
}

abi_kind_impl!(Abi => {
    Rust => "Rust",

    C { unwind: false } => "C",
    C { unwind: true } => "C-unwind",
    System { unwind: false } => "system",
    System { unwind: true } => "system-unwind",

    Aapcs { unwind: false } => "aapcs",
    Aapcs { unwind: true } => "aapcs-unwind",

    Cdecl { unwind: false } => "cdecl",
    Cdecl { unwind: true } => "cdecl-unwind",
    Stdcall { unwind: false } => "stdcall",
    Stdcall { unwind: true } => "stdcall-unwind",
    Fastcall { unwind: false } => "fastcall",
    Fastcall { unwind: true } => "fastcall-unwind",
    Thiscall { unwind: false } => "thiscall",
    Thiscall { unwind: true } => "thiscall-unwind",
    Vectorcall { unwind: false } => "vectorcall",
    Vectorcall { unwind: true } => "vectorcall-unwind",

    SysV64 { unwind: false } => "sysv64",
    SysV64 { unwind: true } => "sysv64-unwind",
    Win64 { unwind: false } => "win64",
    Win64 { unwind: true } => "win64-unwind",
});
