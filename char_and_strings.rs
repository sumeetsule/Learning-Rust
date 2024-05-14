/*
fn main() {
    // implicitly & explicitly define
    let char_2:char = 'a';
    let char_3 = 'b';
    println!("character2: {}", char_2);
    println!("character3: {}", char_3);
} */

// String literal
/*fn main() {
    // explicitly define
    let str_1:&str = "Rust Programming";
    println!("String 1: {}", str_1);

    // implicitly define
    let str_2 = "Rust Programming";
    println!("String 2: {}", str_2);
}*/

// String object
/*
fn main(){
    let empty_string = String::new();
    println!("length is {}",empty_string.len());

    let content_string = String::from("Rachel Green");
    println!("length is {}",content_string.len());
} */

// String operations
// Push Single Character
/*
fn main(){
    let mut name1 = String::from("Hello");
    println!("{}",name1);
    name1.push('!');
    println!("{}",name1);
} */

// Push a string
/*
fn main(){
    let mut name1 = String::from("Hello");
    println!("{}",name1);
    name1.push_str(" World");
    println!("{}",name1);
} */

/*
fn main() {
    let name1 = String::from("Hello!");
    let name2 = name1.replace("Hello", "Howdy"); //find and replace
    println!("{}", name2);
} */


// Convert string literal to object
/*
fn main(){
    let name1 = "Hello!".to_string();              //String object
    let name2 = name1.replace("Hello","Howdy");    //find and replace
    println!("{}",name2);
} */

// Convert string object to literal
/*
fn main() {
    let name1 = String::from("hello");
    let name2 = name1.as_str();
    println!("{},{}", name1, name2);
} */

/*
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let name = "StringSample";
    let name1 = String::from("Hello");
    print_type_of(&name);
    print_type_of(&name1);
} */


// concatenation
/*
fn main() {
    let str1 = "Hello".to_string();
    let str2 = " world".to_string();
    let string = str1 + &str2;
    println!("{}", string);
} */

// Reverse String
/*
fn main() {
    let s = "Hello World";
    let t: String = s.chars().rev().collect();
    println!("{}", t);
} */


// Palindrome
/*
fn main() {
    let s = "rotator";
    let t: String = s.chars().rev().collect();
    
    if s == t {
        println!("Palindrome")
    }
    else{
        println!("Not Palindrome")
    }
} */


 // String Padding

 fn main() {
    let s = "pizza";
    
    println!("{s:-^30}");
    println!("{s:*<30}");
    println!("{s:#>30}");
}



