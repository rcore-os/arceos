pub struct TaskExt {
    pub proc_id: usize,
    pub parent: usize,
}

impl TaskExt {
    pub const fn default() -> Self {
        Self {
            proc_id: 233,
            parent: 456,
        }
    }
}

axtask::def_task_ext!(TaskExt, TaskExt::default());
