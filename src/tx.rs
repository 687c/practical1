use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::operation::Operation;
use rand::distributions::{Alphanumeric, DistString};

#[derive(Debug, PartialEq)]
pub struct Transaction {
    tx_id: String,
    set_of_operations: Vec<Operation>,
    nonce: String,
}

impl Transaction {
    pub fn create_tx(set_of_ops: Vec<Operation>) -> Self {
        let hash = Transaction::calculate_hash(&set_of_ops);
        let nonce = Transaction::gen_nonce();
        let tx_id = format!("{}{}", nonce, hash);

        let set_of_operations = set_of_ops;

        Self {
            tx_id,
            set_of_operations,
            nonce,
        }
    }

    fn calculate_hash<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }

    fn gen_nonce() -> String {
        let nonce = Alphanumeric.sample_string(&mut rand::thread_rng(), 20);
        nonce
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::account::Account;

    #[test]
    fn test_hash_fn() {
        let mut acc1 = Account::gen_account();
        let acc2 = Account::gen_account();

        acc1.airdrop_coins(30);

        let comment = "sending 200 to acc2";
        let sig = acc1.sign_data(comment.to_string(), 0);

        let op = Operation::create_operation(acc1, acc2, 0, sig, comment.to_string());

        let hash = Transaction::calculate_hash(&op);
        let hash2 = Transaction::calculate_hash(&op);

        assert_eq!(hash, hash2);
    }

    #[test]
    fn test_nonde_gen() {
        // let string = Alphanumeric.sample_string(&mut rand::thread_rng(), 20);
        // println!("\nhere it is {}\n", string);
        let nonce = Transaction::gen_nonce();
        let nonce2 = Transaction::gen_nonce();

        //test nonce length
        assert_eq!(nonce.len(), 20);
        //generated nonces are not equal
        assert!(nonce != nonce2);
    }

    #[test]
    fn test_tx() {
        //create first tx
        let mut acc1 = Account::gen_account();
        let acc2 = Account::gen_account();
        acc1.airdrop_coins(30);
        let comment = "sending 200 to acc2";
        let sig = acc1.sign_data(comment.to_string(), 0);
        let op = Operation::create_operation(acc1, acc2, 10, sig, comment.to_string());
        let set_of_ops = vec![op];
        let tx1 = Transaction::create_tx(set_of_ops);

        //create second tx
        let mut acc1 = Account::gen_account();
        let acc2 = Account::gen_account();
        acc1.airdrop_coins(30);
        let comment = "sending 200 to acc2";
        let sig = acc1.sign_data(comment.to_string(), 0);
        let op = Operation::create_operation(acc1, acc2, 10, sig, comment.to_string());
        let set_of_ops = vec![op];
        let tx2 = Transaction::create_tx(set_of_ops);

        assert!(tx1 != tx2);
    }
}
