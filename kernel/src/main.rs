#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(core_intrinsics)]

use core::panic::PanicInfo;
use bootloader_api::{entry_point, BootInfo};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    loop {
        
    }
}

#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    loop {
        
    }
}
