use std::{fs, fs::File, io::{self, Write}};
use serde::{Deserialize, Serialize};
// use serde_json::Value;

// Define the input transaction data structure
#[derive(Debug, Serialize, Deserialize)]
struct Transaction {
    version: u32,
    locktime: u32,
    vin: Vec<Vin>,
    vout: Vec<Vout>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Vin {
    txid: String,
    vout: u32,
    prevout: PrevOut,
    scriptsig: String,
    scriptsig_asm: Option<String>,
    witness: Option<Vec<String>>,
    is_coinbase: Option<bool>,
    sequence: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PrevOut {
    scriptpubkey: String,
    scriptpubkey_asm: Option<String>,
    scriptpubkey_type: Option<String>,
    scriptpubkey_address: Option<String>,
    value: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Vout {
    scriptpubkey: String,
    scriptpubkey_asm: Option<String>,
    scriptpubkey_type: Option<String>,
    scriptpubkey_address: Option<String>,
    value: u64,
}

fn main() -> io::Result<()> {
    let mempool_dir = "mempool";
    let mut transactions = vec![];
    for entry in fs::read_dir(mempool_dir)? {
        let entry = entry?;
        let path = entry.path();
        // println!("{:?}",path);
        if path.is_file() {
            let file = fs::File::open(path)?;
            let tx: Transaction = serde_json::from_reader(file)?;
            let txid = tx.vin[0].txid.clone(); // Assuming there's only one input for simplicity
            transactions.push(txid);
        }
    }

    // Write to output.txt
    let mut output_file = File::create("output.txt")?;
    // Write block header (placeholder)
    writeln!(output_file, "Block Header")?;
    // Write serialized coinbase transaction (placeholder)
    writeln!(output_file, "Serialized Coinbase Transaction")?;
    // Write transaction IDs of mined transactions
    for txid in transactions {
        writeln!(output_file, "{}", txid)?;
    }

    Ok(())
}