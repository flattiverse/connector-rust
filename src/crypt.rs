use aes::block_cipher_trait::BlockCipher;
use aes::{Aes128, Aes256};
use sha2::digest::generic_array::GenericArray;
use sha2::digest::{FixedOutput, Input};
use sha2::Sha512;

const SLOWNESS_LEN: usize = 80_000_000;

pub fn hash_password(salt: &str, password: &str) -> [u8; 16] {
    let s_data = sha512(&salt.to_lowercase());
    let p_data = sha512(password);

    let mut slowness = vec![0u8; SLOWNESS_LEN];

    for pos in 0..SLOWNESS_LEN / 80 {
        let index = pos * 80;
        slowness[index + 0 * 8..index + 1 * 8].copy_from_slice(&p_data[48..48 + 8]);
        slowness[index + 1 * 8..index + 9 * 8].copy_from_slice(&s_data[0..8 * 8]);
        slowness[index + 9 * 8..index + 10 * 8].copy_from_slice(&p_data[56..56 + 8]);
    }

    let aes = Aes256::new(&GenericArray::from_slice(&p_data[0..32]));
    let mut iv = [0u8; 16];
    iv.copy_from_slice(&s_data[32..32 + 16]);
    aes.encrypt_block(&mut GenericArray::from_mut_slice(&mut iv)); // init value???

    for _ in 0..7 {
        for i in 0..SLOWNESS_LEN / 16 {
            aes.encrypt_block(&mut GenericArray::from_mut_slice(&mut slowness[i..i + 16]));
        }
    }
    aes.encrypt_block(&mut GenericArray::from_mut_slice(&mut slowness[0..16]));
    (&mut iv[0..16]).copy_from_slice(&slowness[0..16]);
    return iv;
}

pub fn sha512(salt: &str) -> Vec<u8> {
    let mut sha512 = Sha512::default();
    sha512.input(salt.as_bytes());
    sha512.fixed_result().to_vec()
}
