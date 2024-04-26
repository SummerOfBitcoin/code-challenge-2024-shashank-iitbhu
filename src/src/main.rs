use std::fs;
use serde_json::Value;

fn main() {
    let mempool_dir = "../mempool";
    let mut transactions: Vec<Value> = Vec::new();

    if let Ok(entries) = fs::read_dir(mempool_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Ok(file_content) = fs::read_to_string(&path) {
                    if let Ok(tx) = serde_json::from_str::<Value>(&file_content) {
                        if is_valid_transaction(&tx) {
                            transactions.push(tx);
                        }
                    }
                }
            }
        }
    }

    println!("{:?}", transactions);
}

fn is_valid_transaction(_tx: &Value) -> bool {
    // Implement your transaction validation logic here
    true // Placeholder for example, always returns true
}
