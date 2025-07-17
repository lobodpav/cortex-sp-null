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

    log_sp_pc();

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

fn log_sp_pc() {
    let flash_origin = 0x0u32;

    // Load memory addresses from the Vector Table in Flash
    let sp_ptr = flash_origin as *const u32;       // Stack Pointer
    let pc_ptr = (flash_origin + 4) as *const u32; // Program Counter pointer

    unsafe {
        info!("SP FLASH ADDRESS: {:#010X}", sp_ptr as usize);
        info!("PC FLASH ADDRESS: {:#010X}", pc_ptr as usize);
        info!("SP RAM ADDRESS:   {:#010X}", *sp_ptr);
        info!("PC RAM ADDRESS:   {:#010X}", *pc_ptr);
    }
}
