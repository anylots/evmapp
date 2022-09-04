use crate::storage::spec::Database;
use ethereum_types::U256;
use std::collections::HashMap;

pub struct RamDB {
    pub db: HashMap<U256, U256>,
}

impl RamDB {
    pub fn new() -> Self {
        Self { db: HashMap::new() }
    }
}

impl Database for RamDB {
    fn get(&self, key: U256) -> U256 {
        self.db.get(&key).cloned().unwrap_or_default()
    }

    fn set(&mut self, key: U256, value: U256) {
        if value == U256::default() {
            self.db.remove(&key);
        } else {
            self.db.insert(key, value);
        }
    }
}
