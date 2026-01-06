use pbkdf2::pbkdf2_hmac;
use sha2::{
    Sha512,
    Digest
};
use aes::{
    Aes256, Aes256Dec, Aes256Enc, Block
};
use aes::cipher::{
    BlockDecryptMut,
    BlockEncryptMut,
    KeyIvInit,
    block_padding::ZeroPadding,
    generic_array::GenericArray
};

type Aes256Encoder = cbc::Encryptor<aes::Aes256>;
type Aes256Decoder = cbc::Decryptor<aes::Aes256>;

pub const INITIAL_VECTOR: [u8; 16] = [49, 56, 52, 52, 54, 55, 52, 52, 48, 55, 51, 55, 48, 57, 53, 53]; // first 16 characters of u64::MAX
pub const INITIAL_VECTOR_U: [u8; 40] = [49, 0, 56, 0, 52, 0, 52, 0, 54, 0, 55, 0, 52, 0, 52, 0, 48, 0, 55, 0, 51, 0, 55, 0, 48, 0, 57, 0, 53, 0, 53, 0, 49, 0, 54, 0, 49, 0, 53, 0]; // first 16 characters of u64::MAX, interperted as unicode
// key is just password with length of 16 char.
pub const POSSIBLE_KEY_CHARS: &str =
"abcdefghjkmnpqrstuvxyz\
ABCDEFGHJKMNPQRSTUVXYZ\
123456789\
~!@#$%^*()_-+=:;<,>.?/\\|[]";

#[derive(Debug)]
pub struct Hasher {
    key: String,
    legal_key: [u8; 32],
    magic_number: u32,
    encryptor: cbc::Encryptor<aes::Aes256>,
    decryptor: cbc::Decryptor<aes::Aes256>
}

impl Hasher {
    pub fn new() -> Hasher {
        let key = generate_random_key();
        let byte_key: [u8; 16] = key.as_bytes().try_into().expect("invalid key length");
        let legal_key = generate_legal_key(&byte_key);
        let magic_number = generate_24bit_hash(&byte_key);

        Hasher {
            key: key,
            legal_key: legal_key,
            magic_number: magic_number,
            encryptor: Aes256Encoder::new(legal_key.as_slice().into(), INITIAL_VECTOR.as_slice().into()),
            decryptor: Aes256Decoder::new(legal_key.as_slice().into(), INITIAL_VECTOR.as_slice().into()),
        }
    }

    pub fn from(key: String) -> Hasher {
        let key = key;
        let byte_key: [u8; 16] = key.as_bytes().try_into().expect("invalid key length");
        let legal_key = generate_legal_key(&byte_key);
        let magic_number = generate_24bit_hash(&byte_key);

        Hasher {
            key: key,
            legal_key: legal_key,
            magic_number: magic_number,
            encryptor: Aes256Encoder::new(legal_key.as_slice().into(), INITIAL_VECTOR.as_slice().into()),
            decryptor: Aes256Decoder::new(legal_key.as_slice().into(), INITIAL_VECTOR.as_slice().into()),
        }
    }

    // verify checksum and retrieve embbed magic data
    pub fn verify_and_update_package(&self, bytes: &mut [u8; 32]) {
        let mut magic_bytes = [0u8; 4];
        magic_bytes[3] = bytes[3];
        magic_bytes[2] = bytes[2];
        let acquired_magic_number = u32::from_ne_bytes(magic_bytes);
    
        if acquired_magic_number != (self.magic_number & 0xFFFF0000) {
            bytes[0] = 0xFF; // setting package type as invalid
        }
        let mut checksum = 0;
        for byte in &bytes[2..] {
            let mut calculated_byte = [0u8; 4];
            calculated_byte[0] = byte.clone();
            checksum += i32::from_ne_bytes(calculated_byte);
        }

        if checksum.to_ne_bytes()[0] != bytes[1] {
            bytes[0] = 0xFF; // setting package type as invalid
        }
        // received valid package so erasing checksum and magic number
        bytes[3] = 0;
        bytes[2] = 0;
        bytes[1] = 0;
    }

    // calculating and embedding the checksum and magic number, in
    // 1 and 2, 3 respectively.
    pub fn sign_package(&self, bytes: &mut [u8; 32]) {
        let magic_number = self.magic_number;
        bytes[3] = (magic_number >> 24) as u8;
        bytes[2] = (magic_number >> 16) as u8;

        let mut checksum_byte = 0;
        for byte in &bytes[2..] {
            let mut calculated_byte = [0u8; 4];
            calculated_byte[0] = byte.clone();
            checksum_byte += i32::from_ne_bytes(calculated_byte);
        }
        bytes[1] = checksum_byte.to_ne_bytes()[0];
    }

    pub fn encrypt_data_16(&mut self, target: &mut [u8; 16]) {
       self.encryptor
            .encrypt_block_mut(target.into());
    }

    pub fn encrypt_data_32(&mut self, target: &mut [u8; 32]) {
        let block_size = <Aes256Enc as aes::cipher::BlockSizeUser>::block_size();

        let mut blocks: Vec<Block> = target
            .chunks_exact(block_size)
            .map(|chunk| GenericArray::clone_from_slice(chunk))
            .collect();

        self.encryptor
            .encrypt_blocks_mut(&mut blocks);

        for (dest, src) in target.chunks_exact_mut(block_size).zip(blocks) {
            dest.copy_from_slice(&src);
        }
    }

    pub fn decrypt_data_16(&mut self, encrypted_bytes: &mut [u8; 16]) {
        self.decryptor
            .decrypt_block_mut(encrypted_bytes.into());
    }

    pub fn decrypt_data_32(&mut self, encrypted_bytes: &mut [u8; 32]) {
        let block_size = <Aes256Dec as aes::cipher::BlockSizeUser>::block_size();

        let mut blocks: Vec<Block> = encrypted_bytes
            .chunks_exact(block_size)
            .map(|chunk| GenericArray::clone_from_slice(chunk))
            .collect();

        self.decryptor
            .decrypt_blocks_mut(&mut blocks);

        for (dest, src) in encrypted_bytes.chunks_exact_mut(block_size).zip(blocks) {
            dest.copy_from_slice(&src);
        }
    }
}

// strengthening the generated random key
fn generate_legal_key(key: &[u8]) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    pbkdf2_hmac::<Sha512>(key, &INITIAL_VECTOR_U, 50_000, &mut bytes);
    bytes
}

// magic number generator
fn generate_24bit_hash(key: &[u8]) -> u32 {
    let mut bytes = [0; 32];
    bytes[..key.len()].copy_from_slice(key);
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
fn generate_random_key() -> String {
    let mut key = String::new();
    while key.len() < 16 {
        let random: u8 = rand::random();
        let index = usize::from(random) % 32;
        let selected_char = POSSIBLE_KEY_CHARS.chars().nth(index).expect("out of index: while generating random passwd");
        key.push(selected_char);
    }
    key
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_and_update_package() {
        let hasher = Hasher::from(String::from("asdfgfasdfgfasdf"));
        let mut data: [u8; 32] = [124, 16, 186, 14, 167, 112, 106, 80, 25, 40, 180, 140, 60, 142, 28, 25, 124, 152, 132, 14, 167, 112, 106, 80, 25, 40, 180, 140, 60, 142, 28, 25];
        let mut expected_data: [u8; 32] = [124, 0, 0, 0, 167, 112, 106, 80, 25, 40, 180, 140, 60, 142, 28, 25, 124, 152, 132, 14, 167, 112, 106, 80, 25, 40, 180, 140, 60, 142, 28, 25];
        hasher.verify_and_update_package(&mut data);
        assert_eq!(data, expected_data);
    }

    #[test]
    fn test_sign_package() {
        let hasher = Hasher::from(String::from("asdfgfasdfgfasdf"));
        let mut data: [u8; 32] = [124, 152, 132, 14, 167, 112, 106, 80, 25, 40, 180, 140, 60, 142, 28, 25, 124, 152, 132, 14, 167, 112, 106, 80, 25, 40, 180, 140, 60, 142, 28, 25];
        let mut expected_data: [u8; 32] = [124, 16, 186, 14, 167, 112, 106, 80, 25, 40, 180, 140, 60, 142, 28, 25, 124, 152, 132, 14, 167, 112, 106, 80, 25, 40, 180, 140, 60, 142, 28, 25];
        hasher.sign_package(&mut data);
        assert_eq!(data, expected_data);
    }

    #[test]
    fn test_generate_legal_key() {
        let key = String::from("asdfgfasdfgfasdf");
        let legal_key = generate_legal_key(key.as_bytes());
        let expected_key = [9, 42, 160, 24, 211, 243, 168, 159, 155, 13, 203, 83, 14, 208, 215, 9, 14, 182, 229, 164, 241, 30, 74, 187, 94, 30, 219, 178, 137, 79, 24, 151];
        assert_eq!(legal_key, expected_key);

        let key = String::from("zxcvbv/.,mnmz/x.");
        let legal_key = generate_legal_key(key.as_bytes());
        let expected_key = [180, 150, 134, 69, 23, 179, 205, 48, 105, 115, 31, 228, 72, 191, 109, 128, 47, 4, 161, 147, 75, 140, 38, 182, 190, 23, 104, 104, 38, 250, 228, 34];
        assert_eq!(legal_key, expected_key);
    }

    #[test]
    fn test_generate_24bit_hash() {
        let key = String::from("asdfgfasdfgfasdf");
        let magic_number = generate_24bit_hash(key.as_bytes());
        let expected_magic_number = 247085678;
        assert_eq!(magic_number, expected_magic_number);

        let key = String::from("zxcvbv/.,mnmz/x.");
        let magic_number = generate_24bit_hash(key.as_bytes());
        let expected_magic_number = 1573776049;
        assert_eq!(magic_number, expected_magic_number);
    }

    #[test]
    fn test_generate_random_key() {
        let random_key = generate_random_key();
        assert_eq!(random_key.len(), 16);
        assert_eq!(random_key.is_ascii(), true);
    }
}
