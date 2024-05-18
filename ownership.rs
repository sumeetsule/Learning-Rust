// Demonstrate Shadowing

fn main() {
    let x = String::from("Rachel");
    println!("Memory address of x: {:p},{:p}", &x,x.as_ptr());
    
    // new x is created in another memory location
    let x = "Hello";
    println!("Memory address of x: {:p},{:p}", &x,x.as_ptr());
}