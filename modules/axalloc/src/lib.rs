//! [ArceOS](https://github.com/arceos-org/arceos) global memory allocator.
//!
//! It provides [`GlobalAllocator`], which implements the trait
//! [`core::alloc::GlobalAlloc`]. A static global variable of type
//! [`GlobalAllocator`] is defined with the `#[global_allocator]` attribute, to
//! be registered as the standard library’s default allocator.

#![no_std]

extern crate alloc;

#[doc(no_inline)]
pub use os_memory::{global_allocator, BootState, GlobalAllocator, MemRegion, MemRegionFlags};
mod page;

/// Get the name of the allocator.
pub fn name() -> &'static str {
    os_memory::allocator_name()
}
/// Initialize the allocator.
pub fn init<B: BootState>() {
    os_memory::init_allocator::<B>();
}
