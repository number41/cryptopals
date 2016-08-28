extern crate cryptopals;

use cryptopals::utils::{encode_hex, repeating_xor};

fn main() {
    let input = "Burning 'em, if you ain't quick and nimble I go crazy when I hear a cymbal";
    let encrypted = repeating_xor("ICE".as_bytes(), input.as_bytes());
    let hexed = encode_hex(&encrypted);
    println!("Hexed: {}", hexed);
}
