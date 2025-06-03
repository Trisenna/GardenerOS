#![feature(alloc_error_handler)]
#![no_std]
#![no_main]
extern crate alloc;
use core::arch::global_asm;

#[macro_use]
mod console;
mod lang_items;
mod sbi;
mod syscall;
mod trap;
mod loader;
mod config;
mod task;
mod mm;
mod timer;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

fn clear_bss() {
    unsafe extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}

#[unsafe(no_mangle)]
pub fn rust_main() -> ! {
    clear_bss();
    println!("[kernel] Hello, world!");

    trap::init();
    trap::enable_timer_interrupt();
    timer::set_next_trigger();

    loader::load_apps();
    task::run_first_task();
    mm::init();

    panic!("Unreachable in rust_main!");
}