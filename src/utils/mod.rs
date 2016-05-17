
static CHARS: &'static[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

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