pub mod context_switch;
pub mod exceptions;
pub mod fixed;
pub mod idt;
pub mod ipis;
pub mod x2apic;

pub use fixed::*;
use idt::*;
use spin::Mutex;

pub static BSP_IDT: Mutex<Idt> = Mutex::new(Idt::new());

pub struct Interrupts;

impl crate::isa::interface::interrupts::InterruptIfce for Interrupts {
    type Error = core::convert::Infallible;
    type Ipi = ();

    fn init_interrupt_structures() -> Result<(), Self::Error> {
        let mut idt = BSP_IDT.lock();
        register_fixed_isr_gates(&mut idt);
        Ok(())
    }

    fn send_ipi(
        lp_list: &[<super::lp::LogicalProcessor as crate::isa::interface::lp::LpIfce>::LpId],
        ipi: Self::Ipi,
    ) {
        todo!("Implement IPI sending");
    }
}
