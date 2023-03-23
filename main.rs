use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::collections::HashMap;

mod api;

#[get("/trie/{key}")]
async fn trie_get(
    state: web::Data<api::API>,
    web::Path(key): web::Path<String>,
) -> impl Responder {
    if let Some(value) = state.trie_get(&hex::decode(key).unwrap()) {
        HttpResponse::Ok().body(hex::encode(value))
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[get("/validators")]

// Endpoint for retrieving the current list of validators and their stakes
async fn get_validators(&self) -> Vec<(evm::Address, U256)> {
    let mut validators = vec![];
    for validator in self.validators.iter() {
        let stake = self.stakes.get(&validator).unwrap_or(&U256::zero()).clone();
        validators.push((validator.clone(), stake));
    }
    validators
}

// Endpoint for accessing historical transactions and blocks
async fn get_block(&self, block_number: U256) -> Option<Block> {
    let block_hash = self.block_hashes.get(&block_number)?;
    self.blocks.get(&block_hash).cloned()
}

// Endpoint for interacting with smart contracts deployed on the L1X VM
async fn call_contract(
    &mut self,
    from: evm::Address,
    to: evm::Address,
    value: U256,
    data: Vec<u8>,
) -> Result<(Vec<u8>, bool), Box<dyn Error>> {
    let block = self.current_block();
    let gas_limit = block.gas_limit;
    let gas_price = self.gas_price();
    let context = evm::Context {
        tx_gas_price: gas_price,
        block_timestamp: block.timestamp,
        block_number: block.number,
        block_coinbase: block.coinbase,
        block_difficulty: block.difficulty,
        chain_id: self.chain_id,
    };
    let mut transaction = evm::Transaction {
        nonce: self.nonces.get(&from).unwrap_or(&U256::zero()).clone(),
        gas_price,
        gas_limit,
        action: if to == evm::Address::default() {
            evm::TransactionAction

