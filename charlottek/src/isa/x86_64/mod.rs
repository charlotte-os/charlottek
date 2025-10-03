//! x86_64
//!
//! This module provides an implementation of the ISA interface trait for the x86_64 architecture
//! also known as x64, AMD64, and Intel 64.

pub mod cpu_info;
pub mod init;
pub mod interrupts;
pub mod io;
pub mod lp;
pub mod memory;

pub struct X86_64;
impl crate::isa::interface::IsaIfce for X86_64 {
    type CpuInfoIfce = cpu_info::CpuInfo;
    type InitIfce = init::Init;
    type InterruptIfce = interrupts::Interrupts;
    type IoIfce = io::Io;
    type LpIfce = lp::LogicalProcessor;
    type MemoryIfce = memory::Memory;
}
