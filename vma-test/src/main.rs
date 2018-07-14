extern crate vma_rust;
use vma_rust::vk_mem_alloc::*;
use std::mem;

fn main() {
    unsafe {
        let allocator_info: VmaAllocatorCreateInfo = mem::zeroed();
        let mut vma_allocator = mem::uninitialized();
        vmaCreateAllocator(&allocator_info, &mut vma_allocator);
    }
}
