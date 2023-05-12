#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(abi_x86_interrupt)] // enable the x86 ABI interrupt experimental feature
#![feature(const_mut_refs)]

extern crate alloc;

pub mod interrupts;
pub mod vga_buffer;
pub mod gdt;
pub mod memory;
pub mod allocator;

use allocator::{Locked};

// use linked_list_allocator::LockedHeap;
// #[global_allocator]
// pub static ALLOCATOR: LockedHeap = LockedHeap::empty();
// use allocator::{bump::BumpAllocator};
// #[global_allocator]
// pub static ALLOCATOR: Locked<BumpAllocator> = Locked::new(BumpAllocator::new());
use allocator::linked_list::LinkedListAllocator;
#[global_allocator]
pub static ALLOCATOR: Locked<LinkedListAllocator> =
    Locked::new(LinkedListAllocator::new());

pub fn init() {
    gdt::init(); // init global descriptor table
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() }; // enable programmable interrup controller
    x86_64::instructions::interrupts::enable(); // enable CPU interrups
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}