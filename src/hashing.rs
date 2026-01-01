// hasing & encryption related
use pbkdf2::pbkdf2_hmac;
use sha2::{
    Sha512,
    Digest
};
use aes::{
    Aes256,
    Block
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
// key is just password with length of 16 char.
pub const POSSIBLE_KEY_CHARS: &str =
"abcdefghjkmnpqrstuvxyz\
ABCDEFGHJKMNPQRSTUVXYZ\
123456789\
~!@#$%^*()_-+=:;<,>.?/\\|[]";

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
        let magic_number = generate_24bit_hash(&legal_key);

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
        let magic_number = generate_24bit_hash(&legal_key);

        Hasher {
            key: key,
            legal_key: legal_key,
            magic_number: magic_number,
            encryptor: Aes256Encoder::new(legal_key.as_slice().into(), INITIAL_VECTOR.as_slice().into()),
            decryptor: Aes256Decoder::new(legal_key.as_slice().into(), INITIAL_VECTOR.as_slice().into()),
        }
    }

    pub fn verify_and_update_package(&self, bytes: &mut [u8; 32]) {
        let mut magic_bytes = [0u8; 4];
        magic_bytes[3] = bytes[3];
        magic_bytes[2] = bytes[2];
        let acquired_magic_number = u32::from_ne_bytes(magic_bytes);
    
        // if acquired_magic_number != (self.magic_number & 0xFFFF0000) {
        if acquired_magic_number != self.magic_number {
            dbg!(acquired_magic_number);
            dbg!(self.magic_number);
            dbg!("invalid magic number");
            bytes[0] = 0xFF; // setting the package type as invalid
        }

        let checksum = bytes[2..].iter().copied().fold(0u8, u8::wrapping_add);

        if checksum != bytes[1] {
            dbg!(checksum);
            dbg!(bytes[1]);
            dbg!("invalid checksum");
            bytes[0] = 0xFF; // setting the package type as invalid
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
        // bytes[3] = ((magic_number >> 24) & 0xFF) as u8;
        // bytes[2] = ((magic_number >> 16) & 0xFF) as u8;
        bytes[1] = bytes[2..].iter().copied().fold(0u8, u8::wrapping_add);
    }

    pub fn encrypt_data(&self, target_bytes: [u8; 64]) -> [u8; 64] {
        let mut buffer = [0u8; 64];
        self.encryptor.clone()
            .encrypt_padded_b2b_mut::<ZeroPadding>(&target_bytes, &mut buffer)
            .unwrap();
        buffer
    }

    pub fn decrypt_data(&self, encrypted_bytes: [u8; 64]) -> [u8; 64] {
        let mut buffer = [0u8; 64];
        self.decryptor.clone()
            .decrypt_padded_b2b_mut::<ZeroPadding>(&encrypted_bytes, &mut buffer)
            .unwrap();
        buffer
    }
}

// strengthening the generated random key
fn generate_legal_key(key: &[u8]) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    pbkdf2_hmac::<Sha512>(key, &INITIAL_VECTOR, 50_000, &mut bytes);
    bytes
}

// magic number generator
fn generate_24bit_hash(key: &[u8; 32]) -> u32 {
    let mut computed_hash = Sha512::digest(key);
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
