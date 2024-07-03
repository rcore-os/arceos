#![no_std]
#![no_main]

#[macro_use]
extern crate axlog;
extern crate alloc;
extern crate axstd;

mod task;

use alloc::sync::Arc;

use axhal::arch::{TrapFrame, UspaceContext};
use axhal::mem::virt_to_phys;
use axhal::paging::MappingFlags;
use axruntime::KERNEL_PAGE_TABLE;
use axsync::Mutex;
use axtask::{AxTaskRef, TaskExtRef, TaskInner};
use memory_addr::VirtAddr;

use self::task::{AddrSpace, TaskExt};

const USER_STACK_SIZE: usize = 4096;
const KERNEL_STACK_SIZE: usize = 0x40000; // 256 KiB

fn app_main(arg0: usize) {
    unsafe {
        core::arch::asm!(
            "2:",
            "int3",
            "syscall",
            "add rax, 1",
            "jmp 2b",
            in("rax") arg0,
            in("rdi") 2,
            in("rsi") 3,
            in("rdx") 3,
            in("rcx") 3,
            options(nostack, nomem)
        )
    }
}

fn spawn_user_task(aspace: Arc<Mutex<AddrSpace>>, uctx: UspaceContext) -> AxTaskRef {
    let mut task = TaskInner::new(
        || {
            let curr = axtask::current();
            let kstack_top = curr.kernel_stack_top().unwrap();
            info!(
                "Enter user space: entry={:#x}, ustack={:#x}, kstack={:#x}",
                curr.task_ext().uctx.get_ip(),
                curr.task_ext().uctx.get_sp(),
                kstack_top,
            );
            unsafe { curr.task_ext().uctx.enter_uspace(kstack_top) };
        },
        "".into(),
        KERNEL_STACK_SIZE,
    );
    task.ctx_mut()
        .set_page_table_root(aspace.lock().page_table_root());
    task.init_task_ext(TaskExt::new(uctx, aspace));
    axtask::spawn_task(task)
}

fn sys_clone(tf: &TrapFrame, newsp: usize) -> usize {
    let aspace = axtask::current().task_ext().aspace.clone();
    let mut uctx = UspaceContext::from(tf);
    uctx.set_sp(newsp);
    uctx.set_ret_reg(0);
    let new_task = spawn_user_task(aspace, uctx);
    new_task.id().as_u64() as usize
}

fn run_apps() -> ! {
    let entry = VirtAddr::from(app_main as usize);
    let entry_paddr_align = virt_to_phys(entry.align_down_4k());
    let entry_vaddr_align = VirtAddr::from(0x1000);
    let entry_vaddr = entry_vaddr_align + entry.align_offset_4k();

    let layout = core::alloc::Layout::from_size_align(USER_STACK_SIZE, 4096).unwrap();
    let ustack = unsafe { alloc::alloc::alloc(layout) };
    let ustack_paddr = virt_to_phys(VirtAddr::from(ustack as _));
    let ustack_top = VirtAddr::from(0x7fff_0000);
    let ustack_vaddr = ustack_top - USER_STACK_SIZE;

    let kspace_base = VirtAddr::from(axconfig::PHYS_VIRT_OFFSET);
    let kspace_size = 0x7f_ffff_f000;
    let mut pt = KERNEL_PAGE_TABLE
        .clone_shallow(kspace_base, kspace_size)
        .unwrap();

    pt.map_region(
        entry_vaddr_align,
        entry_paddr_align,
        4096,
        MappingFlags::READ | MappingFlags::EXECUTE | MappingFlags::USER,
        false,
    )
    .unwrap();
    pt.map_region(
        ustack_vaddr,
        ustack_paddr,
        4096,
        MappingFlags::READ | MappingFlags::EXECUTE | MappingFlags::USER,
        false,
    )
    .unwrap();

    spawn_user_task(
        Arc::new(Mutex::new(AddrSpace(pt.root_paddr()))),
        UspaceContext::new(entry_vaddr.into(), ustack_top, 2333),
    );

    axtask::WaitQueue::new().wait();
    unreachable!()
}

#[no_mangle]
fn main() {
    run_apps();
}
