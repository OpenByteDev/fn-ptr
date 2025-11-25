use build_target::{Arch, Os};

fn main() {
    let t = build_target::target().unwrap();

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
    if t.arch == Arch::ARM {
        cargo_emit::rustc_cfg!("has_abi_aapcs");
    }
}
