mod set1;

use set1::single_byte_cipher::brute_force_ciphertext_list;

pub fn main() {
    let (cipher, (plaintext, _)) = brute_force_ciphertext_list();
    println!("Cipher: {cipher}\nPlaintext: {plaintext}");
}
