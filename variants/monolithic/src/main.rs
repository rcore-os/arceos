#![no_std]
#![no_main]

#[macro_use]
extern crate axlog;
extern crate axstd;

use memory_addr::VirtAddr;

use axhal::arch::UspaceContext;

const USER_STACK_SIZE: usize = 4096;
static USTACK: [u8; USER_STACK_SIZE] = [0; USER_STACK_SIZE];

fn app_main(arg0: usize) {
    unsafe {
        core::arch::asm!(
            "2:",
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
    let entry = app_main as usize;
    let ustack_top = VirtAddr::from(USTACK.as_ptr_range().end as _);
    let kstack_top: usize;
    unsafe { core::arch::asm!("mov {}, rsp", out(reg) kstack_top) };
    let kstack_top = VirtAddr::align_down(kstack_top.into(), 16usize);

    let ctx = UspaceContext::new(entry, ustack_top, 2333);

    info!(
        "Enter user space: entry={:#x}, ustack={:#x}, kstack={:#x}",
        entry, ustack_top, kstack_top,
    );
    unsafe { ctx.enter_uspace(kstack_top) };
}

#[no_mangle]
fn main() {
    run_apps();
}
