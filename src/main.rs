use std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();
    
    let query: &String = &args[1];
    let filename: &String = &args[2];

    println!("Query: {:?}", query);
    println!("Filename {:?}", filename);

    let contents: String = fs::read_to_string(filename)
        .expect("Something gone wrong while reading the file.");

    println!("Contents {:?}", contents);
}