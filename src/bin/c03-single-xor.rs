extern crate cryptopals;

use cryptopals::utils::decode_hex;

fn single_xor(key: u8, bytes: &[u8]) -> Vec<u8> {
    bytes.iter().map(|byte| key ^ byte).collect()
}

fn score_bytes(input: &[u8]) -> f32 {
    let mut progress = 0.0;
    for byte in input {
        if !(32 <= *byte && *byte <= 127) {
            return -1.0
        }
        progress += get_frequency(*byte as char)
    }
    return progress;
}

fn get_frequency(c: char) -> f32 {
    let upper = c.to_uppercase().collect::<String>();
    match upper.as_ref() {
        "A" => 8.2,
        "B" => 1.5,
        "C" => 2.8,
        "D" => 4.3,
        "E" => 12.7,
        "F" => 2.2,
        "G" => 2.0,
        "H" => 6.1,
        "I" => 7.0,
        "J" => 0.2,
        "K" => 0.8,
        "L" => 4.0,
        "M" => 2.4,
        "N" => 6.7,
        "O" => 7.5,
        "P" => 1.9,
        "Q" => 0.1,
        "R" => 6.0,
        "S" => 6.3,
        "T" => 9.1,
        "U" => 2.8,
        "V" => 1.0,
        "W" => 2.4,
        "X" => 0.2,
        "Y" => 2.0,
        "Z" => 0.1,
        "'" => 0.05,
        " " => 10.0,
        "." => 0.2,
        "!" => 0.05,
        "?" => 0.05,
        "\"" => 0.05,
        _ => 0.0,
    }
}

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
