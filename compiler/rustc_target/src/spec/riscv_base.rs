use crate::spec::abi::Abi;

// All the calling conventions trigger an assertion(Unsupported calling
// convention) in llvm on RISCV
pub fn unsupported_abis() -> Vec<Abi> {
    vec![
        Abi::Cdecl,
        Abi::Stdcall { unwind: false },
        Abi::Stdcall { unwind: true },
        Abi::Fastcall,
        Abi::Vectorcall,
        Abi::Thiscall { unwind: false },
        Abi::Thiscall { unwind: true },
        Abi::Aapcs,
        Abi::Win64,
        Abi::SysV64,
        Abi::PtxKernel,
        Abi::Msp430Interrupt,
        Abi::X86Interrupt,
        Abi::AmdGpuKernel,
    ]
}
