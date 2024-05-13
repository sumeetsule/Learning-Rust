/*fn main() {
    let language = "Rust"; // define a variable
    println!("Language: {}", language); // print the variable
}*/

// How to create a mutable
/*fn main() {
    let mut language = "Rust"; // define a mutable variable
    println!("Language: {}", language); // print the variable
    language = "Java"; // update the variable
    println!("Language: {}", language); // print the updated value of variable
}*/

// Assigning multiple values
/*fn main() {
    let (fname,lname) =("Sumeet","Sule"); // assign multiple values
    println!(" Student Name is {} {}.", fname,lname); // print the value
}*/

// If variables are unassigned or unused compiler will generate warning
/*#[allow(unused_variables, unused_mut)]
fn main() {
    let (fname,lname,mi) =("Sumeet","Sule",""); // assign multiple values
    println!(" Student Name is {} {}.", fname,lname); // print the value
}*/

// Variable scope
/*fn main() {
let outer_variable = 112;
let inner_variable = 213;
{ // start of code block
      println!("block variable inner: {}", inner_variable);
      println!("block variable outer: {}", outer_variable);
} // end of code block
  println!("inner variable: {}", inner_variable);
}*/


// Shadowing
/*fn main() {
    let outer_variable = 112;
    {
        // start of code block
        let inner_variable = 213;
        println!("block variable: {}", inner_variable);
        let outer_variable = 117;
        println!("block variable outer: {}", outer_variable);
    } // end of code block
    println!("outer variable: {}", outer_variable);
}*/


// Shadowing

fn main() {
    let spaces = "Testing";
    println!("{:?}",spaces);
    let spaces = spaces.len();
    println!("{:?}",spaces);
 }


