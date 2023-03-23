use std::collections::HashMap;
use evm::backend::{Backend, Apply};
use evm::{ExitSucceed, ExitError};

struct TrieBackend {
    trie: HashMap<Vec<u8>, Vec<u8>>,
}

impl Backend for TrieBackend {
    fn gas_price(&self) -> u64 {
        0
    }

    fn origin(&self) -> evm::Address {
        Default::default()
    }

    fn block_number(&self) -> u64 {
        0
    }

    fn block_timestamp(&self) -> u64 {
        0
    }

    fn block_difficulty(&self) -> U256 {
        Default::default()
    }

    fn block_gas_limit(&self) -> U256 {
        Default::default()
    }

    fn chain_id(&self) -> U256 {
        Default::default()
    }

    fn block_hashes(&self, _block_number: U256, _max: usize) -> Vec<H256> {
        vec![]
    }

    fn exists(&self, address: evm::Address) -> bool {
        self.trie.contains_key(&address.as_bytes().to_vec())
    }

    fn basic(&self, address: evm::Address) -> evm::backend::Basic {
        if let Some(data) = self.trie.get(&address.as_bytes().to_vec()) {
            evm::backend::Basic {
                balance: U256::from_big_endian(&data[..32]),
                nonce: U256::from_big_endian(&data[32..64]),
            }
        } else {
            evm::backend::Basic {
                balance: U256::zero(),
                nonce: U256::zero(),
            }
        }
    }

    fn code_size(&self, address: evm::Address) -> U256 {
        if let Some(data) = self.trie.get(&address.as_bytes().to_vec()) {
            U256::from(data.len())
        } else {
