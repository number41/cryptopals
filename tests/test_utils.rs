extern crate cryptopals;

use cryptopals::utils::*;

#[test]
fn test_repeating_xor() {
    let input = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let encrypted = repeating_xor("ICE".as_bytes(), input.as_bytes());
    let hexed = encode_hex(&encrypted);
    assert_eq!("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f", hexed);
}

#[test]
fn test_hamming() {
    let lhs = "this is a test";
    let rhs = "wokka wokka!!!";
    assert_eq!(37, hamming_distance(lhs.as_bytes(), rhs.as_bytes()));
}
