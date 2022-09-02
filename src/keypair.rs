use anyhow::{Ok, Result};
use secp256k1::{rand::rngs::OsRng, Error, PublicKey, Secp256k1, SecretKey};
use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

#[derive(Debug)]
pub struct KeyPair {
    pub public_key: PublicKey,
    pub private_key: SecretKey, //private property
}

impl KeyPair {
    pub fn new_key_pair() -> Result<Self> {
        let secp = secp256k1::Secp256k1::new();
        let mut csprng = OsRng; //cryptographically secure pseudorandom number generator
        let (secret_key, public_key) = secp.generate_keypair(&mut csprng);

        let public_key = public_key;
        let private_key = secret_key.to_owned();

        Ok(Self {
            public_key,
            private_key,
        })
    }

    pub fn print_key_pair(key_pair: &Self) {
        let private_key = key_pair.private_key.display_secret().to_string();
        let public_key = key_pair.public_key.to_owned().to_string();

        println!("public key  -> {public_key}");
        println!("private Key -> {private_key}");
    }

    pub fn save_key_to_file(key_pair: &Self /* path: Path */) -> Result<()> {
        let private_key = key_pair.private_key.secret_bytes();
        let formatted_private = format!("private key -> {:?}\n", private_key);

        //Writing the private key to the file
        let path = Path::new("keys.txt");
        /* TODO
         * GIVE USER ABILITY TO SPECIFY WHERE HE WANTS HIS KEYS TO BE SAVED
         */
        let mut file = File::create(path).unwrap();
        file.write(formatted_private.as_bytes()).unwrap();

        Ok(())
    }

    pub fn read_from_file(/* path: Path */) -> Result<Self> {
        let path = Path::new("keys.txt");
        let mut file = File::open(path).unwrap();

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let lines = contents.lines(); // let line1 = lines.next().unwrap();

        let mut private_key = String::new();

        for line in lines {
            //if field private key not found panic and stop 
            if !line.contains("private key") {
                panic!("private key not found");
            }

            let mut private_line = line.split("->");
            private_key = private_line.nth(1).unwrap().trim().to_string();
        }

        private_key.pop();
        private_key.remove(0);

        let data: Vec<u8> = private_key
            .split(",")
            .map(|val| val.trim().parse().unwrap())
            .collect();

        let private_key = SecretKey::from_slice(&data).unwrap();

        let secp = Secp256k1::new();
        let public_key = PublicKey::from_secret_key(&secp, &private_key);

        Ok(Self {
            private_key,
            public_key,
        })
    }
}
