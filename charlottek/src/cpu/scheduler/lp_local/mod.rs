mod darr;
mod simple_rr;

use alloc::vec::Vec;

use crate::cpu::isa::lp::{LogicalProcessor, LpIfce};
use crate::cpu::threads::{Thread, ThreadId};
use crate::memory::AddressSpaceId;

type HwAsid = <LogicalProcessor as LpIfce>::HwAsid;

pub trait LpScheduler: Send {
    extern "C" fn advance(&self);

    fn assign_ready(&mut self, thread: Thread);

    fn terminate_threads(&mut self, thread_ids: Vec<ThreadId>);

    fn abort_threads(&mut self, thread_ids: Vec<ThreadId>);

    fn abort_as_threads(&mut self, asid: AddressSpaceId);

    fn asid_to_hw_asid(&self, asid: AddressSpaceId) -> Option<HwAsid>;

    fn hw_asid_to_asid(&self, hw_asid: HwAsid) -> Option<AddressSpaceId>;
}
