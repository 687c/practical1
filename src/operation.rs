use crate::{account::Account, signature::Signature};
use anyhow::{Ok, Result};

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct Operation {
    sender: Account,
    receiver: Account,
    amount: u128,
    pub signature: Signature,
    comment: String,
}

impl Operation {
    pub fn create_operation(
        sender: Account,
        receiver: Account,
        amount: u128,
        signature: Signature,
        comment: String,
    ) -> Self {
        Self {
            sender,
            receiver,
            amount,
            signature,
            comment,
        }
    }

    pub fn verify_operation(self /*, operation: Operation */) -> Result<bool> {
        //check the funds ||
        if self.sender.balance < self.amount {
            panic!("Insufficient funds")
        }

        //check the sig
        // let sig = operation.signature;
        // let public_key = operation.sender.wallet[0].public_key;
        // let msg = operation.comment;

        let sig = self.signature;
        let public_key = self.sender.wallet[0].public_key;
        let msg = self.comment;
        if !Signature::verify_sig(sig, public_key, msg).unwrap() {
            panic!("signature verification failed")
        }

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_funds_transfer() {
        let mut acc1 = Account::gen_account();
        let acc2 = Account::gen_account();

        acc1.airdrop_coins(230);

        let comment = "sending 200 to acc2";
        let sig = acc1.sign_data(comment.to_string(), 0);

        let op = Operation::create_operation(acc1, acc2, 200, sig, comment.to_string());
        let verification = op.verify_operation().unwrap();
        // let verification = Operation::verify_operation(op).unwrap();
        assert!(verification);
    }

    #[test]
    #[should_panic]
    fn test_insufficient_funds() {
        let acc1 = Account::gen_account();
        let acc2 = Account::gen_account();

        let comment = "sending 200 to acc2";
        let sig = acc1.sign_data(comment.to_string(), 0);

        let op = Operation::create_operation(acc1, acc2, 220, sig, comment.to_string());
        // Operation::verify_operation(op).unwrap();
        op.verify_operation().unwrap();
    }

    #[test]
    #[should_panic]
    fn test_signature_fail() {
        let mut acc1 = Account::gen_account();
        let acc2 = Account::gen_account();

        acc1.airdrop_coins(30);

        let comment = "sending 200 to acc2";
        let sig = acc1.sign_data(comment.to_string(), 0);

        let fake_sig = acc2.sign_data(comment.to_string(), 0);
        let op = Operation::create_operation(acc1, acc2, 0, fake_sig, comment.to_string());
        op.verify_operation().unwrap();
    }
}
