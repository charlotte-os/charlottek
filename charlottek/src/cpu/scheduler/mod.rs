pub mod lp_local;

use alloc::boxed::Box;
use alloc::collections::vec_deque::VecDeque;
use alloc::vec::Vec;

use hashbrown::HashSet;
use spin::{Lazy, Mutex, RwLock};

use crate::cpu::threads::{Thread, ThreadId};
use crate::isa::lp::LpControl;
use crate::memory::AddressSpaceId;

pub static GLOBAL_SCHEDULER: Lazy<GlobalScheduler> = Lazy::new(GlobalScheduler::new);

pub struct GlobalScheduler {
    blocked_threads: RwLock<HashSet<ThreadId>>,
    ready_unassigned: Mutex<VecDeque<ThreadId>>,
    lp_schedulers: Vec<Mutex<LpScheduler>>,
}

impl GlobalScheduler {
    pub fn new() -> Self {
        Self {
            blocked_threads: RwLock::new(HashSet::new()),
            ready_unassigned: Mutex::new(VecDeque::new()),
            lp_schedulers: Vec::new(),
        }
    }

    pub fn get_local_lp_scheduler(&self) -> &Mutex<Box<dyn LpScheduler>> {
        &self.lp_schedulers[LpControl::get_lp_id() as usize]
    }
}

unsafe impl Sync for GlobalScheduler {}

enum LpScheduler {
    SimpleRr(SimpleRr),
    Darr(Darr),
}
