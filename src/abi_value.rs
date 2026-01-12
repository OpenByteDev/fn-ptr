use core::{
    fmt::{self, Debug, Display},
    str::FromStr,
};

/// The abi or calling convention of a function pointer.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
// from https://github.com/rust-lang/rust/blob/4fa80a5e733e2202d7ca4c203c2fdfda41cfe7dc/compiler/rustc_abi/src/extern_abi.rs#L21
pub enum AbiValue {
    /* universal */
    /// This is the same as `extern fn foo()`; whatever the default your C compiler supports.
    C {
        /// Whether unwinding across this abi boundary is allowed (`*-unwind`).
        unwind: bool,
    },
    /// Usually the same as [`extern "C"`](AbiValue::C), except on Win32, in which case it's
    /// [`"stdcall"`](AbiValue::Stdcall), or what you should use to link to the Windows API itself.
    System {
        /// Whether unwinding across this abi boundary is allowed (`*-unwind`).
        unwind: bool,
    },

    /// The default abi when you write a normal `fn foo()` in any Rust code.
    Rust,

    /* arm */
    /// The default for ARM.
    Aapcs {
        /// Whether unwinding across this abi boundary is allowed (`*-unwind`).
        unwind: bool,
    },

    /* x86 */
    /// The default for `x86_32` C code.
    Cdecl {
        /// Whether unwinding across this abi boundary is allowed (`*-unwind`).
        unwind: bool,
    },
    /// The default for the Win32 API on `x86_32`.
    Stdcall {
        /// Whether unwinding across this abi boundary is allowed (`*-unwind`).
        unwind: bool,
    },
    /// The `fastcall` abi.
    Fastcall {
        /// Whether unwinding across this abi boundary is allowed (`*-unwind`).
        unwind: bool,
    },
    /// The Windows C++ abi.
    Thiscall {
        /// Whether unwinding across this abi boundary is allowed (`*-unwind`).
        unwind: bool,
    },
    /// The `vectorcall` abi.
    Vectorcall {
        /// Whether unwinding across this abi boundary is allowed (`*-unwind`).
        unwind: bool,
    },

    /* x86_64 */
    /// The default for C code on non-Windows `x86_64`.
    SysV64 {
        /// Whether unwinding across this abi boundary is allowed (`*-unwind`).
        unwind: bool,
    },
    /// The default for C code on `x86_64` Windows.
    Win64 {
        /// Whether unwinding across this abi boundary is allowed (`*-unwind`).
        unwind: bool,
    },
}

impl AbiValue {
    /// Returns whether unwinding after a panic is allowed inside the called function.
    #[must_use]
    pub const fn allows_unwind(&self) -> bool {
        match *self {
            AbiValue::Rust => true,
            AbiValue::C { unwind }
            | AbiValue::System { unwind }
            | AbiValue::Aapcs { unwind }
            | AbiValue::Cdecl { unwind }
            | AbiValue::Stdcall { unwind }
            | AbiValue::Fastcall { unwind }
            | AbiValue::Thiscall { unwind }
            | AbiValue::Vectorcall { unwind }
            | AbiValue::SysV64 { unwind }
            | AbiValue::Win64 { unwind } => unwind,
        }
    }

    /// Canonicalize this abi for the current target.
    ///
    /// Maps aliases (e.g. `system`, `cdecl`) to the concrete abi actually used on
    /// the current OS/architecture, following Rust compiler rules.
    ///
    /// Returns [`None`] if this abi is not supported on the current target.
    #[must_use]
    pub fn canonize(self, has_c_varargs: bool) -> Option<AbiValue> {
        // from https://github.com/rust-lang/rust/blob/4fa80a5e733e2202d7ca4c203c2fdfda41cfe7dc/compiler/rustc_target/src/spec/abi_map.rs#L79
        let os_windows = cfg!(target_os = "windows");
        let os_vexos = cfg!(target_os = "vexos");

        let arch_x86 = cfg!(target_arch = "x86");
        let arch_x86_64 = cfg!(target_arch = "x86_64");
        let arch_arm = cfg!(target_arch = "arm");
        let arch_aarch64 = cfg!(target_arch = "aarch64");
        let arch_arm_any = arch_arm || arch_aarch64;

        #[allow(clippy::match_same_arms)]
        let out = match self {
            AbiValue::C { unwind } => AbiValue::C { unwind },
            AbiValue::Rust => AbiValue::Rust,
            AbiValue::System { unwind } if arch_x86 && os_windows && !has_c_varargs => {
                AbiValue::Stdcall { unwind }
            }
            AbiValue::System { unwind } if arch_arm && os_vexos => AbiValue::Aapcs { unwind },
            AbiValue::System { unwind } => AbiValue::C { unwind },

            // arm
            AbiValue::Aapcs { unwind } if arch_arm_any => AbiValue::Aapcs { unwind },
            AbiValue::Aapcs { .. } => return None,

            // x86
            AbiValue::Cdecl { unwind } if arch_x86 => AbiValue::C { unwind },
            AbiValue::Cdecl { unwind } => AbiValue::C { unwind },

            AbiValue::Fastcall { unwind } if arch_x86 => AbiValue::Fastcall { unwind },
            AbiValue::Fastcall { unwind } if os_windows => AbiValue::C { unwind },
            AbiValue::Fastcall { .. } => return None,

            AbiValue::Stdcall { unwind } if arch_x86 => AbiValue::Stdcall { unwind },
            AbiValue::Stdcall { unwind } if os_windows => AbiValue::C { unwind },
            AbiValue::Stdcall { .. } => return None,

            AbiValue::Thiscall { unwind } if arch_x86 => AbiValue::Thiscall { unwind },
            AbiValue::Thiscall { .. } => return None,

            AbiValue::Vectorcall { unwind } if arch_x86 || arch_x86_64 => {
                AbiValue::Vectorcall { unwind }
            }
            AbiValue::Vectorcall { .. } => return None,

            AbiValue::SysV64 { unwind } if arch_x86_64 => AbiValue::SysV64 { unwind },
            AbiValue::Win64 { unwind } if arch_x86_64 => AbiValue::Win64 { unwind },
            AbiValue::SysV64 { .. } | AbiValue::Win64 { .. } => return None,
        };

        Some(out)
    }
}

impl Display for AbiValue {
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
            /// Returns the string representation of this abi.
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

abi_kind_impl!(AbiValue => {
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
