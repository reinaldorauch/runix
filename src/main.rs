#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(runix::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use runix::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello world {}", "!");

    runix::init(); // new

    #[cfg(test)]
    test_main();

    println!("It did not crash!");

    runix::hlt_loop();
}

// our existing panic handler
#[cfg(not(test))] // new attribute
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    runix::hlt_loop();
}

// our panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    runix::test_panic_handler(info);
}
