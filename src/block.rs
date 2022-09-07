use anyhow::{Ok, Result};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::tx::{self, Transaction};

#[derive(Debug, Hash, PartialEq)]
struct Block {
    block_id: String,
    prev_hash: String,
    set_of_tx: Vec<Transaction>,
}

impl Block {
    pub fn create_block(set_of_tx: Vec<Transaction>, prev_hash: String) -> Result<Self> {
        let tx_hash = Block::create_hash(&set_of_tx) as u128;
        let hash_prev_hash = Block::create_hash(&prev_hash) as u128;
        let block_id = (tx_hash + hash_prev_hash).to_string();

        let set_of_tx = set_of_tx;
        let prev_hash = prev_hash;

        Ok(Self {
            block_id,
            set_of_tx,
            prev_hash,
        })
    }

    fn create_hash<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::format;
    use std::vec;

    use super::*;
    use crate::operation::Operation;
    use crate::Account;

    #[test]
    fn test_block_creation() {
        let mut acc1 = Account::gen_account();
        let acc2 = Account::gen_account();
        acc1.airdrop_coins(30);
        let comment = "sending 200 to acc2";
        let sig = acc1.sign_data(comment.to_string(), 0);
        let op = Operation::create_operation(acc1, acc2, 10, sig, comment.to_string());
        let set_of_ops = vec![op];
        let tx = Transaction::create_tx(set_of_ops);

        let set_of_tx = vec![tx];
        let genesis_hash = Block::create_hash(&"00000000000000000000");

        //test if block id is calculated properly
        //we can guess what the id of the block will be from the gen hash
        let hash_of_txs: u128 = Block::create_hash(&set_of_tx) as u128;
        let hash_of_prev_hash: u128 = Block::create_hash(&genesis_hash.to_string()) as u128;
        let expected_block_id = (hash_of_prev_hash + hash_of_txs).to_string();

        let created_block = Block::create_block(set_of_tx, genesis_hash.to_string());

        assert_eq!(created_block.unwrap().block_id, expected_block_id);
    }
}
