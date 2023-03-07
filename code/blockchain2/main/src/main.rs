// use core::blockchain::BlockChain as BC;
use core::account::Account;
use core::transaction::Transaction; // cargo - lib.rs - struct Name
use core::mine::Mine;

fn main() {
    println!("Hello, world!");

    let mut user1 = Account::new("0xabcd".to_string(), "Kim".to_string());
    let mut user2 = Account::new("0xabce".to_string(), "Tom".to_string());
    let mut user3 = Account::new("0xabcf".to_string(), "Jim".to_string());

    let mut mine = Mine::new();
    let mut txs: Vec<Transaction> = Vec::new();

    let res = user1.transfer_to(&mut user2, 9, 1);
    match res {
        Ok(tx) => txs.push(tx),
        Err(e) => panic!("{}", e),
    }
    let res = user1.transfer_to(&mut user2, 5, 1);
    match res {
        Ok(tx) => txs.push(tx),
        Err(e) => panic!("{}", e),
    }

    mine.mining(&mut txs);
    println!("miner info......");
    mine.miner.miner_info();
    println!("account info.......");
    let users = vec![&user1, &user2, &user3]; /* 先构造一个vec再说，而且是指针 */
    for user in users {
        user.account_info();
    }
    println!("block info......");
    mine.blockchain.block_info();
}
