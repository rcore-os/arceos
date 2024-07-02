use memory_addr::VirtAddr;
use x86::{controlregs::cr2, irq::*};
use x86_64::structures::idt::PageFaultErrorCode;

use super::context::TrapFrame;
use crate::trap::PageFaultFlags;

core::arch::global_asm!(include_str!("trap.S"));

#[cfg(feature = "uspace")]
const LEGACY_SYSCALL_VECTOR: u8 = 0x80;

const IRQ_VECTOR_START: u8 = 0x20;
const IRQ_VECTOR_END: u8 = 0xff;

#[no_mangle]
fn x86_trap_handler(tf: &mut TrapFrame) {
    match tf.vector as u8 {
        PAGE_FAULT_VECTOR => {
            let vaddr = VirtAddr::from(unsafe { cr2() });
            let access_flags = get_page_fault_flags(tf.error_code);
            if !crate::trap::handle_page_fault(vaddr, access_flags, tf.is_user()) {
                panic!(
                    "Unhandled {} #PF @ {:#x}, fault_vaddr={:#x}, error_code={:#x}:\n{:#x?}",
                    if tf.is_user() { "user" } else { "kernel" },
                    tf.rip,
                    vaddr,
                    tf.error_code,
                    tf,
                );
            }
        }
        BREAKPOINT_VECTOR => debug!("#BP @ {:#x} ", tf.rip),
        GENERAL_PROTECTION_FAULT_VECTOR => {
            panic!(
                "#GP @ {:#x}, error_code={:#x}:\n{:#x?}",
                tf.rip, tf.error_code, tf
            );
        }
        #[cfg(feature = "uspace")]
        LEGACY_SYSCALL_VECTOR => super::syscall::x86_syscall_handler(tf),
        IRQ_VECTOR_START..=IRQ_VECTOR_END => crate::trap::handle_irq_extern(tf.vector as _),
        _ => {
            panic!(
                "Unhandled exception {} ({}, error_code = {:#x}) @ {:#x}:\n{:#x?}",
                tf.vector,
                vec_to_str(tf.vector),
                tf.error_code,
                tf.rip,
                tf
            );
        }
    }
}

fn get_page_fault_flags(error_code: u64) -> PageFaultFlags {
    let reserved_bits = (PageFaultErrorCode::CAUSED_BY_WRITE
        | PageFaultErrorCode::USER_MODE
        | PageFaultErrorCode::PROTECTION_VIOLATION)
        .complement();
    let code = PageFaultErrorCode::from_bits_truncate(error_code);
    if code.intersects(reserved_bits) {
        panic!("Invalid #PF error code: {:?}", code);
    }

    let mut flags = PageFaultFlags::empty();
    if code.contains(PageFaultErrorCode::CAUSED_BY_WRITE) {
        flags |= PageFaultFlags::WRITE;
    } else {
        flags |= PageFaultFlags::READ;
    }
    if code.contains(PageFaultErrorCode::USER_MODE) {
        flags |= PageFaultFlags::USER;
    }
    if code.contains(PageFaultErrorCode::PROTECTION_VIOLATION) {
        flags |= PageFaultFlags::EXECUTE;
    }
    flags
}

fn vec_to_str(vec: u64) -> &'static str {
    if vec < 32 {
        EXCEPTIONS[vec as usize].mnemonic
    } else {
        "Unknown"
    }
}
