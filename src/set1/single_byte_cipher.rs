use crate::set1::hex_to_base64::*;
use std::io::{BufReader, BufRead};
use std::fs::File;

const TEXT_FILE: &str = "txt/4.txt";

#[allow(dead_code)]
pub fn decrypt_ciphertext() {
    // Single-byte XOR cipher
    let cipher = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

    let (plaintext, _) = decrypt_xor_cipher(&cipher);
    println!("{plaintext}");
}

pub fn brute_force_ciphertext_list() -> (String, (String, f64)) {
    let text_file = match File::open(TEXT_FILE) {
        Err(why) => panic!("Couldn't open {TEXT_FILE}: {why}"),
        Ok(f) => f,
    };

    let file_reader = BufReader::new(text_file);

    file_reader.lines()
                .map(|c| {
                    match c {
                        Err(why) => panic!("Couldn't read line: {why}"),
                        Ok(cx) => (cx.clone(), decrypt_xor_cipher(cx.as_str())),
                    }
                })
                .max_by(|(_, (_, score_a)), (_, (_, score_b))| 
                        score_a.partial_cmp(score_b).unwrap())
                .unwrap()
}