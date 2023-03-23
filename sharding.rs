struct Shard {
    transactions: Vec<Transaction>,
}

struct ShardManager {
    shards: Vec<Shard>,
}

impl ShardManager {
    fn new(num_shards: usize) -> Self {
        let mut shards = Vec::with_capacity(num_shards);
        for _ in 0..num_shards {
            shards.push(Shard { transactions: vec![] });
        }
        Self { shards }
    }

    fn process_transaction(&mut self, transaction: Transaction) -> Result<(), String> {
        let shard_index = transaction.hash() % self.shards.len() as u64;
        let shard = &mut self.shards[shard_index as usize];
        shard.transactions.push(transaction);
        Ok(())
    }

    fn get_transactions(&mut self) -> Vec<Transaction> {
        let mut transactions = vec![];
        for shard in &mut self.shards {
            transactions.append(&mut shard.transactions);
        }
        transactions
    }
}
