use evm::backend::{Backend, Basic};
use evm::{Address, ExitReason, ExitSucceed, ExitError};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

// Define a struct that represents the L1X VM state.
pub struct L1XVM {
    db: Arc<RwLock<HashMap<Vec<u8>, Vec<u8>>>>,
    trie: Arc<RwLock<trie::Trie>>,
    backend: Basic,
}

impl L1XVM {
    // Create a new L1X VM instance.
    pub fn new() -> Self {
        L1XVM {
            db: Arc::new(RwLock::new(HashMap::new())),
            trie: Arc::new(RwLock::new(trie::Trie::new_empty())),
            backend: Basic::default(),
        }
    }

    // Get the value for a given key from the database.
    fn get_db(&self, key: &[u8]) -> Option<Vec<u8>> {
        let db = self.db.read().unwrap();
        db.get(key).cloned()
    }

    // Set the value for a given key in the database.
    fn set_db(&self, key: Vec<u8>, value: Vec<u8>) {
        let mut db = self.db.write().unwrap();
        db.insert(key, value);
    }

    // Get the value for a given key from the trie.
    fn get_trie(&self, key: &[u8]) -> Option<Vec<u8>> {
        let trie = self.trie.read().unwrap();
        trie.get(key)
    }

    // Set the value for a given key in the trie.
    fn set_trie(&self, key: Vec<u8>, value: Vec<u8>) {
        let mut trie = self.trie.write().unwrap();
        trie.insert(key, value);
    }
}

// Implement the `Backend` trait for the L1X VM.
impl Backend for L1XVM {
    fn gas_price(&self) -> U256 {
        self.backend.gas_price()
    }

    fn origin(&self) -> Address {
        self.backend.origin()
    }

    fn block_hash(&self, number: U256) -> H256 {
        self.backend.block_hash(number)
    }

    fn block_number(&self) -> U256 {
        self.backend.block_number()
    }

    fn block_coinbase(&self) -> Address {
        self.backend.block_coinbase()
    }

    fn block_timestamp(&self) -> U256 {
        self.backend.block_timestamp()
    }

    fn block_difficulty(&self) -> U256 {
        self.backend.block_difficulty()
   
