
static CHARS: &'static[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
static HEX_CHARS: &'static [u8; 16] = b"0123456789abcdef";

pub fn encode_base64(bytes: &[u8]) -> String {
    let mut s = Vec::with_capacity((bytes.len() / 3) * 4);
    for chunk in bytes.chunks(3) {
        let bitpattern = match chunk.len() {
            3 => ((chunk[0] as u32) << 16) | ((chunk[1] as u32) << 8) | ((chunk[2] as u32)),
            2 => ((chunk[0] as u32) << 16) | ((chunk[1] as u32) << 8) | (0 as u32),
            1 => ((chunk[0] as u32) << 16) | (0 as u32),
            _ => panic!("Invalide chunk size {}", chunk.len())
        };
        for i in (3-chunk.len()..4).rev() {
            s.push( CHARS[((bitpattern >> 6*i) & 0x3F) as usize]);
        }
        for _ in 0..3-chunk.len() { s.push(b'='); }
    }
    
    String::from_utf8(s).unwrap()
}


pub fn decode_hex(x: &str) -> Vec<u8> {
    fn convert(byte: u8) -> u8 {
        match byte {
            b'a'...b'f' => 10 + byte - b'a',
            b'A'...b'F' => 10 + byte - b'A',
            b'0'...b'9' => byte - b'0',
            _ => panic!("invalid hex character {}", byte as char)
        }
    }
    x.as_bytes()
     .chunks(2)
     .map(|byte| {(convert(byte[0]) << 4 | convert(byte[1]))})
     .collect()
}

pub fn encode_hex(bytes: &[u8]) -> String {
    // Build up the string as UTF8 bytes, then convert
    // into a string. It's a verbose workaround the lack
    // of indexer within Strings in Rust.
    let mut s = Vec::with_capacity(bytes.len() * 2);
    for byte in bytes {
        s.push(HEX_CHARS[(byte >> 4) as usize]);
        s.push(HEX_CHARS[(byte & 0x0F) as usize]);
    }
    
    String::from_utf8(s).unwrap()
}


pub fn xor_buffers(lhs: &[u8], rhs: &[u8]) -> Vec<u8> {
    assert!(lhs.len() == rhs.len());
    lhs.iter().zip(rhs.iter()).map(|(l,r)| l ^ r).collect()
}

pub fn score_bytes(input: &[u8]) -> f32 {
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

pub fn single_xor(key: u8, bytes: &[u8]) -> Vec<u8> {
    bytes.iter().map(|byte| key ^ byte).collect()
}

