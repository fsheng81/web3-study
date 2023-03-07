// 工作量证明
/* 需要一个超大的int */
/* 每打包一个区块就是构造一个工作量任务 ProofOfWork */
 
use std::thread;
use std::time::Duration;
use bigint::U256;
use utils::serialize::{serialize, hash_str, hash_u8};
use crate::block::Block;

const MAX_NONCE: u32 = 0x7FFFFFFF;

pub struct ProofOfWork {
    target: U256,
}

impl ProofOfWork {
    pub fn new(bits: u32) -> Self {
        let (mant, expt) = {
            let unshifted_expt = bits >> 24;
            if unshifted_expt <= 3 {
                ((bits & 0xFFFFFF) >> (8 * (3 - unshifted_expt as usize)), 0)
            } else {
                (bits & 0xFFFFFF, 8 * ((bits >> 24) - 3))
            }
        };

        if mant > 0x7FFFFF {
            Self {
                target: Default::default(),
            }
        } else {
            Self {
                target: U256::from(mant as u64) << (expt as usize),
            }
        }
    }

    /* 挖矿 */
    pub fn run(&self, mut block: &mut Block) {
        println!("start mining...");
        thread::sleep(Duration::from_secs(3));

        let mut nonce: u32 = 0;
        while nonce <= MAX_NONCE {
            // 计算值
            let hd_ser = Self::prepare_data(&mut block, nonce);
            let mut hash_u: [u8; 32] = [0; 32];
            hash_u8(&hd_ser, &mut hash_u);

            // 判断是否满足要求
            let hash_int = U256::from(hash_u);
            if hash_int <= self.target {
                block.hash = hash_str(&hd_ser);
                println!("produce a new block\n");
                return;
            }

            nonce += 1;
        }
    }

    pub fn prepare_data(block: &mut Block, nonce: u32) -> Vec<u8> {
        block.header.nonce = nonce;
        serialize(&(block.header))
    }
}