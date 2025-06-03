// os/src/mm/mod.rs

mod heap_allocator;
mod address;
mod frame_allocator;
mod page_table;

use address::{VPNRange, StepByOne};
pub use address::{PhysAddr, VirtAddr, PhysPageNum, VirtPageNum};
pub use frame_allocator::{FrameTracker, frame_alloc};
use page_table::{PTEFlags};
pub use page_table::{PageTableEntry};

pub fn init() {
    heap_allocator::init_heap();
    heap_allocator::heap_test();
    frame_allocator::init_frame_allocator();
    frame_allocator::frame_allocator_test();
}