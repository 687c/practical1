use anyhow::{anyhow, bail, Error, Ok /* Result */};
use base64ct::{Base64, Encoding};
use core::panic;
use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};
// use secp256k1::PublicKey;
use sha2::{Digest, Sha256};
use thiserror::Error;

use crate::{keypair::KeyPair, signature::Signature};

#[derive(Debug)]
pub struct Account {
    pub account_id: String,
    pub wallet: KeyPair,
    pub balance: u64,
}

impl Account {
    pub fn gen_account() -> Self {
        //generate the keys
        let key_pair = KeyPair::new_key_pair().unwrap();
        let pub_key_hash = Sha256::digest(key_pair.public_key.to_string().as_bytes());

        //LET MY ADDRESSES START WITH 88
        let encoded_hash = Base64::encode_string(&pub_key_hash);
        let account_id = format!("88{encoded_hash}");

        let balance = 0;
        let wallet = key_pair;

        Self {
            account_id,
            balance,
            wallet,
        }
    }

    pub fn add_key_pair_to_wallet(
        account: &Self, /* path: Option<PathBuf >*/
    ) -> std::result::Result<(), Error> {
        /* TODO
         * what if user wants to have multiple accounts - then wallet should be a Vector of type KeyPair
         * or a custom Wallet struct
         */
        //if user already has an account ask whether he wants to create a new one
        //or continue with the old one

        let private_key = account.wallet.private_key.secret_bytes();
        let formatted_private = format!("private key -> {:?}\n", private_key);
        let balance_hash = Sha256::digest(account.balance.to_be_bytes());
        let formatted_balance = format!("balance -> {:?}", balance_hash);
        // println!("balance hash {:?}", balance_hash);

        //Writing the PK and balance to the file
        let path = Path::new("account.txt");
        // if path.exists() && path.metadata().unwrap().len() > 0 {
        //     println!("you already have an account");
        //     return Ok(());
        // }
        let mut file = File::create(path).unwrap();
        file.write(formatted_private.as_bytes()).unwrap();
        file.write(formatted_balance.as_bytes()).unwrap();

        Ok(())
    }

    pub fn airdrop_balance(&mut self, input: u64) {
        self.balance = input;
        println!("\nYou created {:#?} money from nowhere\n", self);
    }

    #[allow(dead_code)]
    pub fn update_balance(/* &mut self, input: u64 */) {
        // self.balance = input;
        // println!("\nYou created {:#?} money from nowhere\n", self);
    }

    pub fn create_payment_op(
        &mut self,
        transfer_amt: u64,
        transfer_to_account: &mut Self,
    ) -> Result<(), Error> {
        //if there is a discrepancy when updating in address do not update the balance
        if self.balance == 0 {
            // return Err(anyhow!("there is not enough"));
            panic!(
                "Insufficient funds to make the transfer.Current balance -> {}",
                self.balance
            )
        }

        //add to receiver amt transferred
        //check if acc address is valid
        if !transfer_to_account.account_id.starts_with("88") {
            // panic!("Invalid account. Accounts must start with 88")
            bail!(AccountErrors::InvalidAccount);
        }

        //deduct from myself amt transferred
        self.balance -= transfer_amt;
        transfer_to_account.balance = transfer_amt;

        println!(
            "\ntransferred {} from acc {} to acc {}\n",
            transfer_amt, self.account_id, transfer_to_account.account_id
        );

        Ok(())
    }

    pub fn get_balance(&self) -> u64 {
        self.balance
    }

    pub fn print_balance(&self) {
        println!("user's balance is -> {}", self.balance);
    }

    pub fn sign_data(&self, msg: String /* index: u64 */) -> Signature {
        let pk = self.wallet.private_key;
        let sig = Signature::sign_data(&pk, msg).unwrap();
        sig
    }
}

#[derive(Error, Debug)]
enum AccountErrors {
    #[error("Not enough funds to create payment")]
    InsufficientFunds,
    #[error("Accounts on this blockchain start with 88")]
    InvalidAccount,
}

#[cfg(test)]
mod tests {
    use super::Account;

    #[test]
    fn account_init() {
        let account = Account::gen_account();

        assert!(account.account_id.starts_with("88"));

        assert_eq!(account.account_id.len(), 41);
        

        println!("the gen acc {:#?}", account.account_id.len());
    }
}
