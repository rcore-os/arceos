//! User-defined task extended data.

extern "C" {
    fn __start_ax_task_ext();
    fn __stop_ax_task_ext();
}

#[no_mangle]
#[linkage = "weak"]
#[link_section = "ax_task_ext"]
static __AX_TASK_EXT: () = ();

pub(crate) struct AxTaskExt {
    ptr: *mut u8,
}

impl AxTaskExt {
    fn size() -> usize {
        __stop_ax_task_ext as usize - __start_ax_task_ext as usize
    }

    pub(crate) fn as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    pub(crate) fn alloc() -> Self {
        let size = Self::size();
        let ptr = if size == 0 {
            core::ptr::null_mut()
        } else {
            let layout = core::alloc::Layout::from_size_align(size, 0x10).unwrap();
            let dst = unsafe { alloc::alloc::alloc(layout) };
            let src = &__AX_TASK_EXT as *const _ as *const u8;
            unsafe { core::ptr::copy_nonoverlapping(src, dst, size) };
            dst
        };
        Self { ptr }
    }
}

impl Drop for AxTaskExt {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            let layout = core::alloc::Layout::from_size_align(Self::size(), 0x10).unwrap();
            unsafe { alloc::alloc::dealloc(self.ptr, layout) };
        }
    }
}

/// A trait to convert [`TaskInner::task_ext_ptr`] to the reference of the
/// concrete type.
///
/// [`TaskInner::task_ext_ptr`]: crate::TaskInner::task_ext_ptr
pub trait TaskExtRef<T: Sized> {
    /// Get a reference to the task extended data.
    fn task_ext(&self) -> &T;
}

/// A trait to convert [`TaskInner::task_ext_ptr`] to the mutable reference of
/// the concrete type.
///
/// [`TaskInner::task_ext_ptr`]: crate::TaskInner::task_ext_ptr
pub trait TaskExtMut<T: Sized> {
    /// Get a mutable reference to the task extended data.
    fn task_ext_mut(&mut self) -> &mut T;
}

/// Define the task extended data.
///
/// It automatically implements [`TaskExtRef`] and [`TaskExtMut`] for
/// [`TaskInner`].
///
/// # Example
///
/// ```no_run
/// # #![allow(non_local_definitions)]
/// use axtask::{def_task_ext, TaskExtRef};
///
/// pub struct TaskExtImpl {
///    proc_id: usize,
/// }
///
/// def_task_ext!(TaskExtImpl, TaskExtImpl { proc_id: 0 });
///
/// let task = axtask::spawn(|| {});
/// assert_eq!(task.task_ext().proc_id, 0);
/// ```
///
/// [`TaskInner`]: crate::TaskInner
#[macro_export]
macro_rules! def_task_ext {
    ($task_ext_struct:ty, $default:expr) => {
        #[no_mangle]
        #[link_section = "ax_task_ext"]
        static __AX_TASK_EXT: $task_ext_struct = $default;

        impl $crate::TaskExtRef<$task_ext_struct> for $crate::TaskInner {
            fn task_ext(&self) -> &$task_ext_struct {
                unsafe { &*(self.task_ext_ptr() as *const $task_ext_struct) }
            }
        }

        impl $crate::TaskExtMut<$task_ext_struct> for $crate::TaskInner {
            fn task_ext_mut(&mut self) -> &mut $task_ext_struct {
                unsafe { &mut *(self.task_ext_ptr() as *mut $task_ext_struct) }
            }
        }
    };
}
