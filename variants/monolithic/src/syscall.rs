#![allow(dead_code)]

use arceos_posix_api as api;
use axerrno::LinuxError;
use axhal::arch::{TrapFrame, UspaceContext};
use axhal::trap::SyscallHandler;
use axtask::TaskExtRef;

const SYS_READ: usize = 0;
const SYS_WRITE: usize = 1;
const SYS_SCHED_YIELD: usize = 24;
const SYS_GETPID: usize = 39;
const SYS_CLONE: usize = 56;
const SYS_FORK: usize = 57;
const SYS_EXECVE: usize = 59;
const SYS_EXIT: usize = 60;

fn sys_clone(tf: &TrapFrame, newsp: usize) -> usize {
    let aspace = axtask::current().task_ext().aspace.clone();
    let mut uctx = UspaceContext::from(tf);
    uctx.set_sp(newsp);
    uctx.set_ret_reg(0);
    let new_task = super::spawn_user_task(aspace, uctx);
    new_task.id().as_u64() as usize
}

struct SyscallHandlerImpl;

#[crate_interface::impl_interface]
impl SyscallHandler for SyscallHandlerImpl {
    fn handle_syscall(tf: &TrapFrame, syscall_num: usize) -> isize {
        let ret = match syscall_num {
            SYS_READ => api::sys_read(tf.arg0() as _, tf.arg1() as _, tf.arg2() as _),
            SYS_WRITE => api::sys_write(tf.arg0() as _, tf.arg1() as _, tf.arg2() as _),
            SYS_SCHED_YIELD => api::sys_sched_yield() as isize,
            SYS_GETPID => api::sys_getpid() as isize,
            _ => {
                warn!("Unimplemented syscall: {}", syscall_num);
                -LinuxError::ENOSYS.code() as _
            }
        };
        ret
    }
}
