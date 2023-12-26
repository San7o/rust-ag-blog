// File I/O
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    
    // FILE I/O -------------------------------------------------------
    // From rust by example

    // Create a path to the desired file
    let path = Path::new("posts/example1.md");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
                // Using markdown library to parse the file
        Ok(_) => println!("{}", markdown::to_html(&s)),
    }



}
