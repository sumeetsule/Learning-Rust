/*
fn main() {
    let ages = [27, 35, 40, 10, 19];
    let mut index = 0;

    while index < ages.len() {
        let age = ages[index];
        println!("Age = {:?}", age);
        index += 1;
    }
}*/


// Using Iterator
/*
fn main() {
    let ages = [27, 35, 40, 10, 19];
    let ages_iterator = ages.iter();

    for age in ages_iterator {
        println!("Age = {:?}", age);
    }
}*/


/*
fn main() {
    let ages = [27, 35, 40, 10, 19];
    let mut ages_iterator = ages.iter();
    
    println!("{:?}",ages_iterator.next());
    println!("{:?}",ages_iterator.next());
    println!("{:?}",ages_iterator.next());
    println!("{:?}",ages_iterator.next());
    println!("{:?}",ages_iterator.next());
    println!("{:?}",ages_iterator.next());

}*/


/* 
fn main() {
    let ages = [27, 35, 40, 10, 19];
    let mut ages_iterator = ages.iter();

    while let Some(x) = ages_iterator.next(){
        println!("{:?},{}",Some(x),x);
    }
}*/


fn main() {
    let ages = [27, 35, 40, 10, 19];
    let mut ages_iterator = ages.iter();

    // Looping thru Array
    for age in ages {
        println!("Age = {:?}", age);
    }

    println!("{:?}", ages_iterator.next());
    
    // Looping thru an Iterator. See the additional functionality it offers
    
    for age in ages_iterator {
        println!("Age = {:?}", age);
    }
}