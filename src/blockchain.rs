use anyhow::Result;
use std::collections::HashMap;

use crate::{Account, Block, Operation, Transaction};

#[derive(Debug)]
struct Blockchain {
    //SHOULD coin_db hashmap value be the balance
    coin_database: HashMap<Account, u128>,
    block_history: Vec<Block>,
    tx_database: Vec<Transaction>,
    faucet_coins: u128,
}

impl Blockchain {
    pub fn init_blockchain() -> Result<Self> {
        //block && tx
        let master_account = Account::gen_account();
        let comment = "initialize Blockchain";
        let sig = master_account.clone().sign_data(comment.to_string(), 0);
        let op = Operation::create_operation(
            master_account.clone(),
            master_account.clone(),
            0,
            sig,
            comment.to_string(),
        );
        let set_of_ops = vec![op];
        let tx = Transaction::create_tx(set_of_ops);
        let tx_database = vec![tx];
        let set_of_tx = tx_database.clone();
        let genesis_hash = "00000000000000000000";
        let genesis_block = Block::create_block(set_of_tx, genesis_hash.to_string()).unwrap();
        let mut block_history: Vec<Block> = Vec::new();
        block_history.push(genesis_block);

        //faucet
        let faucet_coins: u128 = 1000000000;

        //coin db
        let mut coin_database: HashMap<Account, u128> = HashMap::new();
        coin_database.insert(master_account.clone(), faucet_coins);

        Ok(Self {
            block_history,
            coin_database,
            faucet_coins,
            tx_database,
        })
    }

    pub fn get_faucet_tokens(&mut self, acc: &mut Account, amt: u128) {
        //update faucet balance
        self.faucet_coins -= amt;
        let bal = self.faucet_coins;

        //get the first key and update the master acc bal
        let master_acc_key = self.coin_database.keys().next().unwrap().to_owned();
        self.coin_database.insert(master_acc_key, bal);

        //update acc bal that requested coins
        acc.update_balance(amt);
        //insert acc that borrowed coins into DB
        self.coin_database.insert(acc.to_owned(), amt);
    }

    //adds blocks to the blockchain
    pub fn validate_block(&mut self, block: &mut Block) {
        //check for prev hash
        if block.prev_hash.is_empty() {
            panic!("previous hash not found");
        }

        if block.prev_hash == "00000000000000000000" {
            panic!("Previous hash cannot be genesis hash");
        }

        //check that tx in the block haven't been added to history
        let mut found: bool = false;
        for blocks in &self.block_history {
            for txs in &blocks.set_of_tx {
                found = block.set_of_tx.contains(txs);
            }
        }
        if found {
            panic!("transaction already exists");
        }

        //check that block does not contain conflicting tx
        let mut found: bool = false;
        for txs in &self.tx_database {
            found = block.set_of_tx.contains(txs);
        }
        if found {
            panic!("Conflicting TXs");
        }

        //check if the ops in the tx have(USE FOR LOOP TO CHECK ALL OP)
        //1. sig verification
        //2. funds verification(done in operations)
        for txs in block.set_of_tx.clone() {
            for op in txs.set_of_operations.clone() {
                op.verify_operation().unwrap();
            }
        }

        //since everything checks out, push the block and txs into history
        self.block_history.push(block.clone());
        self.tx_database.append(&mut block.set_of_tx);
    }

    pub fn show_coin_db(&self) -> Result<()> {
        println!("\n{:#?}\n", self.coin_database);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //testing the blockchain init
    #[test]
    fn test_genesis_hash() {
        let init = Blockchain::init_blockchain().unwrap();

        assert_eq!(init.block_history[0].prev_hash, "00000000000000000000");
    }

    #[test]
    fn init_block() {
        let init = Blockchain::init_blockchain().unwrap();
        //EXPECT
        //at least one tx
        assert!(init.tx_database.len() >= 1);
        // at least one block
        assert!(init.block_history.len() >= 1);
        //balance of faucet to be 1b
        assert_eq!(init.faucet_coins, 1000000000);
    }

    //test borrowing from faucet
    #[test]
    fn coin_from_faucet() {
        let mut init = Blockchain::init_blockchain().unwrap();
        let mut acc2 = Account::gen_account();

        init.get_faucet_tokens(&mut acc2, 10000);
        // acc2.print_balance();

        //were coins dispensed to acc2
        assert_eq!(acc2.balance, 10000);

        //were coins deducted from faucet
        assert_eq!(init.faucet_coins, 999990000);

        // init.show_coin_db();
    }

    //testing block validation
    fn gen_tx() -> Transaction {
        let acc1 = Account::gen_account();
        let comment = "initialize Blockchain";
        let sig = acc1.clone().sign_data(comment.to_string(), 0);
        let op =
            Operation::create_operation(acc1.clone(), acc1.clone(), 0, sig, comment.to_string());
        let set_of_ops = vec![op];
        let tx = Transaction::create_tx(set_of_ops);

        tx
    }

    fn generate_block(prev_hash: &str) -> Block {
        let tx = gen_tx();
        let tx_database = vec![tx];
        let set_of_tx = tx_database.clone();
        let prev_hash = prev_hash;
        let genesis_block = Block::create_block(set_of_tx, prev_hash.to_string()).unwrap();

        genesis_block
    }

    #[test]
    #[should_panic]
    fn test_absent_prev_hash() {
        //CREATE A BLOCK NOT IN THE HISTORY
        let mut init = Blockchain::init_blockchain().unwrap();
        let mut empty_genesis = generate_block("");
        init.validate_block(&mut empty_genesis);
        // Blockchain::validate_block(genesis_block);
    }

    #[test]
    #[should_panic]
    fn test_fake_genesis() {
        let mut init = Blockchain::init_blockchain().unwrap();
        let mut fake_genesis = generate_block("00000000000000000000");

        init.validate_block(&mut fake_genesis);
        // println!()
    }

    //test that block isn't in the blockchain using latest_hash
    // #[test]
    // fn block_not_added_to_history_yet() {
    //     // let latest_block = generate_block("00000000000000000000");
    //     let mut init = Blockchain::init_blockchain().unwrap();

    //     // let tx = gen_tx();
    //     let hash = "19075484419459801441";
    //     let block = generate_block(hash);

    //     init.validate_block(block); //will pass because tx hasn't been added to blocks
    //                                 //we can confirm that by

    //     assert_eq!(init.tx_database.len(), 1); //CHANGE HERE IF PROBLEM
    // }

    #[test]
    fn test_prev_hash_add() {
        let mut init = Blockchain::init_blockchain().unwrap();
        // let hash = "19075484419459801441";
        let p_hash = &init.block_history[0].block_id;
        let mut block = generate_block(p_hash);
        init.validate_block(&mut block);

        let genesis_block_id = &init.block_history[0].block_id;
        let block1_prev_hash = &init.block_history[1].prev_hash;
        assert_eq!(genesis_block_id, block1_prev_hash);

        // let prev_hash = &init.block_history.iter().next().next().unwrap().prev_hash;
        // let prev_hash = &init.block_history[1].prev_hash;
        // let mut block2 = generate_block(prev_hash);
        // init.validate_block(&mut block2);

        for block in init.block_history {
            println!("prev_hash -> {}", block.prev_hash);
            println!("block_id -> {}", block.block_id);
            println!();
        }
    }

    #[test]
    fn test_block_addition() {
        let mut init = Blockchain::init_blockchain().unwrap();

        let prev_hash = &init.block_history.iter().next().unwrap().prev_hash;
        println!("{:?}", prev_hash);

        let hash = "19075484419459801441";
        let mut block = generate_block(hash);

        init.validate_block(&mut block); //will pass because tx hasn't been added to blocks
                                         //we can confirm that by

        assert_eq!(init.block_history.len(), 2); //CHANGE HERE IF PROBLEM
    }

    #[test]
    fn dummy_test() {
        let init = Blockchain::init_blockchain().unwrap();

        // println!("inited {:#?}", init.coin_database);

        init.show_coin_db();
    }
}
