#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
extern "C" fn _start() {
    let text: &[u8] = b"Hello, world!\n";

    let code: u32 = 64;
    let mut ret: u32 = 0;
    unsafe {
        core::arch::asm!(
            "ecall",
            inlateout("x10") 1 => ret,
            in("x11") text.as_ptr() as u32,
            in("x12") text.len() as u32,
            in("x17") code,
        );

        core::arch::asm!(
            "ecall",
            inlateout("x10") 9 => ret,
            in("x11") 0,
            in("x12") 0,
            in("x17") 93,
        );
    }
}
