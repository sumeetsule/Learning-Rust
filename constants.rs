const ID_1: i32 = 4; // define a global constant variable

fn main() {
    const ID_2: u32 = 3; // define a local constant variable
    println!("ID:{}", ID_1); // print the global constant variable
    println!("ID:{}", ID_2); // print the local constant variable
}