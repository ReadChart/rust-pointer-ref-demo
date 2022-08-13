// Copy On Write; (Shadowing?) -> Before the write operation, it reads the same as the original.
use std::borrow::Cow;
//Foreign Function Interface
use std::ffi::CStr;
use std::os::raw::c_char;

static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
static C: [u8; 11] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108, 0];
fn main() {
    let a: i32 = 42;
    let b: String;
    let c: Cow<str>;

    unsafe {
        // Pointer transforming is unsafe operations
        let b_ptr: *mut u8 = &B as *const u8 as *mut u8;
        b = String::from_raw_parts(b_ptr, 10, 10);

        let c_ptr: *const i8 = &C as *const u8 as *const c_char;
        c = CStr::from_ptr(c_ptr).to_string_lossy();
    }
    println!("a: {}, b: {}, c: {}", a, b, c);
}