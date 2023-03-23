use crate::validator::Validator;

pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
}

pub struct Block {
    pub transactions: Vec<Transaction>,
    pub signature: String,
    pub proposer: Validator,
}
