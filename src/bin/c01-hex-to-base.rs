extern crate cryptopals;

use cryptopals::utils::{encode_base64, decode_hex};

fn main() {
    let input = "49276d206b696c6c696e6720796f7572\
                 20627261696e206c696b65206120706f\
                 69736f6e6f7573206d757368726f6f6d";
    println!("Hexed:   {}", input);
    let bytes = decode_hex(input);
    println!("Encoded: {}", encode_base64(&bytes));
}
