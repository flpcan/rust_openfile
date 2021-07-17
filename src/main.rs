use std::fs::File;
use std::io::prelude::*;

// We read the file with the file::open and then assign this values
// to the contents empty string. We assign as read_to_string
// Then print the file contents \n for new line
fn main() {
    let mut file = File::open("info.txt").expect("Can't open file!");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Can not read the file...");

    println!("FIle Contents: \n\n {}",contents);

}
