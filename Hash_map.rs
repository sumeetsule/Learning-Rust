// Hashing example
/*
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn main() {
    let s = "hello world";

    // Create a hasher
    let mut hasher = DefaultHasher::new();

    // Hash the string
    s.hash(&mut hasher);

    // Get the resulting hash as a u64
    let hash_value = hasher.finish();

    println!("Hash value for '{}': {}", s, hash_value);
}*/


// Hash Map example
/*
use std::collections::HashMap;

fn main() {
    let mut cities = HashMap::new();
    cities.insert("Glassboro", 30000);
    cities.insert("Mullicahill", 25000);
    cities.insert("Swedesboro", 28000);
   
    println!("cities is {:?}", cities);
}*/


// Insert
/*
use std::collections::HashMap;

fn main() {
    let mut cities = HashMap::new();
    cities.insert("Glassboro", 30000);
    cities.insert("Mullicahill", 25000);
    cities.insert("Swedesboro", 28000);
   
    println!("cities is {:?}", cities);

    //Option 1 - Insert / Overwrite Existing Value

    cities.insert("Paulsboro", 11000);
    cities.insert("Glassboro", 31000);

    println!("cities is {:?}", cities);
}*/


// Insert/Update/Get
/*
use std::collections::HashMap;

fn main() {
    let mut cities = HashMap::new();
    cities.insert("Glassboro", 30000);
    cities.insert("Mullicahill", 25000);
    cities.insert("Swedesboro", 28000);
    
    //Option 1 - Update / Overwrite Existing Value
    cities.insert("Glassboro", 31000);
    
    //Option 2 - Insert a new entry if it doesn't exist
    cities.entry("Deptford").or_insert(12000);
    
    //Option 3 - Get the value of the Key and perform a mathematical operation
    let gpopulation = cities.entry("Glassboro").or_insert(0);
    *gpopulation += 1;
    
    //print the hash map values. The print order may or may not be the same
    //as the insert. It changes from time to time.
    println!("cities is {:?}", cities);

    let glassboro_population = cities.get("Glassboro");
    
    if glassboro_population.is_some(){
        println!("glassboro_population is {:?}", glassboro_population);
    }
    else if glassboro_population.is_none(){
        println!("glassboro_population is not available", );
    }
}*/



// Remove
/*
use std::collections::HashMap;

fn main() {
    let mut cities = HashMap::new();
    cities.insert("Glassboro", 30000);
    cities.insert("Mullicahill", 25000);
    cities.insert("Swedesboro", 28000);

    // Remove an entry
    cities.remove("Mullicahill");

    println!("After removal: {:?}", cities);
}*/


// Iterating over the hash map
/*
use std::collections::HashMap;

fn main() {
    let mut cities = HashMap::new();
    cities.insert("Glassboro", 30000);
    cities.insert("Mullicahill", 25000);
    cities.insert("Swedesboro", 28000);

    // Iterating over the HashMap
    for (city, population) in &cities {
        println!("The population of {} is {}", city, population);
    }
}*/


// Check for existence 
/*
use std::collections::HashMap;

fn main() {
    let mut cities = HashMap::new();
    cities.insert("Glassboro", 30000);
    cities.insert("Mullicahill", 25000);
    cities.insert("Swedesboro", 28000);

    // Checking if a key exists
    if cities.contains_key("Swedesboro") {
        println!("Swedesboro is in the HashMap");
    } else {
        println!("Swedesboro is not in the HashMap");
    }
}*/


// Merge
use std::collections::HashMap;

fn main() {
    let mut cities1 = HashMap::new();
    cities1.insert("Glassboro", 30000);
    cities1.insert("Mullicahill", 25000);

    let mut cities2 = HashMap::new();
    cities2.insert("Swedesboro", 28000);
    cities2.insert("Depford", 12000);

    // Merging two HashMaps
    for (city, population) in cities2 {
        cities1.insert(city, population);
    }

    println!("Merged cities: {:?}", cities1);
}


