extern crate cryptopals;

use std::io;
use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;


fn iterate_file(f: File) -> io::Result<()> {
    let reader = BufReader::new(f);

    for line in reader.lines() {
        println!("{}", try!(line));
    }

    Ok(())
}

fn main() {
    let f = match File::open("./datasets/4.txt") {
        Ok(f) => f,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    println!("Opened the file!!!!");
    match iterate_file(f) {
        Err(e) => {
            println!("Error: {}", e);
        },
        Ok(_) => {
        }
    }
}
