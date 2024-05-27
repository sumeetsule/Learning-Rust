// String Slicing
/*
fn main() {
    //define an array of size 4
    let arr:[i32;7] = [1, 2, 3, 4,5,6,7]; 
    
    //define the slice
    let slice_array1 = &arr;
    let slice_array2 = &arr[0..4];
    let slice_array3 = &arr[3..];
    
    // print the slice of an array
    println!("Value of slice_array1: {:?}", slice_array1);
    println!("Value of slice_array2: {:?}", slice_array2);
    println!("Value of slice_array3: {:?}", slice_array3);
}*/

/*
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn main() {
    let s = String::from("hello world");
    let word = first_word(&s); // word will get the value 5
    println!("{}",word);
}*/



// return value

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b'd' {
            return &s[0..i+1];
        }
    }

    &s[..]
}

fn main() {
    let s = String::from("hello world");
    let word = first_word(&s); // word will get the value 5
    println!("{}",word);
}