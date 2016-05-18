extern crate cryptopals;

use cryptopals::utils::{xor_buffers, decode_hex, encode_hex};

fn main() {
    let lhs = "1c0111001f010100061a024b53535009181c";
    let rhs = "686974207468652062756c6c277320657965";
    
    let lhs_bytes = decode_hex(lhs);
    let rhs_bytes = decode_hex(rhs);
    
    let xor_bytes = xor_buffers(&lhs_bytes, &rhs_bytes);
    
    println!("{}", encode_hex(&xor_bytes));
}