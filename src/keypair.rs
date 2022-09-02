use secp256k1::{rand::rngs::OsRng, PublicKey, SecretKey};

#[derive(Debug)]
pub struct KeyPair {
    pub public_key: PublicKey,
    pub private_key: SecretKey, //private property
}

impl KeyPair {
    pub fn new_key_pair() -> Self {
        let secp = secp256k1::Secp256k1::new();
        let mut csprng = OsRng; //cryptographically secure pseudorandom number generator
        let (secret_key, public_key) = secp.generate_keypair(&mut csprng);

        let public_key = public_key;
        let private_key = secret_key.to_owned();

        Self {
            public_key,
            private_key,
        }
    }

    pub fn print_key_pair(key_pair: Self) {
        let private_key = key_pair.private_key.display_secret().to_string();
        let public_key = key_pair.public_key.to_owned().to_string();

        println!("public key  -> {public_key}");
        println!("private Key -> {private_key}");
    }
}
