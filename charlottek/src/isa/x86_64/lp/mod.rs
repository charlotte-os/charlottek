use core::arch::asm;

use crate::isa::interface::lp::LpIfce;

pub mod thread_context;

pub struct LogicalProcessor;

impl LpIfce for LogicalProcessor {
    // MSR 0x802 value
    type LicId = u32;
    // TSC_AUX value
    type LpId = u32;

    #[inline(always)]
    fn halt() -> ! {
        unsafe { asm!("hlt") }
    }

    #[inline(always)]
    fn mask_interrupts() {
        unsafe { asm!("cli") }
    }

    #[inline(always)]
    fn unmask_interrupts() {
        unsafe { asm!("sti") }
    }

    #[inline(always)]
    fn curr_lic_id() -> Self::LicId {
        let mut lic_id: Self::LicId;
        unsafe {
            asm!(
                "rdmsr",
                "shl rdx, 32",
                "or rax, rdx",
                inlateout("ecx") 0x802 => _,
                out("rdx") _,
                out("rax") lic_id
            );
        }
        lic_id
    }

    #[inline(always)]
    fn curr_lp_id() -> Self::LicId {
        let mut lp_id: Self::LicId;
        unsafe {
            asm!(
                "rdpid {}",
                out(reg) lp_id
            );
        }
        lp_id
    }
}
