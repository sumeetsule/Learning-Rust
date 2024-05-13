// \t \n
/*fn main() {
    print!("\t first line is tabbed \nand second line is on a new line");
}*/


// Escape Characters
/*fn main() {
    println!("Here are two escape characters: \\n and \\t");
}*/


// Print multiple \\ , "
/*fn main() {
    println!("File \"folder location is at c:\\users\\Sumeet\\Documents\\01.rs.\" ") 
}*/


// r#  and  #  
/*fn main() {
    println!(r#"File "folder location is at c:\users\Sumeet\Documents\01.rs." "#) 
}*/


// # & ##
fn main() {
    let hashtag_string = r##"The hashtag #IceToSeeYou had become very popular."##; // Has one # so we need at least ##
    let many_hashtags = r####""You don't have to type ### to use a hashtag. You can just use #.""####; // Has three ### so we need at least ####

    println!("{}\n{}\n", hashtag_string, many_hashtags);
}


