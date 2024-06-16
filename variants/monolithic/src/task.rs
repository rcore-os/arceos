use axhal::arch::UspaceContext;
use memory_addr::PhysAddr;

/// Task extended data for the monolithic kernel.
pub struct TaskExt {
    /// The process ID.
    pub proc_id: usize,
    /// The user space context.
    pub uctx: UspaceContext,
    /// The root of the page table.
    pub page_table_root: PhysAddr,
}

impl TaskExt {
    /// Creates an empty [`TaskExt`] for initialization.
    const fn default() -> Self {
        Self {
            proc_id: 233,
            uctx: UspaceContext::empty(),
            page_table_root: PhysAddr::from(0),
        }
    }
}

axtask::def_task_ext!(TaskExt, TaskExt::default());
