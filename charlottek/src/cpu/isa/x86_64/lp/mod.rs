mod pcid;
pub mod thread_context;

use core::arch::asm;

pub use pcid::Pcid;

pub use crate::cpu::isa::interface::lp::LpIfce;

pub struct LogicalProcessor;

impl LpIfce for LogicalProcessor {
    // PCID, stored in the low 12 bits of CR3
    type HwAsid = Pcid;
    // obtained from MSR 0x802
    type LicId = u32;
    // kernel assigned, stored in TSC_AUX
    type LpId = u32;

    #[inline(always)]
    fn halt() -> ! {
    //! Halts the calling processor to wait for any unmasked interrupts
        unsafe {
            asm!(
                "hlt",
                options(noreturn)        
            )
        }
    }

    #[inline(always)]
    fn mask_interrupts() {
    //! Clears the interrupt enable bit in RFLAGS
        unsafe { asm!("cli") }
    }

    #[inline(always)]
    fn unmask_interrupts() {
    //! Sets the interrupt enable bit in RFLAGS
        unsafe { asm!("sti") }
    }

    #[inline(always)]
    fn read_lic_id() -> Self::LicId {
    //! Obtains the local x2APIC identifier for the calling processor
        let mut lic_id: Self::LicId;
        unsafe {
            asm!(
                "rdmsr",
                inlateout("ecx") 0x802 => _,
                out("edx") _,
                out("rax") lic_id
            );
        }
        lic_id
    }

    #[inline(always)]
    fn write_lp_id(lp_id: Self::LpId) {
        unsafe {
            asm!(
                "wrmsr",
                in("ecx") 0xC000_0103u32,
                in("eax") lp_id as u32,
                in("edx") 0u32,
                options(nostack, preserves_flags)
            );
        }
    }

    #[inline(always)]
    fn read_lp_id() -> Self::LicId {
    //! Obtains the kernel assigned logical processor identifier
        let mut lp_id: Self::LicId;
        unsafe {
            asm!(
                "rdpid {:r}",
                out(reg) lp_id
            );
        }
        lp_id
    }
}
