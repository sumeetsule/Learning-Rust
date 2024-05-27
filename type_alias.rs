// type alias
/*
type Bannertype = u32;

fn main() {
    let mut id: Bannertype = 91615214;
    println!("{id}");
    id = 91615200;
    println!("{id}");
}*/


type Kilometers = i32;
type Meters = i32;

fn calculate_distance(distance: Kilometers) -> Meters {
    distance * 1000
}

fn main() {
    let distance: Kilometers = 5;
    let distance_in_meters: Meters = calculate_distance(distance);
    println!("Distance in meters: {}", distance_in_meters);
}



