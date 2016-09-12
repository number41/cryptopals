extern crate rustc_serialize;
extern crate cryptopals;

use std::fs::File;
use std::io::prelude::*;
use rustc_serialize::base64::FromBase64;

use cryptopals::utils::hamming_distance;      
 
fn guess_keysize(buffer: &Vec<u8>, min: usize, max: usize) -> usize {

    let mut keysize: usize = 0;
    let mut best_score = 9999.0;

    for i in min..max+1 {
        if i * 2 > buffer.len() {
            println!("2 * {} is too large for {}", i, buffer.len());
            continue;
        }

        let mut chunks_iter = buffer.chunks(i);
        let first = chunks_iter.next();
        let second = chunks_iter.next();
        if !first.is_some() && !second.is_some() {
            println!("whoops, keysize {} is too large", i);
            continue;
        }

        let dist = hamming_distance(&first.unwrap(), &second.unwrap());
        let score = dist as f32 / i as f32;
        if score < best_score {
            best_score = score;
            keysize = i;
        } 
    }

    println!("Returning keysize {} with score {}", keysize, best_score);
    return 0;
}

fn main() {
    let mut f = match File::open("./datasets/6.txt") {
        Ok(f) => f,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    println!("Opened the file");
    let mut text = String::new();
    match f.read_to_string(&mut text) {
        Err(e) => {
            println!("Error: {}", e);
            return;
        },
        _ => println!("Successfully read in file: length {}", text.len()),
    };

    let buffer = match text.from_base64() {
        Ok(b)  => b,
        Err(e) => {
            println!("Error converting: {}", e);
            return;
        }
    };
    println!("Converted into a buffer of {} bytes", buffer.len());

    /* attempt to find keysizes */
    let likely = guess_keysize(&buffer, 2, 40);
}
