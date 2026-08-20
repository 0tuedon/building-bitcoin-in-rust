use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    U256,
    crypto::{PublicKey, Signature},
    sha256::Hash,
    util::MerkleRoot,
};
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BlockHeader {
    pub timestamp: DateTime<Utc>,
    pub nonce: u64,
    pub prev_block_hash: Hash,
    pub merkle_root: MerkleRoot,
    pub target: U256,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Transaction {
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<TransactionOutput>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TransactionInput {
    pub prev_transaction_output_hash: Hash,
    pub signature: Signature,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TransactionOutput {
    pub value: u64,
    pub unique_id: Uuid,
    pub pubkey: PublicKey,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain { blocks: vec![] }
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }
}

impl Block {
    pub fn new(header: BlockHeader, transactions: Vec<Transaction>) -> Self {
        Block {
            header: header,
            transactions: transactions,
        }
    }
    pub fn hash(self) {
        unimplemented!()
    }
}

impl BlockHeader {
    pub fn new(
        timestamp: DateTime<Utc>,
        nonce: u64,
        prev_block_hash: Hash,
        merkle_root: MerkleRoot,
        target: U256,
    ) -> Self {
        BlockHeader {
            timestamp,
            nonce,
            prev_block_hash,
            merkle_root,
            target,
        }
    }
    pub fn hash() -> ! {
        todo!()
    }
}
impl Transaction {
    pub fn new(inputs: Vec<TransactionInput>, outputs: Vec<TransactionOutput>) -> Self {
        Transaction { inputs, outputs }
    }

    pub fn hash(&self) -> ! {
        todo!()
    }
}

impl TransactionOutput {
    fn hash(&self) -> Hash {
        Hash::hash(&self)
    }
}
