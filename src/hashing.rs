// hasing & encryption related

use pbkdf2::pbkdf2_hmac;
use sha2::Sha512;
use sha2::Digest;
use aes::Aes256;

use crate::constants::*;
use crate::structures::Config;

pub fn init_encryption(config: &mut Config) {
    config.my_key = generate_random_key();
    config.key_generated = true;
    config.magic_number = get_24bit_hash(config.my_key.clone());
}

// generate_legal_iv
pub fn generate_secure_iv() {
    
}

// generate_legal_key
pub fn generate_secure_key(key: String) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    let initial_vector = INITINAL_VECTOR.to_string();
    pbkdf2_hmac::<Sha512>(key.as_bytes(), initial_vector.as_bytes(), 50_000, &mut bytes);
    bytes
}

pub fn get_24bit_hash(key: String) -> u32 {
    let mut bytes = [0u8; PACKET_SIZE];
    let byte_version = key.as_bytes();
    bytes[..byte_version.len()].copy_from_slice(&byte_version);
    let mut computed_hash = Sha512::digest(bytes);
    for _ in 0..50000 {
        computed_hash = Sha512::digest(computed_hash);
    }

    let mut bytes = [0u8; 4];
    bytes[0] = computed_hash[0];
    let mut result = u32::from_ne_bytes(bytes);
    result <<= 23;

    bytes = [0u8; 4];
    bytes[2] = computed_hash[1];
    result += u32::from_ne_bytes(bytes);

    bytes = [0u8; 4];
    bytes[1] = computed_hash.last().unwrap().clone();
    result += u32::from_ne_bytes(bytes);

    bytes = [0u8; 4];
    bytes[0] = computed_hash[2];
    result += u32::from_ne_bytes(bytes);
    result
}

// passwd for the specific device
pub fn generate_random_key() -> String {
    let mut key = String::new();
    while key.len() < PASSWD_LENGTH {
        let random: u8 = rand::random();
        let index = usize::from(random) % POSSIBLE_PASSWD_CHARS.len();
        let selected_char = POSSIBLE_PASSWD_CHARS.chars().nth(index).unwrap();
        key.push(selected_char);
    }
    key
}
