use base64ct::{Base64, Encoding};
use sha2::{Digest, Sha256};

pub struct Hash {}

impl Hash {
    fn to_sha256(message: String) -> String {
        let hashed = Sha256::digest(message.as_bytes());
        let encoded_hash = Base64::encode_string(&hashed);
        encoded_hash
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hashing() {
        let hash = Hash::to_sha256("".to_string());
        assert_eq!(hash, "47DEQpj8HBSa+/TImW+5JCeuQeRkm5NMpJWZG3hSuFU=");
        assert_eq!(hash.len(), 44);
    }
}
