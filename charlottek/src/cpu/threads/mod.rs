use alloc::boxed::Box;

use spin::{Lazy, RwLock};

use crate::isa::lp::thread_context::ThreadContext;
use crate::klib::collections::id_table::IdTable;

type ThreadTable = IdTable<ThreadId, Thread>;
pub type ThreadId = usize;

pub static THREAD_TABLE: Lazy<RwLock<ThreadTable>> = Lazy::new(|| RwLock::new(ThreadTable::new()));

pub struct Thread {
    pub state: ThreadContext,
    _stack_buffer: Box<[u8]>,
}
