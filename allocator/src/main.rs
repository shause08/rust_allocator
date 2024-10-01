#![no_main]
#![no_std]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {

    }
}

///Rust needs a panic handler for now just a no return function that indefinitely loop 
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{
    }
}
/*
fn main(){}
*/
