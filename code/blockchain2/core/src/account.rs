use crate::transaction::Transaction;
use utils::serialize::{serialize, hash_str};
use serde::Serialize;

/* 账户 */
#[derive(Serialize, Debug, Eq, PartialEq, Clone)]
pub struct Account {
    pub nonce: u64,
    pub balance: u64,
    pub name: String,
    pub address: String,
    pub hash: String,
}

impl Account {
    pub fn new(address: String, name: String) -> Self {
        let mut account = Account {
            nonce: 0,
            name: name,
            balance: 100,
            address: address,
            hash: "".to_string(),
        };
        account.set_hash();
        account
    }

    fn set_hash(&mut self) {
        let data = serialize(&self);
        self.hash = hash_str(&data);
    }

    /* 只是转移btc */
    pub fn transfer_to(&mut self, to: &mut Self,
        amount: u64, fee: u64) -> Result<Transaction, String>
    {
        if amount + fee > self.balance {
            return Err("Error: not enough amount".to_string());
        }

        self.balance -= amount;
        self.balance -= fee;
        self.nonce += 1;
        self.set_hash(); /* account账户的hash都会改变一次 */

        to.balance += amount;
        to.nonce += 1;
        self.set_hash();

        let sign = format!("{} -> {}: {} btc", self.address.clone(),
                            to.address.clone(),
                            amount);
        let tx = Transaction::new(self.address.clone(), to.address.clone(),
                            amount, fee, self.nonce, sign);
        Ok(tx)
    }

    pub fn account_info(&self) {
        println!("{:#?}", &self);
    }
}