use core::{
    fmt::{Debug, Display},
    str::FromStr,
    marker::ConstParamTy
};

use const_panic::concat_panic;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, ConstParamTy)]
/// The abi or calling convention of a function pointer.
pub enum Abi {
    /// The default ABI when you write a normal `fn foo()` in any Rust code.
    Rust,
    /// This is the same as `extern fn foo()`; whatever the default your C compiler supports.
    C,
    /// Usually the same as [`extern "C"`](Abi::C), except on Win32, in which case it's [`"stdcall"`](Abi::Stdcall), or what you should use to link to the Windows API itself.
    System,
    /// The default for C code on x86_64 Windows.
    Win64,
    /// The default for C code on non-Windows x86_64.
    Sysv64,
    /// The default for ARM.
    Aapcs,
    /// The default for x86_32 C code.
    Cdecl,
    /// The default for the Win32 API on x86_32.
    Stdcall,
    /// The `fastcall` ABI -- corresponds to MSVC's `__fastcall` and GCC and clang's `__attribute__((fastcall))`
    Fastcall,
    /// The `vectorcall` ABI -- corresponds to MSVC's `__vectorcall` and GCC and clang's `__attribute__((vectorcall))`
    Vectorcall,
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

impl FromStr for Abi {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" | "Rust" => Ok(Abi::C),
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

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

#[must_use]
pub const fn parse_or_fail(conv: &'static str) -> Abi {
    match parse(conv) {
        Some(c) => c,
        None => concat_panic!("invalid or unknown abi", conv),
    }
}
