#![no_std]
#![no_main]

#[macro_use]
extern crate axlog;
extern crate alloc;
extern crate axstd;

mod task;

use memory_addr::VirtAddr;

use axhal::arch::UspaceContext;
use axhal::mem::virt_to_phys;
use axhal::paging::MappingFlags;
use axruntime::KERNEL_PAGE_TABLE;
use axtask::TaskExtRef;

const USER_STACK_SIZE: usize = 4096;

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

    let kstack_top: usize;
    unsafe { core::arch::asm!("mov {}, rsp", out(reg) kstack_top) };
    let kstack_top = VirtAddr::align_down(kstack_top.into(), 16usize);

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

    let ctx = UspaceContext::new(entry_vaddr.into(), ustack_top, 2333);
    let pid = axtask::current().task_ext().proc_id;
    let parent = axtask::current().task_ext().parent;
    warn!("pid = {}", pid);
    warn!("parent = {}", parent);
    assert_eq!(pid, 233);
    assert_eq!(parent, 456);

    info!(
        "Enter user space: entry={:#x}, ustack={:#x}, kstack={:#x}",
        entry_vaddr, ustack_top, kstack_top,
    );
    unsafe {
        axhal::arch::write_page_table_root(pt.root_paddr());
        ctx.enter_uspace(kstack_top)
    }
}

#[no_mangle]
fn main() {
    run_apps();
}
