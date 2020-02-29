use core::slice;

use aes::block_cipher_trait::generic_array::{ArrayLength, GenericArray};
use aes::{Aes128, Aes256};
use block_modes::block_padding::ZeroPadding;
use block_modes::{BlockMode, Cbc};
use sha2::digest::FixedOutput;
use sha2::{Digest, Sha256, Sha512};

const SLOWNESS_LEN: usize = 80_000_000;
const BLOCK_SIZE: usize = 80;

pub type Aes128Cbc = Cbc<Aes128, ZeroPadding>;
pub type Aes256Cbc = Cbc<Aes256, ZeroPadding>;

pub const AES128CBC_BLOCK_BYTE_LENGTH: usize = 128 / 8;

pub fn hash_password(salt: &str, password: &str) -> [u8; 16] {
    let s_data = sha512(&salt.to_lowercase());
    let p_data = sha512(password);

    println!("user: {:x?}", &s_data[..]);
    println!("pass: {:x?}", &p_data[..]);

    let mut slowness = vec![0u8; SLOWNESS_LEN];

    for pos in 0..SLOWNESS_LEN / BLOCK_SIZE {
        let index = pos * BLOCK_SIZE;
        let slowness = &mut slowness[index..index + BLOCK_SIZE];

        slowness[0..8].copy_from_slice(&p_data[48..56]);
        slowness[8..72].copy_from_slice(&s_data[0..64]);
        slowness[72..80].copy_from_slice(&p_data[56..64]);
    }
    println!("slow: {:x?}", &slowness[..80]);

    let mut aes = Aes256Cbc::new_var(&p_data[0..32], &s_data[32..32 + 16]).unwrap();

    for i in 0..7 {
        println!("encrypting... {}", i);
        aes.encrypt_blocks(to_blocks(&mut slowness[..]));
    }
    println!("encrypting... done");

    let mut blocks = [GenericArray::clone_from_slice(&slowness[..16])];
    aes.encrypt_blocks(&mut blocks);
    blocks[0].into()
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

pub(crate) fn to_blocks<N>(data: &mut [u8]) -> &mut [GenericArray<u8, N>]
where
    N: ArrayLength<u8>,
{
    let n = N::to_usize();
    debug_assert!(data.len() % n == 0);
    unsafe { slice::from_raw_parts_mut(data.as_ptr() as *mut GenericArray<u8, N>, data.len() / n) }
}
