// Vector
/*
fn main(){
    let mut my_vec = Vec::new();
    my_vec.push("Rachel");
    my_vec.push("Monica");
    my_vec.push("Phoebe");
    
    println!("{:?}",my_vec);
       
} */

/*
fn main(){
    let mut my_vec = Vec::new();
    my_vec.push("Rachel");
    my_vec.push("Monica");
    my_vec.push("Phoebe");
    
    println!("{:?},{:p},{:p}",my_vec,&my_vec,my_vec.as_ptr());
       
}*/


/*
// &str

fn main(){
    let mut my_vec: Vec<&str> = Vec::new();
    my_vec.push("Rachel");
    my_vec.push("Monica");
    my_vec.push("Phoebe");
    
    println!("{:?}",my_vec);
    
}*/


// Vec macro
/*
fn main(){
    //Using vec macro
    let my_vec = vec![2,4,6,8,10,12,14,16];
    
    let one = &my_vec[2..6];
    let two = &my_vec[2..];
    let three = &my_vec[..6];
    let four = &my_vec[..];
    
    println!("{:?}",one);
    println!("{:?}",two);
    println!("{:?}",three);
    println!("{:?}",four);
}*/


/*
// capacity() number of elements the vector can hold (without reallocating memory). This is usually larger than or equal to the number of elements currently in the vector.
// len() number of elements

fn main(){
    let mut my_vec: Vec<String> = Vec::new();
    my_vec.push("Rachel".to_string());
    my_vec.push("Monica".to_string());
    my_vec.push("Phoebe".to_string());

    println!("{:?}",my_vec);
    //Initial capacity was 4
    println!("{}",my_vec.capacity());
    println!("{}",my_vec.len());
    
    // now Rust is allocating space 100 more elements. Now the total capacity will be 103. 
    my_vec.reserve(100);
    
    println!("{}",my_vec.capacity());
    println!("{}",my_vec.len());
    
}*/

/*
fn main(){
    let arr1 = [1,2,3,4];
    let my_vec:Vec<i8> = arr1.into();
    let my_vec1:Vec<_> = arr1.into();
    
    println!("{:?},{:?},{:?}",arr1,my_vec,my_vec1);
    
    print_type_of(&arr1);
    print_type_of(&my_vec);
    print_type_of(&my_vec1);
}


fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}*/


fn main() {
    let mut vec = vec![14, 33, 12, 56, 3223, 2211, 9122, 3, 299, 67];
    vec.sort();
    println!("Sorted: {:?}", vec)
}