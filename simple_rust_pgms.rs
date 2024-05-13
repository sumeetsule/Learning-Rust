// Simple rust programs using variables

/*fn main() {
    let name = "Sumeet";
    let age=22;
    println!("Hello {},{}", name,age);

    //change the value of variable
    
    let name = "Sumeet Sule";
    println!("Hello {},{}", name,age);

}*/

// Simple rust programs using multiple variables
/*#[allow(unused_variables, unused_mut)]

fn main() {
    let (fname,lname,mi) =("Sumeet","Sule",""); // assign multiple values
    println!(" Student Name is {} {}.", fname,lname); // print the value
}*/

// For loops
/*fn main() {
    for i in 0..5{
        println!("Hello {}",i);
    }
}*/

// Odd Even
fn main() {
    for i in 0..5{
        if i%2==0{
            println!("Even {}",i);
        } else {
            println!("Odd {}",i);
        }
    }
}

