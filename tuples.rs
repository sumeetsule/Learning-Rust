/*
#[allow(unused_variables, unused_mut)]
fn main() {
    //define a tuple
    let person_data = ("Rachel", 30, "50kg", "5.4ft");
    // define a tuple with type annotated
    let person_data2 : (&str, i32, &str, &str) = ("Ross", 31, "55kg", "5.8ft");
    
    println!("{}",person_data.0);
    println!("{}",person_data.1);
    println!("{}",person_data2.0);
} */

/* 
#[allow(unused_variables, unused_mut)]
fn main() {
    //define a tuple
    let person_data = ("Rachel", 30, "50kg", "5.4ft");
    
    let (name,age,wt,ht) = person_data;
    
    println!("{}",name);
    println!("{}",age);
    println!("{}",wt);
    println!("{}",ht);
}*/


/* 
fn main() {
    //define a tuple
    let mut person_data = ("Rachel", 30, "50kg", "5.4ft");
    //print the value of tuple
    println!("The value of the tuple at index 0 and index 1 are {} {}", person_data.0, person_data.1);
    //modify the value at index 0
    person_data.0 = "Monica";
    //print the modified value
    println!("The value of the tuple at index 0 and index 1 are {} {}", person_data.0, person_data.1);
}*/


fn main() {
    //define a tuple
    let person_data = ("Rachel", 30, "50kg", "5.4ft");
    //print the value of tuple
    println!("Tuple - Person Data : {:?}", person_data);
}


