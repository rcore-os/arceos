use alloc::{collections::BTreeMap, sync::Arc};
use core::ops::Deref;
use core::sync::atomic::{AtomicIsize, Ordering};

use crate::BaseScheduler;
use crate::current_ticks;

pub struct SJFTask<T, const A: usize, const LOGB: usize> {
    inner: T,
    expect_runtime: AtomicIsize, // 用整数记录移动平均
    start_time_after_sched: AtomicIsize, // 最近一次被调度的纳秒时刻
    alpha_a: AtomicIsize,
    alpha_log_b: AtomicIsize,
    id: AtomicIsize,
}

// TODO：现在全部都是暴力实现

impl<T, const A: usize, const LOGB: usize> SJFTask<T, A, LOGB> {
    pub const fn new(inner: T, _n: isize) -> Self {
        Self {
            inner,
            expect_runtime: AtomicIsize::new(0 as isize),
            start_time_after_sched: AtomicIsize::new(0 as isize),
            id: AtomicIsize::new(0 as isize),
            alpha_a: AtomicIsize::new(A as isize),
            alpha_log_b: AtomicIsize::new(LOGB as isize),
        }
    }

    pub fn set_id(&self, id: isize) {
        self.id.store(id, Ordering::Release);
    }

    fn get_id(&self) -> isize {
        self.id.load(Ordering::Acquire)
    }
    
    /// 根据本次的运行时间更新期望运行时间，这里 alpha = alpha_a/(2^alpha_log_b)
    pub fn update_expect_runtime(&self) {
        let last_expect_runtime = self.expect_runtime.load(Ordering::Acquire);
        let last_start_time_after_sched = self.start_time_after_sched.load(Ordering::Acquire);
        let delta = current_ticks() as isize - last_start_time_after_sched;
        let expect = (((delta - last_expect_runtime) * self.alpha_a.load(Ordering::Acquire)) >> self.alpha_log_b.load(Ordering::Acquire)) + last_expect_runtime;
        self.expect_runtime.store(expect, Ordering::Release);
    }

    pub fn get_expect_runtime(&self) -> isize {
        self.expect_runtime.load(Ordering::Acquire)
    }

    pub fn sched_timer(&self) {
        self.start_time_after_sched.store(current_ticks() as isize, Ordering::Release);
    }

    pub const fn inner(&self) -> &T {
        &self.inner
    }
}

impl<T, const A: usize, const LOGB: usize> Ord for SJFTask<T, A, LOGB> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let a = self.get_expect_runtime();
        let b = other.get_expect_runtime();
        let a_id = self.get_id();
        let b_id = other.get_id();
        
        if a < b {
            core::cmp::Ordering::Less
        } else if a > b {
            core::cmp::Ordering::Greater
        } else if a_id < b_id {
            core::cmp::Ordering::Less
        } else if a_id > b_id {
            core::cmp::Ordering::Greater
        } else {
            core::cmp::Ordering::Equal
        }
    }
}

impl<T, const A: usize, const LOGB: usize> Eq for SJFTask<T, A, LOGB> {}

impl<T, const A: usize, const LOGB: usize> PartialOrd for SJFTask<T, A, LOGB> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        let a = self.get_expect_runtime();
        let b = other.get_expect_runtime();
        let a_id = self.get_id();
        let b_id = other.get_id();
        if a < b {
            Some(core::cmp::Ordering::Less)
        } else if a > b {
            Some(core::cmp::Ordering::Greater)
        } else if a_id < b_id {
            Some(core::cmp::Ordering::Less)
        } else if a_id > b_id {
            Some(core::cmp::Ordering::Greater)
        } else {
            Some(core::cmp::Ordering::Equal)
        }
    }
}

impl<T, const A: usize, const LOGB: usize> PartialEq for SJFTask<T, A, LOGB> {
    fn eq(&self, other: &Self) -> bool {
        self.get_expect_runtime() == other.get_expect_runtime() 
    }
}

impl<T, const A: usize, const LOGB: usize> const Deref for SJFTask<T, A, LOGB> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub struct SJFScheduler<T, const A: usize, const LOGB: usize> {
    ready_queue: BTreeMap<Arc<SJFTask<T, A, LOGB>>, isize>,
    id_pool: AtomicIsize,
}

impl<T, const A: usize, const LOGB: usize> SJFScheduler<T, A, LOGB> {
    pub const fn new() -> Self {
        Self {
            ready_queue: BTreeMap::new(),
            id_pool: AtomicIsize::new(0 as isize),
        }
    }
}

impl<T, const A: usize, const LOGB: usize> BaseScheduler for SJFScheduler<T, A, LOGB> {
    type SchedItem = Arc<SJFTask<T, A, LOGB>>;

    fn init(&mut self) {}

    fn add_task(&mut self, task: Self::SchedItem) {
        (*task).set_id(self.id_pool.fetch_add(1, Ordering::Release));
        self.ready_queue.insert(task, 0);
    }

    fn remove_task(&mut self, task: &Self::SchedItem) -> Option<Self::SchedItem> {
        if let Some(tmp) = self.ready_queue.remove_entry(task) {
            Some(tmp.0)
        } else {
            None
        }
    }

    fn pick_next_task(&mut self) -> Option<Self::SchedItem> {
        if let Some((k, _)) = self.ready_queue.pop_first() {
            k.sched_timer();
            Some(k)
        } else {
            None
        }
    }

    fn put_prev_task(&mut self, prev: Self::SchedItem, _preempt: bool) {
        // TODO: 现在还不支持 preempt，现在还在研究内核是怎么写的
        prev.update_expect_runtime();
        self.ready_queue.insert(prev, 0);
    }

    fn task_tick(&mut self, _current: &Self::SchedItem) -> bool {
        // 这个算法没有时间片问题
        false
    }
}

//std::thread::sleep(Duration::from_millis(10)); // sleep 10 ms