extern crate cryptopals;

use std::io;
use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;

use cryptopals::utils::decode_hex;
use cryptopals::utils::reverse_xor;

struct Candidate {
    score: f32,
    original: String,
    plaintext: String,
}

fn iterate_file(f: File) -> Option<Candidate> {
    let reader = BufReader::new(f);
    let mut candidate: Option<Candidate> = None;
    let mut max_score = 0.0;

    for line in reader.lines() {
        let hex_text = match line {
            Ok(t) => t,
            Err(e) => panic!("Error?!! {}", e)
        };

        // 1) Convert line -> bytes
        let input_bytes = decode_hex(&hex_text); 

        // 2) Determine best 'decryption' (decrypted text, score)
        match reverse_xor(&input_bytes) {
            Some(c) => {
                println!("Possibility: {}", c.plaintext);
                if c.score > max_score {
                    candidate = Some(Candidate {score: c.score, original: hex_text, plaintext: c.plaintext});
                    max_score = c.score;
                }
            },
            None => {
                //println!("Nothing good was found?");
            }
        }
    }

    return candidate;
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
        Some(c) => {
            println!("Possible best match");
            println!("   Score:    {}", c.score);
            println!("   Original: {}", c.original);
            println!("   Plain:    {}", c.plaintext);
        },
        None => {
            println!("Nothing was found?");
        }
    }
}
