// Some code
// This is string object which is stored in heap
/*
fn main(){
    let s1=String::from("hello");
    println!("{}",s1)
}*/


// Move Trait (Heap)
/*
fn main() {
    let mut name = String::from("Hello World");
    println!("Memory address of name: {},{:p},{:p} \n", name,&name,name.as_ptr());

    //moving

    let name1 = name;
    println!("Memory address of name1: {},{:p},{:p} \n", name1,&name1,name1.as_ptr());
    
    //println!("Memory address of name: {},{:p} \n", name,&name);

    // Setting up another Value for the variable name

    name = String::from("Dear World");
    println!("Memory address of name: {},{:p},{:p}\n", name,&name,name.as_ptr());
}*/




// Copy Trait (Stack - because of using String Literal)

fn main() {
    let name = "Hello World";
    println!("Memory address of name: {},{:p},{:p} \n", name,&name,name.as_ptr());

    //Copying

    let name1 = name;
    println!("Memory address of name1: {},{:p},{:p} \n", name1,&name1,name1.as_ptr());
    
    println!("Memory address of name: {},{:p},{:p} \n", name,&name,name.as_ptr());
}