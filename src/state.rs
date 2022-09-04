use crate::storage::spec::Database;
use ethereum_types::U256;
use std::collections::HashMap;

pub struct State<DB> {
    db: DB,
    cache: HashMap<U256, U256>,
}

impl<DB: Database> State<DB> {
    pub fn new(db: DB) -> Self {
        Self {
            db: db,
            cache: HashMap::new(),
        }
    }

    pub fn store(&mut self, key: U256, value: U256) {
        self.cache.insert(key, value);
    }

    pub fn load(&self, key: U256) -> U256 {
        match self.cache.get(&key) {
            Some(value) => value.into(),
            None=>self.db.get(key)
        }
    }
}
