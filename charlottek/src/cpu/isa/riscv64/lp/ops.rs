#[macro_export]
macro_rules! halt {
    () => {
        unsafe { core::arch::asm!("wfi") }
    };
}

#[macro_export]
macro_rules! mask_interrupts {
    () => {
        unsafe { core::arch::asm!("csrci sstatus, 1") }
    };
}

#[macro_export]
macro_rules! unmask_interrupts {
    () => {
        unsafe { core::arch::asm!("csrsi sstatus, 1") }
    };
}

#[macro_export]
macro_rules! curr_lic_id {
    () => {
        0usize
    };
}
