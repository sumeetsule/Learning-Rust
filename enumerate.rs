/*
fn main() {
    let fruits = ["apple", "banana", "cherry"];

    for i in 0..fruits.len() {
        println!("Index: {}, Fruit: {}", i, fruits[i]);
    }
} */

/*
fn main() {
    let fruits = ["apple", "banana", "cherry"];

    for (index, fruit) in fruits.iter().enumerate() {
        println!("Index: {}, Fruit: {}", index, fruit);
    }
}*/

/*
fn main() {
    for (i, j) in (100..120).enumerate() {
        println!("loop has executed {} times. j = {}", i, j);
    }
}*/



// _ is a generic placeholder.

fn main()  
{ 
    let my_array: [i32; 7] = [1i32,3,5,7,9,11,13]; 
    let mut value = 0i32; 
    for(_, item) in my_array.iter().enumerate() 
    { 
       value += item; 
    } 
    println!("{}", value); 
} 