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
/// A pointer to that allocated memory is passed to the given function.
///
/// The allocated memory is popped from the stack at the end of the function,
/// but the contents of that memory is never `Drop`-ed.
/// You will have to call drop manually.
///
/// # Safety
/// For the love of god don't stack overflow.
pub unsafe fn alloca(size: usize, f: fn(*mut c_void)) {
    use std::arch::asm;

    /*
    rbp = rsp
    rsp -= floor16(16 + 8 + size - 1);
    f(rsp);
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
            // rdi = rsp (rdi will be the first parameter for the following function call)
            "sub rsp, rdi",
            "mov rdi, rsp",

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
