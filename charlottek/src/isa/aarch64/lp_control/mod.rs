use crate::isa::interface::lp;

type LpId = u32;

macro_rules! halt {
    unsafe {
        core::arch::asm!("wfi");
    }
    loop {}
}

macro_rules! mask_interrupts {
    unsafe {
        core::arch::asm!("cpsid i");
    }
}

macro_rules! unmask_interrupts {
    unsafe {
        core::arch::asm!("cpsie i");
    }
}
