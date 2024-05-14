/*
fn main() {
    let my_tuple = (42, "hello", ());
    println!("Tuple: {:?}", my_tuple);
}*/

fn main() {
    let result = do_nothing();
    println!("Result of do_nothing: {:?}", result);
}

fn do_nothing() -> () {
    // This function does nothing and returns unit type
}
