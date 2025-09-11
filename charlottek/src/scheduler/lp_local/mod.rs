use alloc::vec::Vec;
use core::hash::Hash;
use core::time::Duration;

pub use crate::isa::current_isa::lp_control::thread::{Thread, ThreadId};
use crate::memory::vmem::AddressSpaceId;

pub enum Error {
    ThreadQueueFull,
    ThreadNotInQueue,
}

pub enum LpLocalSchedComponent {
    Sched,
    HwAsid,
}

pub struct Sched {
    in_interrupt_ctx: bool,
    thread_idx: Option<usize>,
    threads: Vec<ThreadId>,
    advance_to_next_thread: fn(&mut Sched),
    compute_quantum: fn(ThreadId) -> Duration,
}

impl Sched {
    pub fn new() -> Self {
        Sched {
            in_interrupt_ctx: false,
            thread_idx: 0,
            threads: Vec::new(),
            advance_to_next_thread: |sched: &mut Sched| {
                if !sched.threads.is_empty() {
                    sched.thread_idx = (sched.thread_idx + 1) % sched.threads.len();
                }
            },
            compute_quantum: |_tid: ThreadId| Duration::from_millis(100), /* Default quantum of
                                                                           * 100ms */
        }
    }

    pub fn set_interrupt_context(&mut self) {
        self.in_interrupt_ctx = true;
    }

    pub fn clear_interrupt_context(&mut self) {
        self.in_interrupt_ctx = false;
    }

    pub fn in_interrupt_context(&self) -> bool {
        self.in_interrupt_ctx
    }

    pub fn get_current_thread(&self) -> Option<ThreadId> {
        if let Some(idx) = self.thread_idx {
            Some(self.threads[idx])
        } else {
            None
        }
    }

    pub fn add_thread(&mut self, tid: ThreadId) -> Result<(), Error> {
        self.threads.push(tid);
        Ok(())
    }

    pub fn remove_thread(&mut self, thread_id: ThreadId) -> Result<ThreadId, Error> {
        if let Some(pos) = self.threads.iter().position(|x| *x == thread_id) {
            Ok(self.threads.remove(pos))
        } else {
            Err(Error::ThreadNotInQueue)
        }
    }

    pub fn get_thread_count(&self) -> usize {
        self.threads.len()
    }

    pub fn next(&mut self) {
        self.advance_to_next_thread(self);
    }

    pub fn get_current_thread_quantum(&self) -> Option<Duration> {}

    pub fn is_thread_queued(&self, thread_id: ThreadId) -> bool {
        self.threads.iter().any(|&tid| tid == thread_id)
    }
}

pub struct HwAsid {
    asid_map: HashMap<AddressSpaceId, LpAsid>,
}

pub trait LpLocalSchedIfce {
    fn get_as_count(&self) -> usize;
}
