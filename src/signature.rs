use anyhow::{Ok, Result};
// use base64ct::{Base64, Encoding};
use secp256k1::{ecdsa::Signature as SignatureType, Message, PublicKey, Secp256k1, SecretKey};
// use sha2::{Digest, Sha256};

use secp256k1::hashes::sha256;

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct Signature {
    pub signature: SignatureType,
}

impl Signature {
    pub fn sign_data(private_key: &SecretKey, message: String) -> Result<Self> {
        // let hash = Sha256::digest(message.as_bytes());
        // println!("the digest hash {:?}", hash);

        // let encoded = Base64::encode_string(&hash);
        let secp = Secp256k1::new();
        let message = Message::from_hashed_data::<sha256::Hash>(message.as_bytes());
        let signature = secp.sign_ecdsa(&message, &private_key);

        Ok(Self { signature })
    }

    pub fn verify_sig(sig: Signature, public_key: PublicKey, message: String) -> Result<bool> {
        let secp = Secp256k1::new();
        let msg = Message::from_hashed_data::<sha256::Hash>(message.as_bytes());
        let sig = &sig.signature;

        let verified_sig = secp.verify_ecdsa(&msg, sig, &public_key);

        if verified_sig.is_ok() {
            Ok(true)
        } else {
            panic!("Invalid Signature")
        }
    }

    pub fn print_sig(sig: &Self) {
        println!("signature -> {:?}", sig.signature);
    }
}
