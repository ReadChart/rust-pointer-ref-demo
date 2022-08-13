use std::mem::size_of;

///
// var a (unsigned integer)
//   Addr: 0x7fff24b135d0
//   Size: 8 bytes
//   Value: 42
//
// var b (Boxed Variable) <- Smart Pointer
//   Addr: 0x7fff24b135d8
//   Size: 16 bytes <- Fat pointer?
//   Pointing to: 0x55ced21eaba0 <- clearly not pointing to static Variable B, but the boxed B, the B was wrapped and placed in stack
//
// var c (Reference)
//   Addr: 0x7fff24b135f8 <- ref variable location
//   Size: 8 bytes
//   Pointing to: 0x55ced03d80fa <- reference to
//
// var B (10 bytes array)
//   Addr: 0x55ced03d80f0
//   Size: 10 bytes
//   Value: [99, 97, 114, 114, 121, 116, 111, 119, 101, 108]
//
// var C (11 bytes Number)
//   Addr: 0x55ced03d80fa
//   Size: 11 bytes
//   Value: [99, 97, 114, 114, 121, 116, 111, 119, 101, 108, 0]

static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
static C: [u8; 11] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108, 0];
fn main() {
    let a:usize = 42;
    let b: Box<[u8]> = Box::new(B);
    let c: &[u8; 11] = &C;


    println!("var a (unsigned integer)");
    println!("  Addr: {:p}", &a);
    println!("  Size: {:?} bytes", size_of::<usize>());
    println!("  Value: {:?}\n", a);

    println!("var b (Boxed Variable)");
    println!("  Addr: {:p}", &b);
    println!("  Size: {:?} bytes", size_of::<Box<[u8]>>());
    println!("  Pointing to: {:p}\n", b);

    println!("var c (Reference)");
    println!("  Addr: {:p}", &c);
    println!("  Size: {:?} bytes", size_of::<Box<[u8; 11]>>());
    println!("  Pointing to: {:p}\n", c);

    println!("var B (10 bytes array)");
    println!("  Addr: {:p}", &B);
    println!("  Size: {:?} bytes", size_of::<[u8;10]>());
    println!("  Value: {:?}\n", B);

    println!("var C (11 bytes Number)");
    println!("  Addr: {:p}", &C);
    println!("  Size: {:?} bytes", size_of::<[u8;11]>());
    println!("  Value: {:?}\n", C);
}