/*
#[derive(Debug)]
enum TrafficLight{
    Red,
    Yellow,
    Green
}

fn main(){
    let my_light = TrafficLight::Red;

    println!("{:?}", my_light);
}*/

/*
// Enum with additional data
#[derive(Debug)]
enum TrafficLight{
    Red (bool),
    Yellow (bool),
    Green (bool)
}

fn main(){
    let my_light = TrafficLight::Red(true);

    println!("{:?}", my_light);
}*/


/*
// Separate the value and enum result

#[derive(Debug)]
enum TrafficLight{
    Red (bool),
    Yellow (bool),
    Green (bool)
}

fn main(){
    let my_light = TrafficLight::Red(false);

    println!("{:?}", my_light);
    
    match my_light {
        TrafficLight::Red(is_active) => println!("Red: {}", is_active),
        TrafficLight::Yellow(is_active) => println!("Yellow: {}", is_active),
        TrafficLight::Green(is_active) => println!("Green: {}", is_active),
    }
}*/


/*
#[derive(Debug)]
enum TrafficLight {
    Red(bool),
    Yellow(bool),
    Green(bool),
}

#[derive(Debug)]
enum Vehicle {
    Stop,
    Drive(f64),
    CheckLight(TrafficLight),
}

fn main() {
    let my_light = TrafficLight::Red(true);
    let instruction = Vehicle::CheckLight(my_light);

    match instruction {
        Vehicle::Stop => println!("Stop"),
        Vehicle::Drive(speed) => println!("Drive at speed: {}", speed),
        Vehicle::CheckLight(light) => match light {
            TrafficLight::Red(is_active) => println!("Red light: {}", is_active),
            TrafficLight::Yellow(is_active) => println!("Yellow light: {}", is_active),
            TrafficLight::Green(is_active) => println!("Green light: {}", is_active),
        },
    }
}*/



// Error Handling

// An integer division that doesn't `panic!`
fn checked_division(dividend: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        // Failure is represented as the `None` variant
        None
    } else {
        // Result is wrapped in a `Some` variant
        Some(dividend / divisor)
    }
}

fn try_division(dividend: i32, divisor: i32) {
    // `Option` values can be pattern matched, just like other enums
    match checked_division(dividend, divisor) {
        None => println!("{} / {} failed!", dividend, divisor),
        Some(quotient) => {
            println!("{} / {} = {}", dividend, divisor, quotient)
        },
    }
}

fn main() {
    try_division(4, 2);
    //try_division(4, 0);
    
}
