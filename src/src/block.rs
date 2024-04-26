use std::time::SystemTime;

pub struct Block{
    timestamp: u128,
    transactions: String,
    prev_block_hash: String,
    hash: String,
    height: usize,
    nonce: i32,
}

pub struct Blockchain{
    blocks: Vec<Block>
}

impl Block{
    pub fn new_block(data:String,prev_block_hash:String,height:usize)->Result<Block>{
        let timestamp=SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_millis();
        let mut block= Block{
            timestamp:timestamp,
            transactions:data,
            prev_block_hash,
            hash:String::new(),
            height,
            nonce:0,
        };
        block.run_proof_of_work()?;
        Ok(block)
    }

    fn run_proof_of_work(&mut self)-> Result<()>{
        info!("Mining the block!");
        while !self.validate()?{
            self.nonce+=1;
        }
        let data=self.prepare_hash_data()?;
        let mut hasher=Sha256::new();
        hasher.input(&data[..]);
        self.hash=hasher.result_str();
        Ok(())
    }

    fn prepare_hash_data(&self)->Result<Vec<u8>>{
        let content=(
            self.prev_block_hash.clone(),
            self.transaction.clone(),
            self.timestamp,
            TARGET_HEXT,
            self.nonce
        );
        let bytes=bincode::serialize(&content)?;
        Ok(bytes)
    }

    fn validate(&self) -> Result<bool> {
        let data = self.prepare_hash_data()?;
        let mut hasher = Sha256::new();
        hasher.input(&data[..]);
        let hash_result = hasher.result_str();
        let hash_bytes = hash_result.as_bytes();
    
        let mut target_bytes = vec![0; TARGET_HEXT.len() / 2];
        hex::decode_to_slice(TARGET_HEXT, &mut target_bytes)?;
    
        Ok(&hash_bytes[..target_bytes.len()] == &target_bytes[..])
    }
    
}