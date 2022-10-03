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
/// References like `&mut T` and `&T` in rust will be compiled to raw pointers,
///
/// which means they needn't to be declared within unsafe blocks,
///
/// but they still has the performance as raw pointers
///
///
fn main() {
    let a: i64 = 42;
    let a_ptr: *const i64 = &a as *const i64;

    println!("a: {} ({:p})", a, a_ptr)
}