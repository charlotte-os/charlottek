use alloc::vec::Vec;

use crate::cpu::threads::{Thread, ThreadId};
use crate::isa::lp::LpControl;
use crate::memory::AddressSpaceId;

pub trait LpScheduler {
    extern "C" fn advance(&self);

    fn mark_ready(&mut self, thread: Thread);

    fn terminate_threads(&mut self, thread_ids: Vec<ThreadId>);

    fn abort_threads(&mut self, thread_ids: Vec<ThreadId>);

    fn abort_as_all(&mut self, asid: AddressSpaceId);

    fn asid_to_pcid(&self, asid: AddressSpaceId) -> Option<u16>;
}
