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

fn printname(name:String){
    println!("{},{:p},{:p}",name,&name,name.as_ptr())
}

fn main() {
    let name:String = String::from("Rachel");
    printname(name.clone());
    printname(name.clone());
    printname(name);
}

