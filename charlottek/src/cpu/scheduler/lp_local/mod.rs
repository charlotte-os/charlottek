mod darr;
mod simple_rr;

use alloc::vec::Vec;

use crate::cpu::threads::{Thread, ThreadId};
use crate::isa::lp::LogicalProcessor;
use crate::memory::AddressSpaceId;

pub trait LpScheduler {
    extern "C" fn advance(&self);

    fn assign_ready(&mut self, thread: Thread);

    fn terminate_threads(&mut self, thread_ids: Vec<ThreadId>);

    fn abort_threads(&mut self, thread_ids: Vec<ThreadId>);

    fn abort_as_threads(&mut self, asid: AddressSpaceId);

    fn asid_to_hw_asid(&self, asid: AddressSpaceId) -> Option<LogicalProcessor::HwAsid>;

    fn hw_asid_to_asid(&self, hw_asid: LogicalProcessor::HwAsid) -> Option<AddressSpaceId>;
}
