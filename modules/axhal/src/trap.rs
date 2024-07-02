//! Trap handling.

use crate_interface::{call_interface, def_interface};

use crate::arch::TrapFrame;

pub use memory_addr::VirtAddr;
pub use page_table_entry::MappingFlags as PageFaultFlags;

/// Syscall handler interface.
#[def_interface]
pub trait SyscallHandler {
    /// Handles a system call with the given number, arguments are stored in
    /// [`TrapFrame`].
    fn handle_syscall(tf: &TrapFrame, syscall_num: usize) -> isize;
}

/// Trap handler interface.
///
/// This trait is defined with the [`#[def_interface]`][1] attribute. Users
/// should implement it with [`#[impl_interface]`][2] in any other crate.
///
/// [1]: crate_interface::def_interface
/// [2]: crate_interface::impl_interface
#[def_interface]
pub trait TrapHandler {
    /// Handles interrupt requests for the given IRQ number.
    fn handle_irq(irq_num: usize);
    /// Handles (kernel/user) page faults with the given accessed address and
    /// flags. Returns `true` if it handled successfully.
    fn handle_page_fault(vaddr: VirtAddr, access_flags: PageFaultFlags, is_user: bool) -> bool;
}

/// Call the external IRQ handler.
#[allow(dead_code)]
pub(crate) fn handle_irq_extern(irq_num: usize) {
    call_interface!(TrapHandler::handle_irq, irq_num);
}

/// Call the external page fault handler.
#[allow(dead_code)]
pub(crate) fn handle_page_fault(
    vaddr: VirtAddr,
    access_flags: PageFaultFlags,
    is_user: bool,
) -> bool {
    call_interface!(TrapHandler::handle_page_fault, vaddr, access_flags, is_user)
}

/// Call the external syscall handler.
#[allow(dead_code)]
pub(crate) fn handle_syscall(tf: &TrapFrame, syscall_num: usize) -> isize {
    call_interface!(SyscallHandler::handle_syscall, tf, syscall_num)
}
