mod account;
mod keypair;
mod signature;

use std::path::Path;

use account::Account;
use keypair::KeyPair;
use signature::Signature;

fn main() {
    //check if user has already generated keys and return
    //ro generate them if he hasn't
    // let path = Path::new("keys.txt");
    // if path.exists() && path.metadata().unwrap().len() > 0 {
    //     let retrieved_keys = KeyPair::read_from_file().unwrap();

    //     // KeyPair::print_key_pair(&retrieved_keys);

    //     // let Signature { signature } =
    //     //     Signature::sign_data(&retrieved_keys.private_key, "hello world".to_string()).unwrap();
    //     let signed = Signature::sign_data(&retrieved_keys.private_key, "hello world".to_string()).unwrap();
    //     // println!("the signature {:?}", signed);
    //     Signature::print_sig(&signed);
    //     let verified = Signature::verify_sig(signed, retrieved_keys.public_key, "hello world".to_string()).unwrap();
    //     println!("the verified {:?}", verified);

    //     return;
    // }

    // let key_pair = KeyPair::new_key_pair().unwrap();
    // KeyPair::print_key_pair(&key_pair);
    // // println!("the res {:?}", res);

    // KeyPair::save_key_to_file(&key_pair).unwrap();

    //PLAYING WITH THE ACCOUNT
    let path = Path::new("account.txt");
    if path.exists() && path.metadata().unwrap().len() > 0 {
        // println!("you already have an account!");
        println!("generating acc to try transfer fn");
        let mut account1 = Account::gen_account();
        let mut account2 = Account::gen_account();

        println!("\naccount 1-> \n {:#?}", account1);
        println!("\naccount 2-> \n {:#?}", account2);

        // account1.airdrop_balance(232);
        let sig = account1.sign_data("msg".to_string());
        println!("\nthis is the signature\n{:?}\n", sig);
        account1.create_payment_op(60, &mut account2);

        println!("account 1 after the transfer {:#?}", account1);
        println!("account 2 after the transfer {:#?}", account2);
        return;
    }
    // let account = Account::gen_account();
    // Account::add_key_pair_to_wallet(&account).unwrap();
    // println!("\nthis is the account generated\n{:#?}", account);

    
}

//the structs/classes
struct Block {}

struct Blockchain {}

struct Hash {}

struct Operation {}

struct Transaction {}
