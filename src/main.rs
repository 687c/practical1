mod keypair;

use keypair::KeyPair;

fn main() {
    let key_pair = KeyPair::new_key_pair();
    
    KeyPair::print_key_pair(key_pair);
    // println!("the res {:?}", res);
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

struct Signature {}

impl Account {
    pub fn new() -> String {
        "I am new to this".to_string()
    }
}
