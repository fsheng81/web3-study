//用vec来存储，因为不需要中间插入区块。。。。

use bigint::U256;
use utils::bkey::BKey;
use utils::serialize::{serialize, hash_str, hash_u8};
use leveldb::database::Database;
use std::sync::Mutex;
use std::collections::HashMap;

use crate::block::Block; /* struct Block */
use crate::bcdb::BlockChainDb;
use crate::pow::ProofOfWork;
use crate::transaction::Transaction;

const CURR_BITS: u32 = 0x2100FFFF;
const PRE_HASH: &str = "22caaf24ef23refsdferfefdfet43fddfddfefefef";
const SAVE_DIR: &str = "bd_db";

pub struct BlockChain {
    // pub blocks: Vec<Block>, /* 不再是直接vec线性存储 */
    blocks_db: Box<Database<BKey>>,
    blocks_index: Mutex<HashMap<String, Block>>,
    pub gnes_hash: String,
    pub curr_hash: String,
    pub curr_bits: u32,
}

impl BlockChain {
    pub fn new() -> Self {
        let mut db = BlockChainDb::new(SAVE_DIR);
        let genesis = Self::genesis_block();
        BlockChain::write_block(&mut db, &genesis);
        BlockChain::write_tail(&mut db, &genesis);
        println!("new block saved in blockChain new\n");

        let gene_block = genesis.clone();
        let mut block_index = Mutex::new(HashMap::new());
        Self::update_hmap(&mut block_index, gene_block); /* 这是？ */

        let gnes_hash = genesis.hash.clone();
        let curr_hash = genesis.hash.clone();

        BlockChain {
            // blocks: vec![genesis],
            blocks_db: Box::new(db),
            blocks_index: block_index,
            gnes_hash: gnes_hash,
            curr_hash: curr_hash,
            curr_bits: CURR_BITS,
        }
    }

    fn genesis_block() -> Block {
        println!("start mining....");
        let from = "0x0000".to_string();
        let to = "0x0000".to_string();
        let sign = "创世区块".to_string();
        let tx = Transaction::new(from, to, 0, 0, 0, sign);
        // Block::new("传世区块".to_string(), PRE_HASH.to_string(), CURR_BITS)
        let mut block = Block::new(vec![tx], PRE_HASH.to_string(), CURR_BITS);

        let header_ser = ProofOfWork::prepare_data(&mut block, 0);
        block.hash = hash_str(&header_ser);
        println!("produce a new block genesis");

        block
    }

    /* 交易信息 链上数据 */
    pub fn add_block(&mut self, block: Block) {
        // let pre_block = &self.blocks[self.blocks.len() - 1];
        // let pre_hash = pre_block.hash.clone(); /* "引用的结构体" 的成员 */
        // let new_block = Block::new(txs, pre_hash, self.curr_bits);

        Self::write_block(&mut (self.blocks_db), &block);
        Self::write_tail(&mut (self.blocks_db), &block);

        println!("new produced block saved\n");
        self.curr_hash = block.hash.clone();
        self.curr_bits = block.header.bits.clone();
        Self::update_hmap(&mut self.blocks_index, block);
        // self.blocks.push(new_block);
    }

    pub fn block_info(&self) {
        let mut hash = self.curr_hash.clone();
        let hmap = self.blocks_index.lock().unwrap();
        let mut blocks: Vec<Block> = Vec::new();

        loop {
            if let Some(b) = hmap.get(&hash) {
                blocks.push(b.clone());
                hash = b.header.pre_hash.clone(); /* 不断寻找上一个区块 */
            } else {
                panic!("error getting block");
            }

            /* 直到了创世区块 */
            if hash == self.gnes_hash {
                if let Some(b) = hmap.get(&hash) {
                    blocks.push(b.clone());
                }
                break;
            }
        }
        blocks.reverse(); /* 逆序 */
        for b in blocks.iter() { // 不再是vec
            println!("{:#?}", b);
        }
    }

    fn write_block(db: &mut Database<BKey>, block: &Block) {
        // 基于header生成key
        let header_ser = serialize(&(block.header));
        let mut hash_u: [u8; 32] = [0; 32];
        hash_u8(&header_ser, &mut hash_u);

        let key = BKey{ val: U256::from(hash_u) };
        let val = serialize(&block);
        BlockChainDb::write_db(db, key, &val);
    }

    // 把区块hash作为尾巴写入
    fn write_tail(mut db: &mut Database<BKey>, block: &Block) {
        let key = BKey{ val: U256::from("tail".as_bytes()) };
        let val = serialize(&(block.hash));
        BlockChainDb::write_db(&mut db, key, &val);
    }

    fn update_hmap(hmap: &mut Mutex<HashMap<String, Block>>, block: Block) {
        let mut hmap = hmap.lock().unwrap();
        let hash = block.hash.clone(); /* hash 什么类型？存入hashmap是不是要复制一份？ */
        hmap.insert(hash, block);
    }
}