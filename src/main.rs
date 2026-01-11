#![no_main]
use std::{
    ffi::c_void,
    mem::{ManuallyDrop, MaybeUninit},
    slice,
};

#[unsafe(no_mangle)]
pub fn main() {
    // Look Ma! No heap!
    unsafe {
        let mut ret = Some([4, 3, 2, 1]);

        alloca(4, |ptr| {
            let slice = slice::from_raw_parts_mut(ptr as *mut u8, 4);

            slice[0] = 1;
            slice[1] = 2;
            slice[2] = 3;
            slice[3] = 4;

            ret = Some({
                let mut dst = [0; 4];
                dst.copy_from_slice(slice);
                dst
            });
        });

        println!("{ret:?}");
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
pub unsafe fn alloca(size: usize, closure: impl FnOnce(*mut MaybeUninit<u8>)) {
    let f = get_trampoline(&closure);
    let mut closure = ManuallyDrop::new(closure);
    let data = &raw mut closure as *mut c_void;
    unsafe { raw_alloca(size, f, data) };
}

// Thanks to the [alloca](https://docs.rs/alloca/latest/alloca/) rust crate for this closure->fn idea.
fn get_trampoline<F: FnOnce(*mut MaybeUninit<u8>)>(
    _closure: &F,
) -> extern "C" fn(*mut MaybeUninit<u8>, *mut c_void) {
    trampoline::<F>
}

extern "C" fn trampoline<F: FnOnce(*mut MaybeUninit<u8>)>(
    ptr: *mut MaybeUninit<u8>,
    data: *mut c_void,
) {
    let f = unsafe { ManuallyDrop::take(&mut *(data as *mut ManuallyDrop<F>)) };
    f(ptr)
}

unsafe fn raw_alloca(
    size: usize,
    f: extern "C" fn(*mut MaybeUninit<u8>, *mut c_void),
    data: *mut c_void,
) {
    use std::arch::asm;

    /*
    rbp = rsp
    rsp -= floor16((16 - 1) + size);
    f(rsp, data);
    rsp = rbp;
    */

    unsafe {
        asm!(
            // adding 15 makes the next operation round-up instead of round-down
            "add r12, 15",
            // sets the last 4 bits to zero (effectively rounds-down to 16)
            // it needs to be rounded to 16 because stack alignment
            "and r12, 0xfffffffffffffff0",

            // rsp -= size
            // rdi = rsp (rdi will be the first parameter for the following function call)
            "sub rsp, r12",
            "mov rdi, rsp",

            // call the function with rdi (ptr) and rsi (data)
            "call {f}",

            // restore stack
            "add rsp, r12",

            out("rdi") _,
            in("rsi") data,
            in("r12") size, // any callee-saved register will do
            f = in(reg) f,
            clobber_abi("C"),
        )
    };
}
