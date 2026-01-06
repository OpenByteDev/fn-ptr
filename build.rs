use build_target::{Arch, Os};

fn main() {
    let version_meta = rustc_version::version_meta().unwrap();
    let use_nightly = cfg!(feature = "nightly") && !cfg!(feature = "stable")
        || version_meta.channel == rustc_version::Channel::Nightly;
    if use_nightly {
        cargo_emit::rustc_cfg!("nightly");
    }

    // from https://github.com/rust-lang/rust/blob/873122c006315e541c30809210089606877122c5/tests/ui/abi/unsupported.rs
    let t = build_target::target();

    if t.arch == Arch::X86 {
        cargo_emit::rustc_cfg!("has_abi_cdecl");
        cargo_emit::rustc_cfg!("has_abi_stdcall");
        cargo_emit::rustc_cfg!("has_abi_fastcall");
        cargo_emit::rustc_cfg!("has_abi_thiscall");
    }

    if matches!(t.arch, Arch::X86 | Arch::X86_64) && cfg!(feature = "abi_vectorcall") && use_nightly
    {
        cargo_emit::rustc_cfg!("has_abi_vectorcall");
    }

    if t.arch == Arch::X86_64 {
        if t.os == Os::Windows {
            cargo_emit::rustc_cfg!("has_abi_win64")
        } else {
            cargo_emit::rustc_cfg!("has_abi_sysv64")
        }
    }

    if t.arch == Arch::Arm {
        cargo_emit::rustc_cfg!("has_abi_aapcs");
    }
}
