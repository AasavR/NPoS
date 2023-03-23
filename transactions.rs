use evm::backend::MemoryVicinity;
use evm::{TransactionAction, Context, ExitReason};
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

struct Transaction {
    from: String,
    to: String,
    value: u64,
    data: Vec<u8>,
}

struct Mempool {
    queue: Arc<Mutex<VecDeque<Transaction>>>,
}

impl Mempool {
    fn new() -> Self {
        Self {
            queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    fn add_transaction(&self, tx: Transaction) {
        let mut queue = self.queue.lock().unwrap();
        queue.push_back(tx);
    }

    fn get_transactions(&self) -> Vec<Transaction> {
        let mut queue = self.queue.lock().unwrap();
        let mut txs = Vec::new();
        while let Some(tx) = queue.pop_front() {
            txs.push(tx);
        }
        txs
    }
}

#[get("/transactions")]
fn get_transactions(mempool: State<Mempool>) -> Json<Vec<Transaction>> {
    let txs = mempool.get_transactions();
    Json(txs)
}

#[post("/transactions", data = "<tx>")]
fn post_transaction(mempool: State<Mempool>, tx: Json<Transaction>) -> Result<Json<String>, String> {
    mempool.add_transaction(tx.into_inner());
    Ok(Json(String::from("Transaction added to mempool")))
}
