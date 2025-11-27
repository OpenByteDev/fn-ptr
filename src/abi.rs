#[cfg(nightly_build)]
use core::marker::ConstParamTy;
use core::{
    fmt::{Debug, Display},
    str::FromStr,
};

use const_panic::concat_panic;

/// The abi or calling convention of a function pointer.
#[repr(u8)]
#[cfg_attr(nightly_build, derive(ConstParamTy))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Abi {
    /// The default ABI when you write a normal `fn foo()` in any Rust code.
    Rust = 0,
    /// This is the same as `extern fn foo()`; whatever the default your C compiler supports.
    C = 1,
    /// Usually the same as [`extern "C"`](Abi::C), except on Win32, in which case it's [`"stdcall"`](Abi::Stdcall), or what you should use to link to the Windows API itself.
    System = 2,
    /// The default for C code on `x86_64` Windows.
    Win64 = 3,
    /// The default for C code on non-Windows `x86_64`.
    Sysv64 = 4,
    /// The default for ARM.
    Aapcs = 5,
    /// The default for `x86_32` C code.
    Cdecl = 6,
    /// The default for the Win32 API on `x86_32`.
    Stdcall = 7,
    /// The `fastcall` ABI -- corresponds to MSVC's `__fastcall` and GCC and clang's `__attribute__((fastcall))`
    Fastcall = 8,
    /// The `vectorcall` ABI -- corresponds to MSVC's `__vectorcall` and GCC and clang's `__attribute__((vectorcall))`
    Vectorcall = 9,
}

impl Abi {
    /// Returns the string representation of this ABI.
    #[must_use]
    pub const fn to_str(&self) -> &'static str {
        match self {
            Abi::Rust => "Rust",
            Abi::C => "C",
            Abi::System => "system",
            Abi::Win64 => "win64",
            Abi::Sysv64 => "sysv64",
            Abi::Aapcs => "aapcs",
            Abi::Cdecl => "cdecl",
            Abi::Stdcall => "stdcall",
            Abi::Fastcall => "fastcall",
            Abi::Vectorcall => "vectorcall",
        }
    }
}

impl Abi {
    /// Returns true if this ABI is an alias.
    #[must_use]
    pub fn is_alias(&self) -> bool {
        matches!(self, Abi::C | Abi::System)
    }

    /// Returns true if this ABI is a concrete ABI, not an alias.
    #[must_use]
    pub fn is_concrete(&self) -> bool {
        !self.is_alias()
    }

    /// Returns the concrete ABI for this ABI on the current target.
    #[must_use]
    pub fn concrete(&self) -> Abi {
        match self {
            Abi::C => {
                // "C" maps to platform default for C
                if cfg!(target_os = "windows") && cfg!(target_arch = "x86_64") {
                    Abi::Win64
                } else if cfg!(target_arch = "x86_64") {
                    Abi::Sysv64
                } else if cfg!(target_arch = "x86") {
                    Abi::Cdecl
                } else if cfg!(target_arch = "arm") || cfg!(target_arch = "aarch64") {
                    Abi::Aapcs
                } else {
                    Abi::Cdecl // fallback
                }
            }
            Abi::System => {
                if cfg!(target_os = "windows") && cfg!(target_arch = "x86_64") {
                    Abi::Win64
                } else if cfg!(target_os = "windows") && cfg!(target_arch = "x86") {
                    Abi::Stdcall
                } else if cfg!(target_arch = "x86_64") {
                    Abi::Sysv64
                } else if cfg!(target_arch = "x86") {
                    Abi::Cdecl
                } else if cfg!(target_arch = "arm") || cfg!(target_arch = "aarch64") {
                    Abi::Aapcs
                } else {
                    Abi::Cdecl // fallback
                }
            }
            other => *other,
        }
    }
}

impl FromStr for Abi {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" | "Rust" => Ok(Abi::Rust),
            "C" => Ok(Abi::C),
            "system" => Ok(Abi::System),
            "win64" => Ok(Abi::Win64),
            "sysv64" => Ok(Abi::Sysv64),
            "aapcs" => Ok(Abi::Aapcs),
            "cdecl" => Ok(Abi::Cdecl),
            "stdcall" => Ok(Abi::Stdcall),
            "fastcall" => Ok(Abi::Fastcall),
            "vectorcall" => Ok(Abi::Vectorcall),
            _ => Err(()),
        }
    }
}

impl Display for Abi {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

/// Parse a string into an [`Abi`] if known, otherwise returns `None`.
#[must_use]
pub const fn parse(conv: &'static str) -> Option<Abi> {
    if konst::eq_str(conv, "") || konst::eq_str(conv, "Rust") {
        Some(Abi::Rust)
    } else if konst::eq_str(conv, "C") {
        Some(Abi::C)
    } else if konst::eq_str(conv, "system") {
        Some(Abi::System)
    } else if konst::eq_str(conv, "win64") {
        Some(Abi::Win64)
    } else if konst::eq_str(conv, "sysv64") {
        Some(Abi::Sysv64)
    } else if konst::eq_str(conv, "aapcs") {
        Some(Abi::Aapcs)
    } else if konst::eq_str(conv, "cdecl") {
        Some(Abi::Cdecl)
    } else if konst::eq_str(conv, "stdcall") {
        Some(Abi::Stdcall)
    } else if konst::eq_str(conv, "fastcall") {
        Some(Abi::Fastcall)
    } else if konst::eq_str(conv, "vectorcall") {
        Some(Abi::Vectorcall)
    } else {
        None
    }
}

/// Parse a string into an [`Abi`] and panic if unknown.
#[must_use]
pub const fn parse_or_fail(conv: &'static str) -> Abi {
    if let Some(c) = parse(conv) {
        c
    } else {
        concat_panic!("invalid or unknown abi", conv)
    }
}

#[cfg(nightly_build)]
pub(crate) type AbiKey = Abi;
#[cfg(not(nightly_build))]
pub(crate) type AbiKey = u8;

#[must_use]
/// Returns the value used to designate the given ABI in const generics.
/// For stable or beta builds this returns an [`u8`], while on nightly the [`Abi`] instance is returned.
pub const fn key(abi: Abi) -> AbiKey {
    abi as _
}
