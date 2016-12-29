extern crate cryptopals;
extern crate rustc_serialize;

use cryptopals::utils::*;
use rustc_serialize::base64::{FromBase64,ToBase64, STANDARD};

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

#[test]
fn test_base64() {
    let actual = "Hello World!";
    let encoded = "SGVsbG8gV29ybGQh";

    assert_eq!(actual.as_bytes().to_base64(STANDARD), encoded);
    assert_eq!(encoded.from_base64().unwrap(), actual.as_bytes());
}

#[test]
fn test_reverse_xor() {
    let hexed = "7b5a4215415d544115415d5015455447414c155c46155f4058455c5b523f";
    let ciphertext = decode_hex(&hexed);
    let result = reverse_xor(&ciphertext);

    assert!(result.is_some());
    assert_eq!(result.unwrap().plaintext, "Now that the party is jumping\n");
}
