use alloc::boxed::Box;

use crate::event::Completion;
use crate::isa::current_isa::lp_control::{LpControl, LpControlIfce};
use crate::process_mgmt::ProcessId;
use crate::threading::thread::ThreadGuid;

#[repr(i32)]
pub enum Status {
    Success = 0,
    StructureAllocationFailure = 1,
}

pub trait LpLocalScheduler {
    extern "C" fn get_current_thread(&self) -> ThreadGuid;
    extern "C" fn get_next_thread(&self) -> ThreadGuid;
    extern "C" fn assign_thread(self: Box<Self>, thread: ThreadGuid) -> Status;
}

pub trait GlobalScheduler {
    fn block_thread(&self, thread: ThreadGuid, until: Completion);
    fn relinquish_blocked_thread(&self, thread: ThreadGuid);
    fn assign_thread(&self, thread: ThreadGuid) -> Status;
    fn abort_thread(&self, thread: ThreadGuid) -> Status;
    fn abort_process(&self, process: ProcessId) -> Status;
    fn terminate_thread(&self, thread: ThreadGuid) -> Status;
    fn terminate_process(&self, process: ProcessId) -> Status;
}
