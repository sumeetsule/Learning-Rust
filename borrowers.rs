// Print Name - The second print fails because the name transferred its ownership to p_name
/*
fn printname(p_name:String){
    println!("{name}")
}

fn main() {
    let name:String = String::from("Rachel");
    printname(name);
    printname(name);
} */


// How to fix the above error
// Instead of passing the actual value, passing the Reference / Borrow operator
/*
fn printname(p_name:&String){
    println!("{p_name}")
}

// by default len() returns usize.

fn get_length(p_name: &String) -> i8 {
    let name_length:i8 = p_name.len() as i8;
    name_length
}

fn main() {
    let name:String = String::from("Rachel");
    printname(&name);
    printname(&name);
    let name_length = get_length(&name);
    println!("Length is {name_length}");
}*/




// String.clone()
/*
fn printname(name:String){
    println!("{},{:p},{:p}",name,&name,name.as_ptr())
}

fn main() {
    let name:String = String::from("Rachel");
    printname(name.clone());
    printname(name.clone());
    printname(name);
}*/





// Cannot change as it's not a mutable reference
/*
fn changeme(param_msg: &String) {
    param_msg.push_str(" Green")
}

fn main() {
    let msg = String::from("Rachel");
    changeme(&msg);
}*/




// Mutable reference
/*
fn changeme(param_msg: &mut String) {
    param_msg.push_str(" Green")
}

fn main() {
    let mut msg = String::from("Rachel");
    changeme(&mut msg);
    println!("{}", msg);
}*/



// Immutable Borrow - Stack
/*
fn main() {
    let x = 5;
    
    // Immutable borrow
    let y = &x;
    //*y += 1;
    println!("Value of y: {}", y);
   
}  */
*/

/*

fn main() {
    let mut x = 5;
    
    // Mutable borrow
    let z = &mut x;
    *z += 1;
    
    println!("Value of z: {}->{:p}", z, &z);
    
    println!("Value of x: {}->{:p}", x, &x);
    
    // immutable borrows
    let y1 = &x;
    println!("Value of y1: {}->{:p}", y1, &y1);
    
    // Flip the immutable and mutable and print the Immutable value after Mutable
  
} */



// Borrow Checker - String - Heap
/*
fn main() {
    let mut a = String::from("Rachel");
    
    let b = &mut a;
    
    println!("variable 'b' initial value is {} stored at this {:p}", b,b.as_ptr());
    
    b.push_str(" Green");
    
    //println!("variable 'a' {}{:p}", a,a.as_ptr());
    
    println!("variable 'b' {} {:p}", b,b.as_ptr());
    
    //println!("variable 'a' {}{:p}", a,a.as_ptr());
    
    b.push_str(" !");
    
    println!("variable 'b' after update {},{:p}", b,b.as_ptr());
    
    println!("variable 'a' after update {} {:p}", a,a.as_ptr());
} */



// Not allowed to have multiple Mutable Borrowers at the same time/scope
/*
fn main() {
    let mut a = String::from("Rachel");
    
    let b = &mut a;
    let c = &mut a;
    
    println!("{}", b);
    println!("{}", c);
   
} */



// To rectify the above code
// Multiple Mutable Borrowers (different scope)
/*
fn main() {
    let mut a = String::from("Rachel");
   
    let b = &mut a;
    println!("{}", b);

    let c = &mut a;
    println!("{}", c);
} */




// Immutable and Mutable
// Mutable  first, followed by Immutable
fn main() {
    let mut a = String::from("Rachel");
    
    let c = &mut a;
    c.push_str("!");
    println!("c = {}", c);
    
    let b = &a;
    println!("b = {}", b);
}