use std::thread;
use std::time::Duration;
use chrono::prelude::*;
use utils::serialize::{serialize, hash_str};
use serde::Serialize;
use crate::pow::ProofOfWork;
use crate::transaction::Transaction;

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub struct BlockHeader {
    pub time: i64,
    pub pre_hash: String,
    pub txs_hash: String,
    pub nonce: u32,
    pub bits: u32
}

#[derive(Serialize, Debug, Clone)]
pub struct Block {
    pub header: BlockHeader,
    pub tranxs: Vec<Transaction>,
    pub hash: String,
}

impl Block {
    pub fn new(txs: Vec<Transaction>, pre_hash: String, bits: u32) -> Self {
        // 首先延迟3s，模拟挖矿
        
        let time = Utc::now().timestamp();
        let txs_ser = serialize(&txs);
        let txs_hash = hash_str(&txs_ser);
        let mut block = Block {
            header: BlockHeader {
                time: time,
                txs_hash: txs_hash,
                pre_hash: pre_hash,
                bits: bits,
                nonce: 0,
            },
            tranxs: txs,
            hash: "".to_string(),
        };
        // block.set_hash();
        // println!("produce a new block\n");

        // 开始挖矿
        let pow = ProofOfWork::new(bits);
        pow.run(&mut block);

        block
    }

    fn set_hash(&mut self) {
        let header = serialize(&(self.header));
        self.hash = hash_str(&header);
    }
}