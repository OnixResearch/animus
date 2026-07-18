#![no_std]

use core::panic::PanicInfo;

const FIXTURE_DOMAIN_MASK: u32 = 0xA11C_E5ED;

/// Test-only bounded pure fixture operation. This is not a plugin SDK.
#[no_mangle]
pub extern "C" fn run(value: u32) -> u32 {
    value.rotate_left(1) ^ FIXTURE_DOMAIN_MASK
}

#[panic_handler]
fn panic(_: &PanicInfo<'_>) -> ! {
    loop {}
}
