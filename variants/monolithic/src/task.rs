use alloc::sync::Arc;

use axhal::arch::UspaceContext;
use axmm::AddrSpace;
use axsync::Mutex;

/// Task extended data for the monolithic kernel.
pub struct TaskExt {
    /// The process ID.
    pub proc_id: usize,
    /// The user space context.
    pub uctx: UspaceContext,
    /// The virtual memory address space.
    pub aspace: Arc<Mutex<AddrSpace>>,
}

impl TaskExt {
    pub const fn new(uctx: UspaceContext, aspace: Arc<Mutex<AddrSpace>>) -> Self {
        Self {
            proc_id: 233,
            uctx,
            aspace,
        }
    }
}

axtask::def_task_ext!(TaskExt);
