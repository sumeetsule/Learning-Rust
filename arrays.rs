/*
fn main() {
    //define an array of size 4
    let arr:[i32;4] = [1, 2, 3, 4]; 
    //print the first element of array
    println!("The first value of array is {}", arr[0]);
    // initialize an array of size 4 with 0
    let arr1 = [10; 14]; 
    //print the first element of array
    println!("The first value of array is {}", arr1[0]);
} */

// Mutable arrays
/*
fn main() {
    //define a mutable array of size 4
    let mut arr:[i32;4] = [1, 2, 3, 4]; 
    println!("The value of array at index 1: {}", arr[1]);
    arr[1] = 9;
    println!("The value of array at index 1: {}", arr[1]);
} */


/*
fn main() {
    //define an array of size 4
    let arr:[i32;4] = [1, 2, 3, 4]; 
    //Using debug trait
    println!("\nPrint using a debug trait");
    println!("Array: {:?}", arr);
} */

/*
fn main() {
    //define an array of size 4
    let arr:[i32;4] = [1, 2, 3, 4]; 
    // print the length of array
    println!("Length of array: {}", arr.len());
} */


fn main() {
    //define an array of size 4
    let arr:[i32;4] = [1, 2, 3, 4]; 
    //define the slice
    let slice_array1:&[i32] = &arr;
    let slice_array2:&[i32] = &arr[0..2];
    let slice_array3:&[i32] = &arr[0..];
    // print the slice of an array
    println!("Slice of an array: {:?}", slice_array1);
    println!("Slice of an array: {:?}", slice_array2);
    println!("Slice of an array: {:?}", slice_array3);
}

