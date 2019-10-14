use std::borrow::BorrowMut;
use std::fmt::Debug;
use std::clone::Clone;
use std::fmt::Write;

use serde::{Serialize, Deserialize, de::DeserializeOwned};

use crate::blockchain::transaction::{Transaction, Transactional};
use crate::crypto::merkle;

/// A header of a block in the blockchain
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockHeader {
    /// The creation timestamp of the block.
    timestamp: i64,

    /// The nonce of the block.
    ///
    /// It is used to obtain a hash for a certain input that fulfills certain arbitrary condition.
    pub nonce: u32,

    /// The hash of the previous block.
    pre_hash: String,

    /// The merkle tree of a block.
    ///
    /// A Merkle tree summarizes all the transactions in a block by producing a digital fingerprint
    /// of the entire set of transactions, thereby enabling a user to verify whether or not a transaction is included in a bloc
    merkle: String,

    /// The difficulty to mine a new block.
    ///
    /// The difficulty is a number that regulates how long it takes for miners to add new blocks of
    /// transactions to the blockchain.
    pub difficulty: u32,
}

impl PartialEq for BlockHeader {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp.eq(&other.timestamp) && self.pre_hash.eq(&other.pre_hash)
            && self.merkle.eq(&other.merkle)
    }
}

impl BlockHeader 
where Self: Send
{
    /// Used to format the header of a block.
    pub fn fmt(&self) -> String {
        let mut str = String::new();

        write!(&mut str, "        BlockHeader: [\n").expect("[BlockHeader fmt()]: Unable to write in Buffer!");
        write!(&mut str, "            Timestamp:     {}\n", self.timestamp).expect("[BlockHeader fmt()]: Unable to write in Buffer!");
        write!(&mut str, "            Nonce:         {}\n", self.nonce).expect("[BlockHeader fmt()]: Unable to write in Buffer!");
        write!(&mut str, "            Previous Hash: {}\n", self.pre_hash).expect("[BlockHeader fmt()]: Unable to write in Buffer!");
        write!(&mut str, "            Merkle:        {}\n", self.merkle).expect("[BlockHeader fmt()]: Unable to write in Buffer!");
        write!(&mut str, "            Difficulty:    {}\n", self.difficulty).expect("[BlockHeader fmt()]: Unable to write in Buffer!");
        write!(&mut str, "        ]\n").expect("[BlockHeader fmt()]: Unable to write in Buffer!");

        str
    }
}

impl Eq for BlockHeader {}

/// A block of the blockchain
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block<T> {
    /// The header informations in the current block.
    pub header: BlockHeader,
    /// The number of transactions in the current block.
    count: u32,
    /// The transactions in the current block.
    transactions: Vec<Transaction<T>>,
}

impl<T> PartialEq for Block<T> {
    fn eq(&self, other: &Self) -> bool {
        self.header.eq(&other.header)
    }
}

impl<T> Eq for Block<T> {}

impl<T> Block<T>
where T: Serialize + DeserializeOwned + Debug + Clone + Transactional + Send
{
    pub fn new(
        hash: String,
        difficulty: u32,
        miner_address: String,
        reward: u32,
        transactions: &mut Vec<Transaction<T>>
                                        ) -> Self {
        let header = BlockHeader {
            timestamp: time::now().to_timespec().sec,
            nonce: 0,
            pre_hash: hash,
            merkle: String::new(),
            difficulty
        };

        let reward_trans = T::genesis(miner_address, reward);

        let mut block = Block {
            header,
            count: 0,
            transactions: vec![]
        };

        block.transactions.push(reward_trans);
        block.transactions.append(transactions.borrow_mut());
        block.count = block.transactions.len() as u32;
        block.header.merkle = merkle::get_merkle(block.transactions.clone());
        block
    }

    /// Used to format a block of the blockchain.
    pub fn fmt(&self) -> String {
        let mut str = String::new();

        write!(&mut str, "    Block: [\n").expect("[Block fmt()]: Unable to write in Buffer!");
        write!(&mut str, "{}", self.header.fmt()).expect("[Block fmt()]: Unable to write in Buffer!");
        write!(&mut str, "        Number of Transactions: {}\n", self.count).expect("[Block fmt()]: Unable to write in Buffer!");
        write!(&mut str, "        Transactions: [\n").expect("[Block fmt()]: Unable to write in Buffer!");

        for transaction in &self.transactions {
            write!(&mut str, "{:?}", transaction.fmt()).expect("[Block fmt()]: Unable to write in Buffer!");
        }

        write!(&mut str, "        ]\n").expect("[Block fmt()]: Unable to write in Buffer!");
        write!(&mut str, "    ]\n").expect("[Block fmt()]: Unable to write in Buffer!");

        str
    }
}

#[cfg(test)]
mod tests {
    use crate::blockchain::block::{BlockHeader, Block};
    use crate::blockchain::transaction::{CryptoPayload, Transactional};

    #[test]
    fn block_header_eq() {
        let block_header_1 = BlockHeader {
            timestamp: 0,
            pre_hash: String::from("00xxxxxxxxxxxxxxxxxx"),
            nonce: 24,
            merkle: String::from("xxxxxxxxxxxxxxxxxxxx"),
            difficulty: 2,
        };

        let block_header_2 = BlockHeader {
            timestamp: 1,
            pre_hash: String::from("00yyyyyyyyyyyyyyyyyy"),
            nonce: 42,
            merkle: String::from("yyyyyyyyyyyyyyyyyyyy"),
            difficulty: 2,
        };

        assert_eq!(block_header_1.eq(&block_header_2), false);
        assert_eq!(block_header_1.eq(&block_header_1), true);
    }

    #[test]
    fn block_eq() {
        let block_header_1 = BlockHeader {
            timestamp: 0,
            pre_hash: String::from("00xxxxxxxxxxxxxxxxxx"),
            nonce: 24,
            merkle: String::from("xxxxxxxxxxxxxxxxxxxx"),
            difficulty: 2,
        };

        let block_header_2 = BlockHeader {
            timestamp: 1,
            pre_hash: String::from("00yyyyyyyyyyyyyyyyyy"),
            nonce: 42,
            merkle: String::from("yyyyyyyyyyyyyyyyyyyy"),
            difficulty: 2,
        };

        let block_1: Block<CryptoPayload> = Block {
            header: block_header_1,
            count: 0,
            transactions: vec![],
        };

        let block_2: Block<CryptoPayload> = Block {
            header: block_header_2,
            count: 0,
            transactions: vec![],
        };

        assert_eq!(block_1.eq(&block_2), false);
        assert_eq!(block_1.eq(&block_1), true);
    }

    #[test]
    fn new_block() {
        let hash = String::from("00xxxxxxxxxxxxxxxxxx");
        let difficulty = 2;
        let miner_addr = String::from("Schwurbel");
        let reward = 42;

        let crypto_payload = CryptoPayload {
            receiver: String::from("Peter"),
            amount: 42,
        };
        let mut transaction = vec![CryptoPayload::new(miner_addr.clone(), crypto_payload)];

        let block: Block<CryptoPayload> = Block::new(hash, difficulty, miner_addr,
                                                     reward, &mut transaction);

        assert_eq!(block.count, 2);
        assert_eq!(block.transactions.len(), 2);
    }
}
