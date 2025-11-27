use build_target::{Arch, Os};

fn main() {
    let version_meta = rustc_version::version_meta().unwrap();
    if cfg!(feature = "nightly") && !cfg!(feature = "stable")
        || version_meta.channel == rustc_version::Channel::Nightly
    {
        cargo_emit::rustc_cfg!("nightly");
    }

    let t = build_target::target();

    // x86: cdecl, stdcall and fastcall
    if t.arch == Arch::X86 {
        cargo_emit::rustc_cfg!("has_abi_cdecl");
        cargo_emit::rustc_cfg!("has_abi_stdcall");
        cargo_emit::rustc_cfg!("has_abi_fastcall");
    }

    // x86_64: Windows uses the Win64 ABI, other OSes use the SysV AMD64 ABI
    if t.arch == Arch::X86_64 {
        if t.os == Os::Windows {
            cargo_emit::rustc_cfg!("has_abi_win64");
        } else {
            cargo_emit::rustc_cfg!("has_abi_sysv64");
        }
    }

    // ARM (32-bit) has aapcs
    if t.arch == Arch::Arm {
        cargo_emit::rustc_cfg!("has_abi_aapcs");
    }
}
