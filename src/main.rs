mod keypair;
mod signature;

use std::path::Path;

use keypair::KeyPair;
use signature::Signature;

fn main() {
    //check if user has already generated keys and return
    //ro generate them if he hasn't
    let path = Path::new("keys.txt");
    if path.exists() && path.metadata().unwrap().len() > 0 {
        let retrieved_keys = KeyPair::read_from_file().unwrap();

        // KeyPair::print_key_pair(&retrieved_keys);

        // let Signature { signature } =
        //     Signature::sign_data(&retrieved_keys.private_key, "hello world".to_string()).unwrap();
        let signed = Signature::sign_data(&retrieved_keys.private_key, "hello world".to_string()).unwrap();
        // println!("the signature {:?}", signed);
        Signature::print_sig(&signed);
        let verified = Signature::verify_sig(signed, retrieved_keys.public_key, "hello world".to_string()).unwrap();
        println!("the verified {:?}", verified);

        return;
    }

    let key_pair = KeyPair::new_key_pair().unwrap();
    KeyPair::print_key_pair(&key_pair);
    // println!("the res {:?}", res);

    KeyPair::save_key_to_file(&key_pair).unwrap();
}

//the structs/classes
struct Account {
    //
}

struct Block {}

struct Blockchain {}

struct Hash {}

struct Operation {}

struct Transaction {}

impl Account {
    pub fn new() -> String {
        "I am new to this".to_string()
    }
}
