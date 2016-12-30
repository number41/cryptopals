extern crate rustc_serialize;
extern crate cryptopals;

use std::fs::File;
use std::io::prelude::*;
use rustc_serialize::base64::FromBase64;

use cryptopals::utils::hamming_distance;  
use cryptopals::utils::encode_hex;
use cryptopals::utils::reverse_xor; 
use cryptopals::utils::repeating_xor;   
 
fn guess_keysize(buffer: &Vec<u8>, min: usize, max: usize) -> usize {

    let mut keysize: usize = 0;
    let mut best_score = 9999.0;

    for i in min..max+1 {
        if i * 2 > buffer.len() {
            println!("2 * {} is too large for {}", i, buffer.len());
            return keysize;
        }

        let mut chunks_iter = buffer.chunks(i);
        let first = chunks_iter.next();
        let second = chunks_iter.next();
        if !first.is_some() && !second.is_some() {
            println!("whoops, keysize {} is too large", i);
            return keysize;
        }

        let dist = hamming_distance(&first.unwrap(), &second.unwrap());
        let score = dist as f32 / i as f32;
        if score < best_score {
            best_score = score;
            keysize = i;
        } 
    }

    return keysize;
}

fn compute_transposed(buffer: &Vec<u8>, blocksize: usize) -> Vec<Vec<u8>> {
    let mut transposed = Vec::new();
    for _ in 0..blocksize {
        transposed.push(Vec::new());
    }

    for (byte, index) in buffer.iter().zip((0..blocksize).cycle()) {
        transposed[index].push(*byte);
    }

    return transposed;
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
    println!("Using {} as the keysize", likely);
    println!("  cipher     blocks: count {}, size {}", buffer.len() / likely, likely);
    println!("  transposed blocks: count {}, size {}", likely, buffer.len() / likely);

    /* Transpose blocks */
    let transposed = compute_transposed(&buffer, likely);
    for (i, tp) in transposed.iter().enumerate() {
        println!(" Transpose block {}: {} bytes", i, tp.len());
    }

    let mut key = Vec::new();
    for i in 0..transposed.len() {
        if let Some(candidate) = reverse_xor(&transposed[i]) {
            key.push(candidate.key);
        } else {
            println!("Unable to find XOR for transpose block {}", i);
            return;
        }
    }

    let hexed_key = encode_hex(&key);
    println!("Possibly found key {}", hexed_key);

    let decrypted = repeating_xor(&key, &buffer);
    if let Ok(plaintext) = String::from_utf8(decrypted) {
        println!("{}", plaintext);
    } else {
        println!("Unable to make sense of the 'plaintext'. Key is probably wrong");
    }

}
