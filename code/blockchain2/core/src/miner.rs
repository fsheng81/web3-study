use crate::block::Block;
use crate::pow::ProofOfWork; /* 用其他的POS实现呢？ */
use crate::transaction::Transaction;

const MINER_NAME: &str = "anonymous"; /* 匿名 */

// 矿工
#[derive(Debug, Clone)]
pub struct Miner {
    name: String,
    pub balance: u64,
    address: String,
}

impl Miner {
    pub fn new(address: String) -> Self {
        Miner {
            name: MINER_NAME.to_string(),
            balance: 100_u64,
            address: address,
        }
    }

    pub fn mine_block(&mut self, txs: &mut Vec<Transaction>, pre_hash: String, bits: u32) -> Block
    {
        let mut fee = 0; /* 挖矿手续费 */
        for tx in txs.iter() {
            fee += tx.fee.clone();
        }

        let from = "0x0000".to_string();
        let to = self.address.clone();
        let sign = format!("{} -> {}: 50 btc", from, to);
        let coinbase = Transaction::new(from, to, 0, 0, 0, sign); /* 矿工奖励随时间半衰 */

        /* 加入 coinbase 交易和普通交易 */
        let mut txs_2: Vec<Transaction> = Vec::new();
        txs_2.push(coinbase);
        txs_2.append(txs);
        let block = Self::mine_job(txs_2, pre_hash, bits);

        /* 此时是挖矿ok了 */
        self.balance += 50;
        self.balance += fee;

        block
    }

    fn mine_job(txs: Vec<Transaction>, pre_hash: String, bits: u32) -> Block {
        let mut block = Block::new(txs, pre_hash, bits);
        let pow = ProofOfWork::new(bits);
        pow.run(&mut block); /* 执行pow证明 */

        block
    }

    pub fn miner_info(&self) {
        println!("{:#?}", &self);
    }
}