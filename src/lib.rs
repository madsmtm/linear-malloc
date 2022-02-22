//! An ultra simple single-threaded linear allocator.
//!
//! Useful for applications running under cachegrind/callgrind.

use core::ffi::c_void;
use core::ptr;

const HEAP_SIZE: usize = 1024 * 1024 * 128; // 128 Mbit
const MAX_ALIGN: usize = 16;

static mut HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];
static mut CURRENT: *mut u8 = unsafe { &HEAP[0] as *const _ as *mut _ };

#[no_mangle]
pub unsafe extern "C" fn malloc(size: usize) -> *mut c_void {
    let prev = CURRENT;

    let new_pos = CURRENT.add(MAX_ALIGN);
    let new_pos = new_pos.add(size);
    let new_pos = new_pos.add(new_pos.align_offset(MAX_ALIGN));
    let offset = new_pos.offset_from(&HEAP[0]); // This is UB
    if offset > HEAP_SIZE as isize {
        // TODO: Better error handling
        libc::exit(1);
    }
    CURRENT = new_pos;

    ptr::write(prev as *mut usize, size);
    let prev = prev.add(MAX_ALIGN);
    prev as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn free(_ptr: *mut c_void) {
    // Noop
}

#[no_mangle]
pub unsafe extern "C" fn calloc(count: usize, size: usize) -> *mut c_void {
    malloc(count * size)
}

#[no_mangle]
pub unsafe extern "C" fn realloc(old: *mut c_void, new_size: usize) -> *mut c_void {
    let new = malloc(new_size);
    if !old.is_null() {
        let old_size = ptr::read((old as *mut u8).sub(MAX_ALIGN) as *mut usize);
        ptr::copy_nonoverlapping(old as *mut u8, new as *mut u8, old_size.min(new_size));
        free(old);
    }
    new
}
