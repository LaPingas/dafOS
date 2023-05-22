#![allow(non_snake_case)]
#![cfg_attr(not(test), no_std)] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(abi_x86_interrupt)] // enable the x86 ABI interrupt experimental feature
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![feature(const_mut_refs)]
#![test_runner(crate::test_runner)]
#![allow(unused_imports, unreachable_code)]

use dafOS_mobile::{print, println, 
    allocator, gdt, interrupts, memory, vga_buffer, 
    task::{Task, simple_executor::SimpleExecutor, executor::Executor, keyboard}};

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}
#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}

extern crate alloc;
use core::panic::PanicInfo;
use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};

// mod vga_buffer;
// // pub mod interrupts;
// pub mod gdt;
// pub mod memory;
// pub mod allocator;

use allocator::Locked;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    print!("\n{}", info);
    loop {}
}

use bootloader::{BootInfo, entry_point};
entry_point!(kernel_main);
// #[no_mangle] // don't mangle the name of this function
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    
    use x86_64::{structures::paging::{Translate, Page}, VirtAddr};

    // print!("Hello again");
    // println!(", some numbers: {} {}", 42, 1.337);

    dafOS_mobile::init(); // use the init in libs.irs

    // invoke a breakpoint exception
    //x86_64::instructions::interrupts::int3();

    // unsafe {
    //     *(0x0 as *mut u64) = 42; // didn't handle page-faults yet
    // };
    // fn stack_overflow() {
    //     stack_overflow(); // for each recursion, the return address is pushed
    // }
    // trigger a stack overflow
    // stack_overflow();

    // demoing page fault
    // let ptr = 0x0 as *mut u32;
    // unsafe { *ptr = 42; }
    // let ptr = 0x0 as *mut i32;
    // unsafe {core::ptr::write_unaligned(ptr, 0x2031b2)};

    // // read from a code page
    // unsafe { let x = *ptr; }
    // println!("read worked");

    // // write to a code page
    // unsafe { *ptr = 42; }
    // println!("write worked");
    // use x86_64::registers::control::Cr3;
    // let (level_4_page_table, _) = Cr3::read();
    // println!("Level 4 page table at: {:?}", level_4_page_table.start_address());

    // let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    // let l4_table = unsafe { active_level_4_table(phys_mem_offset) };

    // for (i, entry) in l4_table.iter().enumerate() {
    //     use x86_64::structures::paging::PageTable;
    //     if !entry.is_unused() {
    //         println!("L4 Entry {}: {:?}", i, entry);
        
    //         // get the physical address from the entry and convert it
    //         let phys = entry.frame().unwrap().start_address();
    //         let virt = phys.as_u64() + boot_info.physical_memory_offset;
    //         let ptr = VirtAddr::new(virt).as_mut_ptr();
    //         let l3_table: &PageTable = unsafe { &*ptr };
        
    //         // print non-empty entries of the level 3 table
    //         for (i, entry) in l3_table.iter().enumerate() {
    //             if !entry.is_unused() {
    //                 println!("  L3 Entry {}: {:?}", i, entry);
    //             }
    //         }
    //     }
        
    // }

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        memory::BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
    // // map an unused page
    // let page = Page::containing_address(VirtAddr::new(0));
    // memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // // write the string `New!` to the screen through the new mapping
    // let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    // unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};

    // let addresses = [
    //     // the identity-mapped vga buffer page
    //     0xb8000,
    //     // some code page
    //     0x201008,
    //     // some stack page
    //     0x0100_0020_1a10,
    //     // virtual address mapped to physical address 0
    //     boot_info.physical_memory_offset,
    // ];

    // for &address in &addresses {
    //     let virt = VirtAddr::new(address);
    //     let phys = mapper.translate_addr(virt);
    //     println!("{:?} -> {:?}", virt, phys);
    // }

    // allocator::init_heap(&mut mapper, &mut frame_allocator, &dafOS_mobile::ALLOCATOR)
    //     .expect("heap initialization failed");
    allocator::init_heap(&mut mapper, &mut frame_allocator, &dafOS_mobile::ALLOCATOR)
        .expect("heap initialization failed");

    // vga_buffer::GLOBAL_COMMAND_BUFFER.push(8);

    let mut executor = Executor::new();
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();

    // let x = Box::new(41);
    // println!("heap_value at {:p}: {}", x, x);
    // let mut vec = Vec::new();
    // for i in 0..500 {
    //     vec.push(i);
    // }
    // println!("vec at {:p}", vec.as_slice());
    // println!("vec at {:p}: {:?}", vec.as_slice(), vec);
    // create a reference counted vector -> will be freed when count reaches 0
    // let reference_counted = Rc::new(vec![1, 2, 3]);
    // let cloned_reference = reference_counted.clone();
    // println!("current reference count is {}", Rc::strong_count(&cloned_reference));
    // core::mem::drop(reference_counted);
    // println!("reference count is {} now", Rc::strong_count(&cloned_reference));

    #[cfg(test)]
    test_main();

    println!("It did not crash!");

    // panic!("This is a panic");
    // loop {
    //     // print!("-");
    // }
    dafOS_mobile::hlt_loop();
}