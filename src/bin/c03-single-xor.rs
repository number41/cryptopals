extern crate cryptopals;

use cryptopals::utils::decode_hex;
use cryptopals::utils::score_bytes;
use cryptopals::utils::single_xor;

fn main() {
    let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let input_bytes = decode_hex(input);
    
    let mut candidate: Option<String> = None;
    let mut max_score = 0.0;
    
    for key in 0..255 {
        let decoded = single_xor(key, &input_bytes);
        let score = score_bytes(&decoded);

        if score > max_score {
            candidate = Some(String::from_utf8(decoded).unwrap());
            max_score = score;
        }
    }

    match candidate {
        Some(c) => println!("{}", c),
        None    => println!("None found"),
    }
}
