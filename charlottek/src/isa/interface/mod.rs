pub mod cpu_info;
pub mod init;
pub mod interrupts;
pub mod io;
pub mod lp;
pub mod memory;

/// This trait defines the ISA abstraction used by this kernel.
/// Implementing it correctly allows this kernel to interface with a given ISA.
pub trait IsaIfce {
    type InitIfce: init::InitIfce;
    type InterruptIfce: interrupts::InterruptIfce;
    type IoIfce: io::IoIfce;
    type LpIfce: lp::LpIfce;
    type MemoryIfce: memory::MemoryIfce;
    type SystemInfoIfce: cpu_info::CpuInfoIfce;
}
