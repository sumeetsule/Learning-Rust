// Copy Trait
fn main() {
    // i32 is a simple type and are normally stored on the stack.
    // copy trait

    let x = 42;
    let y = &x; 

    // The value bound to x is Copy, so no error will be raised.
    println!("{:?}", (x, y));

    // The value bound to x is Copy, so no error will be raised.
    println!("x: {:?},{:p}",&x, &y);
} 



