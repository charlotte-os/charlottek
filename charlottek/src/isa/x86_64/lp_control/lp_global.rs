use alloc::boxed::Box;
use core::mem::MaybeUninit;

use crate::threading::scheduler::LpLocalScheduler;

/* This structure contains the global data for each logical processor */
struct GsSegment {
    local_sched: MaybeUninit<Box<dyn LpLocalScheduler>>,
}

impl GsSegment {
    pub fn new() -> Self {
        GsSegment {
            local_sched: MaybeUninit::uninit(),
        }
    }
}
