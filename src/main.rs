#![no_std]
#![no_main]

use core::arch::asm;
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

    info!("SP FLASH ADDRESS: {:#010X}", sp_ptr as usize);
    info!("PC FLASH ADDRESS: {:#010X}", pc_ptr as usize);
    info!("SP RAM ADDRESS:   {:#010X}", safe_read_u32(sp_ptr));
    info!("PC RAM ADDRESS:   {:#010X}", safe_read_u32(pc_ptr));
}

/// Safely reads a [u32] value where the [pointer] points to,
/// even when it is a [null pointer](https://doc.rust-lang.org/std/ptr/fn.null.html) (a pointer to the `0x0` address).
///
/// See https://stackoverflow.com/a/79706233/1379273 for more details.
fn safe_read_u32(pointer: *const u32) -> u32 {
    let sp_value: u32;
    unsafe {
        asm!(
            "ldr {0}, [{1}]",   // Load from [r1] into r0 register
            out(reg) sp_value,  // {0} = output register
            in(reg) pointer,    // {1} = input register
        );
    };
    sp_value
}
