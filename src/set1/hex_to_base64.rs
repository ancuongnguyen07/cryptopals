use hex::{encode, decode};
use base64::{Engine as _, engine::general_purpose};

const LETTER_FREQ: [f64; 27] = [
    0.08167, 0.01492, 0.02782, 0.04253, 0.12702, 0.02228, 0.02015, // A-G
    0.06094, 0.06966, 0.00153, 0.00772, 0.04025, 0.02406, 0.06749, // H-N
    0.07507, 0.01929, 0.00095, 0.05987, 0.06327, 0.09056, 0.02758, // O-U
    0.00978, 0.02360, 0.00150, 0.01974, 0.00074, 0.19181, // V-Z & space char
];

#[allow(dead_code)]
pub fn hex_to_base64(hex_str: &str) -> String {
    general_purpose::STANDARD.encode(hex::decode(hex_str).unwrap())
}

#[allow(dead_code)]
pub fn xor(hex_a: &str, hex_b: &str) -> String {
    let bytes_a = decode(hex_a).unwrap();
    let bytes_b = decode(hex_b).unwrap();

    encode(bytes_a.iter()
            .zip(bytes_b.iter())
            .map(|(&ba, &bb)| ba ^ bb)
            .collect::<Vec<u8>>()
    )
}

pub fn calc_score_freq(message: &str) -> f64 {
    let mut counts = vec![0_u32; 27];
    
    for &byte in message.as_bytes() {
        match byte {
            b'a'..=b'z' => counts[(byte - b'a') as usize] += 1,
            b'A'..=b'Z' => counts[(byte - b'A') as usize] += 1,
            b' ' => counts[26] += 1,
            _ => (),
        }
    }

    (0..27)
        .fold(0_f64, |acc, i| acc + counts[i] as f64 * LETTER_FREQ[i])
}

pub fn decrypt_xor_cipher(cipher_str: &str) -> (String, f64) {
    let cipher_bytes = decode(cipher_str).unwrap();
    let mut highest_score = 0_f64;
    let mut plaintext = String::new();

    for byte in 0_u8..255_u8 {
        let plain_bytes = cipher_bytes.iter()
                                    .map(|&b| b ^ byte)
                                    .collect::<Vec<u8>>();
        let msg = String::from_utf8_lossy(&plain_bytes);
        let score = calc_score_freq(&msg);

        if score > highest_score {
            highest_score = score;
            plaintext = String::from(msg);
        }
    }

    (plaintext, highest_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_hex_base64 () {
        let hex_str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        assert_eq!(expected.to_string(), hex_to_base64(hex_str));
    }

    #[test]
    fn test_valid_xor () {
        let hex_a = "1c0111001f010100061a024b53535009181c";
        let hex_b = "686974207468652062756c6c277320657965";
        let expected = "746865206b696420646f6e277420706c6179";
        assert_eq!(xor(hex_a, hex_b), expected.to_string());
    }
}
