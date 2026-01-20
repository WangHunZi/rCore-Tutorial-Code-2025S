#![no_std]
#![no_main]

use core::{arch::asm, panic::PanicInfo};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn syscall(number: usize, arg0: usize, arg1: usize, arg2: usize) -> isize {
    let mut ret: isize;
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") arg0 => ret,
            in("x11") arg1,
            in("x12") arg2,
            in("x17") number
        );
    };
    ret
}

#[no_mangle]
extern "C" fn _start() {
    let text: &[u8] = b"Hello, RISCV!";

    let exit = syscall(64, 1, text.as_ptr() as usize, text.len());
    syscall(93, exit as usize, 0, 0);
}
