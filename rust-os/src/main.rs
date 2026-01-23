#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
extern "C" fn _start() {
    unsafe {
        const TEXT: &[u8] = b"Hello RISCV!\n";
        core::arch::asm!(
            "li t1, 0x10000000",
            "mv t0, a0",
            "mv t2, a1",
        "1:",
            "beqz t2, 2f",
            "lb t3, 0(t0)",
            "sb t3, 0(t1)",

            "addi t0, t0, 1",
            "addi t2, t2, -1",
            "j 1b",
        "2:",
            in("a0") TEXT.as_ptr(),
            in("a1") TEXT.len(),
        );
    }
}
