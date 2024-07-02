use axhal::trap::{PageFaultFlags, VirtAddr};
struct TrapHandlerImpl;

#[crate_interface::impl_interface]
impl axhal::trap::TrapHandler for TrapHandlerImpl {
    fn handle_irq(_irq_num: usize) {
        #[cfg(feature = "irq")]
        {
            let guard = kernel_guard::NoPreempt::new();
            axhal::irq::dispatch_irq(_irq_num);
            drop(guard); // rescheduling may occur when preemption is re-enabled.
        }
    }

    fn handle_page_fault(vaddr: VirtAddr, access_flags: PageFaultFlags, is_user: bool) -> bool {
        if is_user {
            warn!(
                "User #PF: fault_vaddr={:#x}, access_flags={:?}",
                vaddr, access_flags,
            );
            true
        } else {
            panic!(
                "Kernel #PF: fault_vaddr={:#x}, access_flags={:?}",
                vaddr, access_flags,
            );
        }
    }
}
