use alloc::sync::Arc;
use axhal::time::wall_time;
use kspin::SpinNoIrq;
use lazyinit::LazyInit;
use timer_list::{TimeValue, TimerEvent, TimerList};

use crate::{AxTaskRef, RUN_QUEUE};

// TODO: per-CPU
static TIMER_LIST: LazyInit<SpinNoIrq<TimerList<TaskWakeupEvent>>> = LazyInit::new();

struct TaskWakeupEvent(AxTaskRef);

impl TimerEvent for TaskWakeupEvent {
    fn callback(self, _now: TimeValue) {
        let mut rq = RUN_QUEUE.lock();
        self.0.set_in_timer_list(false);
        rq.unblock_task(self.0, true);
    }
}

pub fn set_alarm_wakeup(deadline: TimeValue, task: AxTaskRef) {
    let mut timers = TIMER_LIST.lock();
    task.set_in_timer_list(true);
    timers.set(deadline, TaskWakeupEvent(task));
}

pub fn cancel_alarm(task: &AxTaskRef) {
    let mut timers = TIMER_LIST.lock();
    task.set_in_timer_list(false);
    timers.cancel(|t| Arc::ptr_eq(&t.0, task));
}

pub fn check_events() {
    loop {
        let now = wall_time();
        let event = TIMER_LIST.lock().expire_one(now);
        if let Some((_deadline, event)) = event {
            event.callback(now);
        } else {
            break;
        }
    }
}

pub fn init() {
    TIMER_LIST.init_once(SpinNoIrq::new(TimerList::new()));
}
