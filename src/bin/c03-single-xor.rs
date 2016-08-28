extern crate cryptopals;

use cryptopals::utils::decode_hex;
use cryptopals::utils::reverse_xor;

fn main() {
    let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let input_bytes = decode_hex(input);
    
    match reverse_xor(&input_bytes) {
        Some(c) => println!("{}", c.plaintext),
        None    => println!("None found"),
    }
}
