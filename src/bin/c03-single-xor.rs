extern crate cryptopals;

use cryptopals::utils::decode_hex;
use std::collections::HashMap;

fn single_xor(key: u8, bytes: &[u8]) -> Vec<u8> {
    bytes.iter().map(|byte| key ^ byte).collect()
}

fn score(input: &[u8]) -> i32 {
    let mut progress = 0;
    for byte in input {
        if !(32 <= *byte && *byte <= 127) {
            return -1
        }
        progress += 1
    }
    return progress;
}

fn frequency(input: &[u8]) -> HashMap<u8, u32> {
    let mut blah: HashMap<u8, u32> = HashMap::new();
    
    for byte in input {
        let mut count = blah.entry(*byte).or_insert(0);
        *count = *count + 1;
    }
    
    return blah;
}

fn main() {
    let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let input_bytes = decode_hex(input);
    
    let mut candidates: Vec<String> = Vec::new();
    
    for key in 0..255 {
        let decoded = single_xor(key, &input_bytes);
        if score(&decoded) > 0 {
            let frequencies = frequency(&decoded);
            let candidate = String::from_utf8(decoded).unwrap();

            println!("Key {} gives candidate: {}", key, candidate);
            for (char, freq) in &frequencies {
                println!("  {} occurs {}", char, freq);
            }
            candidates.push(candidate);
        }
        
    }
    
    /*
    for candidate in candidates {
        println!("Candidate: {}", candidate);
    }
    */
}
