use evm::{Address, Transaction};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionRequest {
    pub from: Address,
    pub to: Address,
    pub gas: u64,
    pub gas_price: u64,
    pub value: u64,
    pub data: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionResponse {
    pub hash: String,
}

pub struct API {
    pub trie: HashMap<Vec<u8>, Vec<u8>>,
    pub transactions: Vec<Transaction>,
    pub blocks: Vec<Vec<Transaction>>,
}

impl API {
    pub fn new() -> Self {
        API {
            trie: HashMap::new(),
            transactions: vec![],
            blocks: vec![],
        }
    }

    pub fn submit_transaction(&mut self, transaction: TransactionRequest) -> TransactionResponse {
        let tx = Transaction {
            nonce: 0,
            gas_price: transaction.gas_price.into(),
            gas_limit: transaction.gas.into(),
            action: if transaction.to == Address::zero() {
                evm::Create
            } else {
                evm::Call(transaction.to)
            },
            value: transaction.value.into(),
            input: transaction.data.clone(),
            signature: None,
        };

        self.transactions.push(tx.clone());

        TransactionResponse {
            hash: hex::encode(tx.hash()),
        }
    }

    pub fn trie_get(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.trie.get(key).cloned()
    }

    pub fn code_size(&self, address: Address) -> u64 {
        if let Some(data) = self.trie.get(&address.as_bytes().to_vec()) {
            data.len() as u64
        } else {
            0
        }
    }

    pub fn validators(&self) -> Vec<(Address, u64)> {
        vec![]
    }

    pub fn block(&self, block_number: u64) -> Option<Vec<Transaction>> {
        self.blocks.get(block_number as usize).cloned()
    }

    pub fn call(&mut self, from: Address, to: Address, data: Vec<u8>) -> Vec<u8> {
        let mut machine = evm::Evm::new(evm::Config::istanbul());
        let context = evm::Context {
            address: to,
            caller: from,
            apparent_value: evm::U256::zero(),
        };
        machine.create_context(context);
        let tx = Transaction {
            nonce: 0,
            gas_price: evm::U256::zero(),
            gas_limit: evm::U256::from(100000),
            action: evm::Call(to),
            value: evm::U256::zero(),
            input: data,
            signature: None,
        };
        machine.apply_transaction(&tx);
        machine.return_value().to_vec()
    }
}
