use crate::cpu::isa::interface::lp::LpIfce;
use crate::cpu::isa::lp::LogicalProcessor;
pub trait InterruptIfce {
    type Error;
    type Ipi;

    fn init_interrupt_structures() -> Result<(), Self::Error>;
    fn send_ipi(lp_list: &[<LogicalProcessor as LpIfce>::LpId], ipi: Self::Ipi);
}
