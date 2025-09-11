use alloc::alloc::Global;
use alloc::boxed::Box;
use alloc::collections::btree_map::BTreeMap;
use alloc::vec::Vec;

use crate::event::Completion;
use crate::isa::x86_64::lp_control::LpControl;
use crate::isa::x86_64::lp_control::thread::ThreadId;

pub enum Error {
    AtMaxRunningThreads,
    AllExecutionQueuesFull,
    InactiveThreadId,
}

#[repr(u8)]
pub enum SchedulerImplementation {
    SimpleRoundRobin = 0,
    ProportionalRoundRobin = 1,
    Pdarr = 2, // Proportional Demand Attenuated Round Robin
}

pub struct GlobalScheduler {
    active_thread_count: usize,
    blocked_set: BTreeMap<ThreadId, Box<Completion>>,
    available_thread_ids: Vec<ThreadId>,
}
