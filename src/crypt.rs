use crypto::aes::cbc_encryptor;
use crypto::aes::KeySize::{KeySize128, KeySize256};
use crypto::blockmodes::NoPadding;
use crypto::buffer::{RefReadBuffer, RefWriteBuffer};
use sha2::digest::FixedOutput;
use sha2::{Digest, Sha256, Sha512};

const SLOWNESS_LEN: usize = 80_000_000;
const BLOCK_SIZE: usize = 80;

pub fn hash_password(salt: &str, password: &str) -> [u8; 16] {
    let s_data = sha512(&salt.to_lowercase());
    let p_data = sha512(password);

    let mut slowness = vec![0u8; SLOWNESS_LEN];

    for pos in 0..SLOWNESS_LEN / BLOCK_SIZE {
        let index = pos * BLOCK_SIZE;
        let slowness = &mut slowness[index..index + BLOCK_SIZE];

        slowness[0..8].copy_from_slice(&p_data[48..56]);
        slowness[8..72].copy_from_slice(&s_data[0..64]);
        slowness[72..80].copy_from_slice(&p_data[56..64]);
    }

    let mut aes = cbc_encryptor(KeySize128, &p_data[0..32], &s_data[32..32 + 16], NoPadding);

    for _ in 0..7 {
        println!("encrypting...");
        let src = slowness.clone();
        aes.encrypt(
            &mut RefReadBuffer::new(&src[..]),
            &mut RefWriteBuffer::new(&mut slowness[..]),
            false,
        )
        .unwrap();
    }
    println!("encrypting... done");

    let mut iv = [0u8; 16];
    aes.encrypt(
        &mut RefReadBuffer::new(&slowness[0..16]),
        &mut RefWriteBuffer::new(&mut iv[0..16]),
        false,
    )
    .unwrap();

    return iv;
}

pub fn sha512(salt: &str) -> [u8; 64] {
    let mut sha = Sha512::new();
    sha.input(salt.as_bytes());
    let fixed = sha.fixed_result();
    assert_eq!(64, fixed.len());
    let mut result = [0u8; 64];
    result.copy_from_slice(&fixed);
    result
}

pub fn sha256(salt: &str) -> [u8; 32] {
    let mut sha = Sha256::default();
    sha.input(salt.as_bytes());
    sha.fixed_result().into()
}
