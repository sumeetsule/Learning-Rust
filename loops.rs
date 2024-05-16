// While loop
// Some code
/*
fn main() {
    let mut i = 0;
    while i != 6 {
        i += 1;
        println!("inside loop value of i : {i}")
    }
    println!("finally i is {}", i);
}*/



// Left side Inclusive, Right side exclusive
/* 
fn main() {
    for x in 0..5 {
        print!("{}", x);
    }
    println!();
// By adding =, both sides are inclusive

    for x in 0..=5 {
        println!("{}", x);
    }
}*/


// Some code
fn main() {
    let mut x = 0;
    loop {
        x += 1;
        if x == 7 {
            break;
        }
    }
    println!("{}", x);
}


