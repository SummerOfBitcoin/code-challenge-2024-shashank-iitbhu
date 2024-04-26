use std::{fs, io};
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Define the input transaction data structure
#[derive(Debug, Serialize, Deserialize)]
struct Transaction {
    version: u32,
    locktime: u32,
    vin: Vec<Value>,
    vout: Vec<Value>,
}

fn is_valid_transaction(transaction: &Transaction) -> bool {
    transaction.version != 0 && !transaction.vin.is_empty() && !transaction.vout.is_empty()
    // Add more validation logic as needed
}

fn main() -> io::Result<()> {
    let mempool_dir = "../mempool";
    let mut transactions = vec![];
    for entry in fs::read_dir(mempool_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let file = fs::File::open(path)?;
            let tx: Transaction = serde_json::from_reader(file)?;
            if is_valid_transaction(&tx) {
                transactions.push(tx);
            }
        }
    }
    Ok(())
}

