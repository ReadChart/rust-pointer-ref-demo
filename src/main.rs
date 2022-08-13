/// ## Raw Pointers
///
/// unsafe access
///
/// immutable raw pointer: `*const Type`
///
/// mutable raw pointer: `*mut Type`
///
/// Note: `*const Type` is a pointer points to a specific type
///
/// For example:
///
/// `*const String`: an immutable pointer points to a String;
///
/// Also: `*const Type` and `*mut Type` can be transform into each other and they just have slight difference
///
/// References in rust will be compiled to raw pointers,
///
/// which means they needn't to be declared within unsafe blocks,
///
/// but they still has the performance as raw pointers
///
///
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