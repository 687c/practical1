use anyhow::{Error, Ok /* Result */};
use base64ct::{Base64, Encoding};
use sha2::{Digest, Sha256};

use crate::{KeyPair, Signature};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Account {
    pub account_id: String,
    pub wallet: Vec<KeyPair>,
    pub balance: u128,
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
        let wallet = vec![key_pair];

        Self {
            account_id,
            balance,
            wallet,
        }
    }

    pub fn add_key_pair_to_wallet(&mut self, key_pair: KeyPair) {
        self.wallet.push(key_pair);
    }

    // pub fn save_acc_to_file(
    //     account: &Self, /* path: Option<PathBuf >*/
    // ) -> std::result::Result<(), Error> {
    //     /* TODO
    //      * what if user wants to have multiple accounts - then wallet should be a Vector of type KeyPair
    //      * or a custom Wallet struct
    //      */
    //     //if user already has an account ask whether he wants to create a new one
    //     //or continue with the old one

    //     let private_key = account.wallet.private_key.secret_bytes();
    //     let formatted_private = format!("private key -> {:?}\n", private_key);
    //     let balance = account.balance.to_be_bytes();

    //     //Writing the PK and balance to the file
    //     let path = Path::new("account.txt");
    //     // if path.exists() && path.metadata().unwrap().len() > 0 {
    //     //     println!("you already have an account");
    //     //     return Ok(());
    //     // }
    //     let mut file = File::create(path).unwrap();
    //     file.write(formatted_private.as_bytes()).unwrap();
    //     file.write(&balance).unwrap();

    //     Ok(())
    // }

    pub fn airdrop_coins(&mut self, input: u128) {
        self.balance = input;
        println!(
            "\n{} coins airdropped to account {}\n",
            input, self.account_id
        );
    }

    #[allow(dead_code)]
    pub fn update_balance(&mut self, input: u128) {
        self.balance = input;
    }

    pub fn create_payment_op(
        &mut self,
        transfer_amt: u128,
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
            panic!("Invalid account. Accounts must start with 88")
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

    pub fn get_balance(&self) -> u128 {
        self.balance
    }

    pub fn print_balance(&self) {
        println!("user's balance is -> {}", self.balance);
    }

    pub fn sign_data(&self, msg: String, index: usize) -> Signature {
        let pk = self.wallet[index].private_key;
        let sig = Signature::sign_data(&pk, msg).unwrap();
        sig
    }
}

#[cfg(test)]
mod tests {
    use super::Account;
    use crate::KeyPair;

    #[test]
    fn account_init() {
        let account = Account::gen_account();

        assert!(account.account_id.starts_with("88"));

        //acc_id length
        assert_eq!(account.account_id.len(), 46);
        assert_eq!(account.balance, 0);
    }

    #[test]
    fn add_keys_to_wallet() {
        let mut acc = Account::gen_account();

        let new_key_pair = KeyPair::new_key_pair().unwrap();
        acc.add_key_pair_to_wallet(new_key_pair);

        assert_eq!(acc.wallet.len(), 2);
    }

    #[test]
    fn test_airdrop_coins() {
        let mut acc1 = Account::gen_account();

        acc1.airdrop_coins(230);
        assert_eq!(acc1.get_balance(), 230);
        assert_eq!(acc1.balance, 230);
    }

    #[test]
    fn test_transfer_funds() {
        let mut acc1 = Account::gen_account();
        let mut acc2 = Account::gen_account();

        acc1.airdrop_coins(230);
        assert_eq!(acc1.balance, 230);

        acc1.create_payment_op(30, &mut acc2).unwrap();
        assert_eq!(acc1.balance, 200);
        assert_eq!(acc2.get_balance(), 30);
    }

    #[test]
    #[should_panic]
    fn test_transfer_insufficient_funds() {
        let mut acc = Account::gen_account();
        let mut acc2 = Account::gen_account();

        //should panic and test should pass
        acc2.create_payment_op(23423, &mut acc).unwrap();
    }

    #[test]
    #[ignore = "Test when generated keys are saved permanently"]
    fn test_sign_data() {
        let acc = Account::gen_account();

        let res = acc.sign_data("hello world!".to_string(), 0);
        println!("\n{:#?}", res);
    }
}
