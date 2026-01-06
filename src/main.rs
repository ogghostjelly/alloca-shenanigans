#![no_main]
use std::{ffi::c_void, slice};

#[unsafe(no_mangle)]
pub fn main() {
    // Look Ma! No heap!
    unsafe {
        alloca(4, |ptr| {
            let slice = slice::from_raw_parts_mut(ptr as *mut u8, 4);

            slice[0] = 1;
            slice[1] = 2;
            slice[2] = 3;
            slice[3] = 4;

            println!("{slice:?}");
        })
    };
}

/// Allocate an amount of memory in bytes on the stack.
/// The pointer to that memory is passed to the given function.
///
/// # Safety
/// For the love of god don't stack overflow.
pub unsafe fn alloca(size: usize, f: fn(*mut c_void)) {
    use std::arch::asm;

    /*
    rbp = rsp
    rsp -= floor16(16 + 8 + size - 1);
    f(floor16(rsp + 15));
    rsp = rbp;
    */

    unsafe {
        asm!(
            // rbp is callee-saved (as opposed to caller-saved)
            // meaning it will need to be restored later.
            "push rbp",
            // set the stack base pointer (rbp) to rsp
            "mov rbp, rsp",

            // rdi = size, because it was specified as an in(reg)
            // 16 + 8 + size - 1
            "add rdi, 23",

            // sets the last 4 bits to zero (effectively rounds-down to 16)
            // it needs to be rounded to 16 because stack alignment
            "and rdi, 0xfffffffffffffff0",

            // rsp -= rdi
            // rdi = rsp + 15
            "sub rsp, rdi",
            "mov rdi, rsp",
            "add rdi, 15",

            // round rdi to 16 again
            "and rdi, 0xfffffffffffffff0",

            // call the function
            "call {f}",

            // restore stack using the base pointer (rbp)
            "mov rsp, rbp",
            // restore registers
            "pop rbp",

            in("rdi") size,
            f = in(reg) f,
        )
    };
}
