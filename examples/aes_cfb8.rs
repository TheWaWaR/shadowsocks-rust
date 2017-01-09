
extern crate shadowsocks;

use std::str;
use shadowsocks::crypto::*;
use shadowsocks::crypto::cipher::*;


fn main() {
    let key = CipherType::Aes128Cfb8.bytes_to_key(b"PassWORD");
    let iv = CipherType::Aes128Cfb8.gen_init_vec();
    let mut encryptor = with_type(CipherType::Aes128Cfb8,
                                  &key[0..],
                                  &iv[0..],
                                  CryptoMode::Encrypt);
    let mut decryptor = with_type(CipherType::Aes128Cfb8,
                                  &key[0..],
                                  &iv[0..],
                                  CryptoMode::Decrypt);
    let message = "ABC";      // ASCII=36

    println!("message={}", message);
    let mut encrypted_msg = Vec::new();
    encryptor.update(message.as_bytes(), &mut encrypted_msg).unwrap();
    println!("encrypted_msg({})={:?}", encrypted_msg.len(), encrypted_msg);
    for c in encrypted_msg {
        let mut decrypted_byte = Vec::new();
        decryptor.update(&vec![c], &mut decrypted_byte).unwrap();
        println!("{} => decrypted_byte({})={:?}", c, decrypted_byte.len(),
                 str::from_utf8(&decrypted_byte).unwrap());
    }
}
