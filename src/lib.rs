#![allow(non_snake_case)]
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
pub mod task;
pub mod terminal;

use allocator::{Locked, linked_list::LinkedListAllocator};

// use linked_list_allocator::LockedHeap;
// #[global_allocator]
// pub static ALLOCATOR: LockedHeap = LockedHeap::empty();
// use allocator::{bump::BumpAllocator};
// #[global_allocator]
// pub static ALLOCATOR: Locked<BumpAllocator> = Locked::new(BumpAllocator::new());
#[global_allocator]
pub static ALLOCATOR: Locked<LinkedListAllocator> =
    Locked::new(LinkedListAllocator::new());
// #[global_allocator]
// static ALLOCATOR: Locked<FixedSizeBlockAllocator> = Locked::new(
//     FixedSizeBlockAllocator::new());

pub fn init() {
    gdt::init(); // init global descriptor table
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() }; // enable programmable interrup controller
    x86_64::instructions::interrupts::enable(); // enable CPU interrups
    print!("> ");
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}