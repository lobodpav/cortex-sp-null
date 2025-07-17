#![no_std]
#![no_main]

use core::panic::PanicInfo;
use cortex_m::asm::delay;
use cortex_m_rt::entry;
use log::{error, info};
use rtt_target::rtt_init_log;

#[entry]
fn main() -> ! {
    rtt_init_log!();

    info!("Started the application");

    let mut counter = 0;
    loop {
        info!("Counter: {}", counter);
        delay(64_000_000);

        counter += 1;
    }
}

#[panic_handler]
fn panic_handler(panic_info: &PanicInfo) -> ! {
    error!("The panic handler was called with PanicInfo: {}", panic_info);
    loop {}
}
